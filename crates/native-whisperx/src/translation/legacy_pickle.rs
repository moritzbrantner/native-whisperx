//! Restricted reader for the pre-1.6 PyTorch serialization used by OPUS-MT.
//!
//! This is deliberately not a general pickle implementation. It interprets a
//! small data-only protocol-2 subset, allowlists the three globals emitted by
//! tensor state dictionaries, and never imports or invokes Python code.

use candle_core::{DType, Device, Shape, Tensor};
use candle_nn::{var_builder::SimpleBackend, VarBuilder};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

use super::LegacyPytorchError;

const MAGIC_NUMBER: i128 = 119_547_037_146_038_801_333_356;
const PROTOCOL_VERSION: i128 = 1001;
const MAX_PICKLE_BYTES: usize = 2 * 1024 * 1024;
const MAX_MEMO_ENTRIES: usize = 4_096;
const MAX_STACK_ENTRIES: usize = 4_096;
const MAX_COLLECTION_ENTRIES: usize = 4_096;
const MAX_STRING_BYTES: usize = 1_024;
const MAX_OBJECT_ALLOCATION_BYTES: usize = 64 * 1024 * 1024;
const MAX_RAW_STORAGE_BYTES: u64 = 1024 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Object {
    Class(String, String),
    Int(i128),
    Unicode(String),
    Bool(bool),
    None,
    Tuple(Vec<Object>),
    List(Vec<Object>),
    Dict(Vec<(Object, Object)>),
    Mark,
    Reduce(Box<Object>, Box<Object>),
    Build(Box<Object>, Box<Object>),
    Persistent(Box<Object>),
}

struct PickleParser<'a, R> {
    reader: &'a mut R,
    start_offset: u64,
    offset: usize,
    stack: Vec<Object>,
    memo: HashMap<u32, Object>,
    allocated: usize,
}

impl<'a, R: BufRead> PickleParser<'a, R> {
    fn new(reader: &'a mut R, start_offset: u64) -> Self {
        Self {
            reader,
            start_offset,
            offset: 0,
            stack: Vec::new(),
            memo: HashMap::new(),
            allocated: 0,
        }
    }

    fn parse(mut self) -> Result<Object, LegacyPytorchError> {
        loop {
            let opcode_offset = self.absolute_offset();
            let opcode = self.byte()?;
            match opcode {
                0x80 => {
                    let version = self.byte()?;
                    if version != 2 {
                        return Err(LegacyPytorchError::UnsupportedPickleProtocol { version });
                    }
                }
                b'c' => {
                    let module = self.line()?;
                    let name = self.line()?;
                    if !matches!(
                        (module.as_str(), name.as_str()),
                        ("collections", "OrderedDict")
                            | ("torch._utils", "_rebuild_tensor_v2")
                            | ("torch", "FloatStorage")
                            | ("torch", "HalfStorage")
                            | ("torch", "BFloat16Storage")
                            | ("torch", "DoubleStorage")
                            | ("torch", "LongStorage")
                    ) {
                        return Err(LegacyPytorchError::UnsupportedGlobal { module, name });
                    }
                    self.push(Object::Class(module, name))?;
                }
                b'q' => {
                    let id = self.byte()? as u32;
                    self.memo_put(id)?;
                }
                b'r' => {
                    let id = self.u32()?;
                    self.memo_put(id)?;
                }
                b'h' => {
                    let id = self.byte()? as u32;
                    self.push(self.memo_get(id)?)?;
                }
                b'j' => {
                    let id = self.u32()?;
                    self.push(self.memo_get(id)?)?;
                }
                b'(' => self.push(Object::Mark)?,
                b')' => self.push(Object::Tuple(Vec::new()))?,
                b']' => self.push(Object::List(Vec::new()))?,
                b'}' => self.push(Object::Dict(Vec::new()))?,
                b'X' => {
                    let len = self.u32()? as usize;
                    if len > MAX_STRING_BYTES {
                        return Err(self.malformed("pickle string exceeds the supported limit"));
                    }
                    let bytes = self.bytes(len)?;
                    let value = String::from_utf8(bytes)
                        .map_err(|_| self.malformed("pickle string is not valid UTF-8"))?;
                    self.push(Object::Unicode(value))?;
                }
                b'J' => {
                    let value = self.i32()?;
                    self.push(Object::Int(value as i128))?;
                }
                b'K' => {
                    let value = self.byte()?;
                    self.push(Object::Int(value as i128))?;
                }
                b'M' => {
                    let value = self.u16()?;
                    self.push(Object::Int(value as i128))?;
                }
                0x8a => {
                    let len = self.byte()? as usize;
                    if len > 16 {
                        return Err(self.malformed("LONG1 exceeds the supported integer width"));
                    }
                    let bytes = self.bytes(len)?;
                    let mut value = 0i128;
                    for (index, byte) in bytes.iter().enumerate() {
                        value |= (*byte as i128) << (index * 8);
                    }
                    self.push(Object::Int(value))?;
                }
                0x88 => self.push(Object::Bool(true))?,
                0x89 => self.push(Object::Bool(false))?,
                b'N' => self.push(Object::None)?,
                0x85 => {
                    let value = self.pop()?;
                    self.push(Object::Tuple(vec![value]))?;
                }
                0x86 => {
                    let second = self.pop()?;
                    let first = self.pop()?;
                    self.push(Object::Tuple(vec![first, second]))?;
                }
                0x87 => {
                    let third = self.pop()?;
                    let second = self.pop()?;
                    let first = self.pop()?;
                    self.push(Object::Tuple(vec![first, second, third]))?;
                }
                b't' => {
                    let values = self.pop_to_mark()?;
                    self.push(Object::Tuple(values))?;
                }
                b'd' => {
                    let values = self.pop_to_mark()?;
                    self.push(Object::Dict(pairs(values, self.absolute_offset())?))?;
                }
                b'Q' => {
                    let value = self.pop()?;
                    self.push(Object::Persistent(Box::new(value)))?;
                }
                b'R' => {
                    let args = self.pop()?;
                    let callable = self.pop()?;
                    let reduced = match (&callable, &args) {
                        (Object::Class(module, name), Object::Tuple(values))
                            if module == "collections"
                                && name == "OrderedDict"
                                && values.is_empty() =>
                        {
                            Object::Dict(Vec::new())
                        }
                        (Object::Class(module, name), Object::Tuple(_))
                            if module == "torch._utils" && name == "_rebuild_tensor_v2" =>
                        {
                            Object::Reduce(Box::new(callable), Box::new(args))
                        }
                        _ => {
                            return Err(self.malformed(
                                "REDUCE callable is outside the tensor state-dict subset",
                            ));
                        }
                    };
                    self.push(reduced)?;
                }
                b'b' => {
                    let state = self.pop()?;
                    let value = self.pop()?;
                    self.push(Object::Build(Box::new(value), Box::new(state)))?;
                }
                b's' => {
                    let value = self.pop()?;
                    let key = self.pop()?;
                    self.dict_extend(vec![(key, value)])?;
                }
                b'u' => {
                    let values = self.pop_to_mark()?;
                    let pairs = pairs(values, self.absolute_offset())?;
                    self.dict_extend(pairs)?;
                }
                b'a' => {
                    let value = self.pop()?;
                    self.list_extend(vec![value])?;
                }
                b'e' => {
                    let values = self.pop_to_mark()?;
                    self.list_extend(values)?;
                }
                b'.' => {
                    if self.stack.len() != 1 {
                        return Err(self.malformed("pickle STOP did not leave one result"));
                    }
                    return self.pop();
                }
                unsupported => {
                    return Err(LegacyPytorchError::UnsupportedOpcode {
                        offset: opcode_offset,
                        opcode: unsupported,
                    });
                }
            }
        }
    }

    fn absolute_offset(&self) -> u64 {
        self.start_offset + self.offset as u64
    }

    fn malformed(&self, detail: impl Into<String>) -> LegacyPytorchError {
        LegacyPytorchError::MalformedPickle {
            offset: self.absolute_offset(),
            detail: detail.into(),
        }
    }

    fn push(&mut self, value: Object) -> Result<(), LegacyPytorchError> {
        if self.stack.len() >= MAX_STACK_ENTRIES {
            return Err(self.malformed("pickle stack exceeds the supported limit"));
        }
        self.charge_object(&value)?;
        self.stack.push(value);
        Ok(())
    }

    fn pop(&mut self) -> Result<Object, LegacyPytorchError> {
        self.stack
            .pop()
            .ok_or_else(|| self.malformed("pickle stack underflow"))
    }

    fn pop_to_mark(&mut self) -> Result<Vec<Object>, LegacyPytorchError> {
        let index = self
            .stack
            .iter()
            .rposition(|value| value == &Object::Mark)
            .ok_or_else(|| self.malformed("pickle MARK is missing"))?;
        let values = self.stack.split_off(index + 1);
        self.stack.pop();
        if values.len() > MAX_COLLECTION_ENTRIES {
            return Err(self.malformed("pickle collection exceeds the supported limit"));
        }
        Ok(values)
    }

    fn memo_put(&mut self, id: u32) -> Result<(), LegacyPytorchError> {
        if self.memo.len() >= MAX_MEMO_ENTRIES {
            return Err(self.malformed("pickle memo exceeds the supported limit"));
        }
        if self.memo.contains_key(&id) {
            return Err(self.malformed(format!("duplicate pickle memo id {id}")));
        }
        let value = self
            .stack
            .last()
            .cloned()
            .ok_or_else(|| self.malformed("pickle memo references an empty stack"))?;
        self.charge_object(&value)?;
        self.memo.insert(id, value);
        Ok(())
    }

    fn charge_object(&mut self, value: &Object) -> Result<(), LegacyPytorchError> {
        let mut pending = vec![value];
        let mut cost = 0usize;
        while let Some(value) = pending.pop() {
            cost = cost.saturating_add(std::mem::size_of::<Object>());
            match value {
                Object::Class(module, name) => {
                    cost = cost.saturating_add(module.len()).saturating_add(name.len());
                }
                Object::Unicode(value) => cost = cost.saturating_add(value.len()),
                Object::Tuple(values) | Object::List(values) => pending.extend(values),
                Object::Dict(values) => {
                    for (key, value) in values {
                        pending.push(key);
                        pending.push(value);
                    }
                }
                Object::Reduce(callable, args) | Object::Build(callable, args) => {
                    pending.push(callable);
                    pending.push(args);
                }
                Object::Persistent(value) => pending.push(value),
                Object::Int(_) | Object::Bool(_) | Object::None | Object::Mark => {}
            }
            if self.allocated.saturating_add(cost) > MAX_OBJECT_ALLOCATION_BYTES {
                return Err(self.malformed("pickle object allocation exceeds the supported limit"));
            }
        }
        self.allocated += cost;
        Ok(())
    }

    fn memo_get(&self, id: u32) -> Result<Object, LegacyPytorchError> {
        self.memo
            .get(&id)
            .cloned()
            .ok_or_else(|| self.malformed(format!("pickle memo id {id} is missing")))
    }

    fn dict_extend(&mut self, values: Vec<(Object, Object)>) -> Result<(), LegacyPytorchError> {
        match self.stack.last_mut() {
            Some(Object::Dict(dict)) => {
                if dict.len().saturating_add(values.len()) > MAX_COLLECTION_ENTRIES {
                    return Err(LegacyPytorchError::MalformedPickle {
                        offset: self.absolute_offset(),
                        detail: "pickle dictionary exceeds the supported limit".to_string(),
                    });
                }
                dict.extend(values);
                Ok(())
            }
            _ => Err(self.malformed("SETITEM target is not a dictionary")),
        }
    }

    fn list_extend(&mut self, values: Vec<Object>) -> Result<(), LegacyPytorchError> {
        match self.stack.last_mut() {
            Some(Object::List(list)) => {
                if list.len().saturating_add(values.len()) > MAX_COLLECTION_ENTRIES {
                    return Err(LegacyPytorchError::MalformedPickle {
                        offset: self.absolute_offset(),
                        detail: "pickle list exceeds the supported limit".to_string(),
                    });
                }
                list.extend(values);
                Ok(())
            }
            _ => Err(self.malformed("APPEND target is not a list")),
        }
    }

    fn read_exact(&mut self, bytes: &mut [u8]) -> Result<(), LegacyPytorchError> {
        if self.offset.saturating_add(bytes.len()) > MAX_PICKLE_BYTES {
            return Err(self.malformed("pickle stream exceeds the supported size"));
        }
        self.reader
            .read_exact(bytes)
            .map_err(|error| LegacyPytorchError::MalformedPickle {
                offset: self.absolute_offset(),
                detail: format!("truncated pickle data: {error}"),
            })?;
        self.offset += bytes.len();
        Ok(())
    }

    fn byte(&mut self) -> Result<u8, LegacyPytorchError> {
        let mut bytes = [0];
        self.read_exact(&mut bytes)?;
        Ok(bytes[0])
    }

    fn bytes(&mut self, len: usize) -> Result<Vec<u8>, LegacyPytorchError> {
        let mut bytes = vec![0; len];
        self.read_exact(&mut bytes)?;
        Ok(bytes)
    }

    fn u16(&mut self) -> Result<u16, LegacyPytorchError> {
        let mut bytes = [0; 2];
        self.read_exact(&mut bytes)?;
        Ok(u16::from_le_bytes(bytes))
    }

    fn u32(&mut self) -> Result<u32, LegacyPytorchError> {
        let mut bytes = [0; 4];
        self.read_exact(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn i32(&mut self) -> Result<i32, LegacyPytorchError> {
        let mut bytes = [0; 4];
        self.read_exact(&mut bytes)?;
        Ok(i32::from_le_bytes(bytes))
    }

    fn line(&mut self) -> Result<String, LegacyPytorchError> {
        let mut bytes = Vec::new();
        loop {
            let byte = self.byte()?;
            if byte == b'\n' {
                break;
            }
            if bytes.len() >= MAX_STRING_BYTES {
                return Err(self.malformed("pickle line exceeds the supported limit"));
            }
            bytes.push(byte);
        }
        if bytes.last() == Some(&b'\r') {
            bytes.pop();
        }
        String::from_utf8(bytes).map_err(|_| self.malformed("pickle line is not valid UTF-8"))
    }
}

fn pairs(values: Vec<Object>, offset: u64) -> Result<Vec<(Object, Object)>, LegacyPytorchError> {
    if !values.len().is_multiple_of(2) {
        return Err(LegacyPytorchError::MalformedPickle {
            offset,
            detail: "dictionary contains an odd number of values".to_string(),
        });
    }
    let mut iter = values.into_iter();
    let mut pairs = Vec::new();
    while let Some(key) = iter.next() {
        let value = iter.next().expect("even value count checked above");
        pairs.push((key, value));
    }
    Ok(pairs)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TensorInfo {
    shape: Vec<usize>,
    storage_key: String,
    storage_offset: usize,
    storage_numel: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StorageInfo {
    numel: usize,
    data_offset: u64,
}

#[derive(Debug)]
struct LegacyPytorchTensors {
    path: PathBuf,
    tensors: HashMap<String, TensorInfo>,
    storages: HashMap<String, StorageInfo>,
}

pub(super) fn is_zip_archive(path: &Path) -> Result<bool, LegacyPytorchError> {
    let mut file = File::open(path).map_err(|error| LegacyPytorchError::Io {
        detail: format!("failed to open weights: {error}"),
    })?;
    let mut magic = [0; 4];
    file.read_exact(&mut magic)
        .map_err(|error| LegacyPytorchError::Io {
            detail: format!("failed to read weights header: {error}"),
        })?;
    Ok(matches!(
        magic,
        [b'P', b'K', 3, 4] | [b'P', b'K', 5, 6] | [b'P', b'K', 7, 8]
    ))
}

pub(super) fn var_builder<'a>(
    path: &Path,
    config: &candle_transformers::models::marian::Config,
    dtype: DType,
    device: &Device,
) -> Result<VarBuilder<'a>, LegacyPytorchError> {
    let backend = LegacyPytorchTensors::read(path, config)?;
    Ok(VarBuilder::from_backend(
        Box::new(backend),
        dtype,
        device.clone(),
    ))
}

impl LegacyPytorchTensors {
    fn read(
        path: &Path,
        config: &candle_transformers::models::marian::Config,
    ) -> Result<Self, LegacyPytorchError> {
        let file = File::open(path).map_err(|error| LegacyPytorchError::Io {
            detail: format!("failed to open legacy weights: {error}"),
        })?;
        let file_len = file
            .metadata()
            .map_err(|error| LegacyPytorchError::Io {
                detail: format!("failed to inspect legacy weights: {error}"),
            })?
            .len();
        let mut reader = BufReader::new(file);
        let magic = read_pickle(&mut reader)?;
        if magic != Object::Int(MAGIC_NUMBER) {
            return Err(LegacyPytorchError::InvalidMagic);
        }
        let protocol = read_pickle(&mut reader)?;
        if protocol != Object::Int(PROTOCOL_VERSION) {
            return Err(LegacyPytorchError::UnsupportedSerializationProtocol {
                value: object_integer(&protocol).unwrap_or(-1),
            });
        }
        validate_system_info(read_pickle(&mut reader)?)?;
        let tensors = extract_state_dict(read_pickle(&mut reader)?)?;
        validate_required_weights(&tensors, config)?;
        let storage_keys = extract_storage_keys(read_pickle(&mut reader)?)?;
        let mut declared_storages = HashMap::new();
        for tensor in tensors.values() {
            match declared_storages.insert(tensor.storage_key.clone(), tensor.storage_numel) {
                Some(previous) if previous != tensor.storage_numel => {
                    return Err(LegacyPytorchError::InvalidStorageSize {
                        key: tensor.storage_key.clone(),
                    });
                }
                _ => {}
            }
        }
        let referenced = declared_storages.keys().cloned().collect::<HashSet<_>>();
        let listed = storage_keys.iter().cloned().collect::<HashSet<_>>();
        if listed != referenced {
            let key = referenced
                .symmetric_difference(&listed)
                .next()
                .cloned()
                .unwrap_or_else(|| "<unknown>".to_string());
            return Err(LegacyPytorchError::MissingStorage { key });
        }

        let mut storages = HashMap::new();
        let mut total_raw_bytes = 0u64;
        for key in storage_keys {
            let expected = declared_storages[&key];
            let mut length_bytes = [0; 8];
            reader
                .read_exact(&mut length_bytes)
                .map_err(|_| LegacyPytorchError::TruncatedStorage { key: key.clone() })?;
            let numel = usize::try_from(u64::from_le_bytes(length_bytes))
                .map_err(|_| LegacyPytorchError::InvalidStorageSize { key: key.clone() })?;
            if numel != expected {
                return Err(LegacyPytorchError::InvalidStorageSize { key });
            }
            let data_offset = reader
                .stream_position()
                .map_err(|error| LegacyPytorchError::Io {
                    detail: format!("failed to locate storage `{key}`: {error}"),
                })?;
            let byte_len = (numel as u64)
                .checked_mul(DType::F32.size_in_bytes() as u64)
                .ok_or_else(|| LegacyPytorchError::InvalidStorageSize { key: key.clone() })?;
            total_raw_bytes = total_raw_bytes
                .checked_add(byte_len)
                .ok_or_else(|| LegacyPytorchError::InvalidStorageSize { key: key.clone() })?;
            if total_raw_bytes > MAX_RAW_STORAGE_BYTES {
                return Err(LegacyPytorchError::StorageLimitExceeded);
            }
            let end = data_offset
                .checked_add(byte_len)
                .ok_or_else(|| LegacyPytorchError::InvalidStorageSize { key: key.clone() })?;
            if end > file_len {
                return Err(LegacyPytorchError::TruncatedStorage { key });
            }
            reader
                .seek(SeekFrom::Start(end))
                .map_err(|error| LegacyPytorchError::Io {
                    detail: format!("failed to skip storage `{key}`: {error}"),
                })?;
            if storages
                .insert(key.clone(), StorageInfo { numel, data_offset })
                .is_some()
            {
                return Err(LegacyPytorchError::DuplicateStorage { key });
            }
        }
        if reader
            .stream_position()
            .map_err(|error| LegacyPytorchError::Io {
                detail: format!("failed to validate legacy weights length: {error}"),
            })?
            != file_len
        {
            return Err(LegacyPytorchError::TrailingData);
        }
        Ok(Self {
            path: path.to_path_buf(),
            tensors,
            storages,
        })
    }

    fn load(&self, name: &str) -> candle_core::Result<Option<Tensor>> {
        let Some(info) = self.tensors.get(name) else {
            return Ok(None);
        };
        let storage = self.storages.get(&info.storage_key).ok_or_else(|| {
            candle_core::Error::Msg(format!("validated storage missing for {name}"))
        })?;
        let byte_offset = (info.storage_offset as u64)
            .checked_mul(DType::F32.size_in_bytes() as u64)
            .and_then(|offset| storage.data_offset.checked_add(offset))
            .ok_or_else(|| {
                candle_core::Error::Msg(format!("storage offset overflow for {name}"))
            })?;
        let mut file = File::open(&self.path)?;
        file.seek(SeekFrom::Start(byte_offset))?;
        let byte_len = info
            .shape
            .iter()
            .try_fold(DType::F32.size_in_bytes(), |bytes, dim| {
                bytes.checked_mul(*dim)
            })
            .ok_or_else(|| candle_core::Error::Msg(format!("tensor size overflow for {name}")))?;
        let mut bytes = vec![0; byte_len];
        file.read_exact(&mut bytes)?;
        Tensor::from_raw_buffer(&bytes, DType::F32, &info.shape, &Device::Cpu).map(Some)
    }
}

impl SimpleBackend for LegacyPytorchTensors {
    fn get(
        &self,
        expected: Shape,
        name: &str,
        _hint: candle_nn::Init,
        dtype: DType,
        device: &Device,
    ) -> candle_core::Result<Tensor> {
        let tensor = self.get_unchecked(name, dtype, device)?;
        if tensor.shape() != &expected {
            return Err(candle_core::Error::UnexpectedShape {
                msg: format!("shape mismatch for {name}"),
                expected,
                got: tensor.shape().clone(),
            });
        }
        Ok(tensor)
    }

    fn get_unchecked(
        &self,
        name: &str,
        dtype: DType,
        device: &Device,
    ) -> candle_core::Result<Tensor> {
        self.load(name)?
            .ok_or_else(|| candle_core::Error::CannotFindTensor {
                path: name.to_string(),
            })?
            .to_device(device)?
            .to_dtype(dtype)
    }

    fn contains_tensor(&self, name: &str) -> bool {
        self.tensors.contains_key(name)
    }
}

fn read_pickle<R: BufRead + Seek>(reader: &mut R) -> Result<Object, LegacyPytorchError> {
    let start = reader
        .stream_position()
        .map_err(|error| LegacyPytorchError::Io {
            detail: format!("failed to locate pickle stream: {error}"),
        })?;
    PickleParser::new(reader, start).parse()
}

fn validate_system_info(object: Object) -> Result<(), LegacyPytorchError> {
    let Object::Dict(values) = object else {
        return Err(LegacyPytorchError::InvalidSystemInfo);
    };
    let mut little_endian = None;
    let mut protocol = None;
    let mut type_sizes = None;
    for (key, value) in values {
        match (object_string(&key), value) {
            (Some("little_endian"), Object::Bool(value)) => little_endian = Some(value),
            (Some("protocol_version"), Object::Int(value)) => protocol = Some(value),
            (Some("type_sizes"), Object::Dict(values)) => type_sizes = Some(values),
            _ => return Err(LegacyPytorchError::InvalidSystemInfo),
        }
    }
    let Some(type_sizes) = type_sizes else {
        return Err(LegacyPytorchError::InvalidSystemInfo);
    };
    let sizes = type_sizes
        .iter()
        .filter_map(|(key, value)| Some((object_string(key)?, object_integer(value)?)))
        .collect::<HashMap<_, _>>();
    if little_endian != Some(true)
        || protocol != Some(PROTOCOL_VERSION)
        || type_sizes.len() != 3
        || sizes.get("short") != Some(&2)
        || sizes.get("int") != Some(&4)
        || sizes.get("long") != Some(&4)
    {
        return Err(LegacyPytorchError::InvalidSystemInfo);
    }
    Ok(())
}

fn extract_state_dict(object: Object) -> Result<HashMap<String, TensorInfo>, LegacyPytorchError> {
    let values = match object {
        Object::Build(value, state) => {
            validate_metadata(*state)?;
            match *value {
                Object::Dict(values) => values,
                _ => return Err(LegacyPytorchError::InvalidStateDict),
            }
        }
        Object::Dict(values) => values,
        _ => return Err(LegacyPytorchError::InvalidStateDict),
    };
    let mut tensors = HashMap::new();
    for (name, value) in values {
        let Object::Unicode(name) = name else {
            return Err(LegacyPytorchError::InvalidParameterName {
                name: "<non-string>".to_string(),
            });
        };
        validate_parameter_name(&name)?;
        let tensor = extract_tensor(&name, value)?;
        if tensors.insert(name.clone(), tensor).is_some() {
            return Err(LegacyPytorchError::DuplicateParameter { name });
        }
    }
    if tensors.is_empty() {
        return Err(LegacyPytorchError::InvalidStateDict);
    }
    Ok(tensors)
}

fn validate_metadata(object: Object) -> Result<(), LegacyPytorchError> {
    let Object::Dict(mut values) = object else {
        return Err(LegacyPytorchError::InvalidMetadata);
    };
    if values.len() != 1 || object_string(&values[0].0) != Some("_metadata") {
        return Err(LegacyPytorchError::InvalidMetadata);
    }
    let (_, metadata) = values.pop().expect("length checked");
    let Object::Dict(entries) = metadata else {
        return Err(LegacyPytorchError::InvalidMetadata);
    };
    for (path, value) in entries {
        let Some(path) = object_string(&path) else {
            return Err(LegacyPytorchError::InvalidMetadata);
        };
        if !path.is_empty() {
            validate_parameter_name(path)?;
        }
        let Object::Dict(version) = value else {
            return Err(LegacyPytorchError::InvalidMetadata);
        };
        if version.len() != 1
            || object_string(&version[0].0) != Some("version")
            || object_integer(&version[0].1) != Some(1)
        {
            return Err(LegacyPytorchError::InvalidMetadata);
        }
    }
    Ok(())
}

fn extract_tensor(name: &str, object: Object) -> Result<TensorInfo, LegacyPytorchError> {
    let Object::Reduce(callable, args) = object else {
        return Err(LegacyPytorchError::InvalidTensor {
            parameter: name.to_string(),
        });
    };
    if !matches!(*callable, Object::Class(ref module, ref class) if module == "torch._utils" && class == "_rebuild_tensor_v2")
    {
        return Err(LegacyPytorchError::InvalidTensor {
            parameter: name.to_string(),
        });
    }
    let Object::Tuple(mut args) = *args else {
        return Err(LegacyPytorchError::InvalidTensor {
            parameter: name.to_string(),
        });
    };
    if args.len() != 6 {
        return Err(LegacyPytorchError::InvalidTensor {
            parameter: name.to_string(),
        });
    }
    let hooks = args.pop().expect("length checked");
    let requires_grad = args.pop().expect("length checked");
    let stride = tuple_usizes(args.pop().expect("length checked"), name)?;
    let shape = tuple_usizes(args.pop().expect("length checked"), name)?;
    let storage_offset = object_usize(&args.pop().expect("length checked"), name)?;
    let storage = args.pop().expect("length checked");
    if requires_grad != Object::Bool(false)
        || !matches!(hooks, Object::Dict(ref values) if values.is_empty())
    {
        return Err(LegacyPytorchError::InvalidTensor {
            parameter: name.to_string(),
        });
    }
    if shape.len() != stride.len() || shape.len() > 8 || !is_contiguous(&shape, &stride) {
        return Err(LegacyPytorchError::UnsupportedLayout {
            parameter: name.to_string(),
        });
    }
    let Object::Persistent(storage) = storage else {
        return Err(LegacyPytorchError::InvalidTensor {
            parameter: name.to_string(),
        });
    };
    let Object::Tuple(storage) = *storage else {
        return Err(LegacyPytorchError::InvalidTensor {
            parameter: name.to_string(),
        });
    };
    if storage.len() != 6
        || object_string(&storage[0]) != Some("storage")
        || object_string(&storage[3]) != Some("cpu")
        || storage[5] != Object::None
    {
        return Err(LegacyPytorchError::UnsupportedDType {
            parameter: name.to_string(),
        });
    }
    if !matches!(&storage[1], Object::Class(module, class) if module == "torch" && class == "FloatStorage")
    {
        return Err(LegacyPytorchError::UnsupportedDType {
            parameter: name.to_string(),
        });
    }
    let storage_key = object_string(&storage[2])
        .ok_or_else(|| LegacyPytorchError::InvalidTensor {
            parameter: name.to_string(),
        })?
        .to_string();
    if storage_key.is_empty() || !storage_key.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(LegacyPytorchError::InvalidStorageKey { key: storage_key });
    }
    let storage_numel = object_usize(&storage[4], name)?;
    let info = TensorInfo {
        shape,
        storage_key,
        storage_offset,
        storage_numel,
    };
    if storage_bound(&info) > storage_numel {
        return Err(LegacyPytorchError::StorageBounds {
            parameter: name.to_string(),
        });
    }
    Ok(info)
}

fn extract_storage_keys(object: Object) -> Result<Vec<String>, LegacyPytorchError> {
    let Object::List(values) = object else {
        return Err(LegacyPytorchError::InvalidStorageList);
    };
    let mut seen = HashSet::new();
    let mut keys = Vec::new();
    for value in values {
        let Object::Unicode(key) = value else {
            return Err(LegacyPytorchError::InvalidStorageList);
        };
        if !seen.insert(key.clone()) {
            return Err(LegacyPytorchError::DuplicateStorage { key });
        }
        keys.push(key);
    }
    Ok(keys)
}

fn validate_parameter_name(name: &str) -> Result<(), LegacyPytorchError> {
    if name.is_empty()
        || name.len() > 256
        || name.starts_with('.')
        || name.ends_with('.')
        || name.contains("..")
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.'))
    {
        return Err(LegacyPytorchError::InvalidParameterName {
            name: name.to_string(),
        });
    }
    Ok(())
}

fn validate_required_weights(
    tensors: &HashMap<String, TensorInfo>,
    config: &candle_transformers::models::marian::Config,
) -> Result<(), LegacyPytorchError> {
    let mut expected = vec![
        (
            "final_logits_bias".to_string(),
            vec![1, config.decoder_vocab_size.unwrap_or(config.vocab_size)],
        ),
        (
            "model.shared.weight".to_string(),
            vec![config.vocab_size, config.d_model],
        ),
    ];
    expected.extend([
        (
            "model.encoder.embed_tokens.weight".to_string(),
            vec![config.vocab_size, config.d_model],
        ),
        (
            "model.encoder.embed_positions.weight".to_string(),
            vec![config.max_position_embeddings, config.d_model],
        ),
        (
            "model.decoder.embed_tokens.weight".to_string(),
            vec![config.vocab_size, config.d_model],
        ),
        (
            "model.decoder.embed_positions.weight".to_string(),
            vec![config.max_position_embeddings, config.d_model],
        ),
    ]);
    for index in 0..config.encoder_layers {
        let prefix = format!("model.encoder.layers.{index}");
        add_attention(
            &mut expected,
            &format!("{prefix}.self_attn"),
            config.d_model,
        );
        add_norm(
            &mut expected,
            &format!("{prefix}.self_attn_layer_norm"),
            config.d_model,
        );
        add_linear(
            &mut expected,
            &format!("{prefix}.fc1"),
            config.encoder_ffn_dim,
            config.d_model,
        );
        add_linear(
            &mut expected,
            &format!("{prefix}.fc2"),
            config.d_model,
            config.encoder_ffn_dim,
        );
        add_norm(
            &mut expected,
            &format!("{prefix}.final_layer_norm"),
            config.d_model,
        );
    }
    for index in 0..config.decoder_layers {
        let prefix = format!("model.decoder.layers.{index}");
        add_attention(
            &mut expected,
            &format!("{prefix}.self_attn"),
            config.d_model,
        );
        add_norm(
            &mut expected,
            &format!("{prefix}.self_attn_layer_norm"),
            config.d_model,
        );
        add_attention(
            &mut expected,
            &format!("{prefix}.encoder_attn"),
            config.d_model,
        );
        add_norm(
            &mut expected,
            &format!("{prefix}.encoder_attn_layer_norm"),
            config.d_model,
        );
        add_linear(
            &mut expected,
            &format!("{prefix}.fc1"),
            config.decoder_ffn_dim,
            config.d_model,
        );
        add_linear(
            &mut expected,
            &format!("{prefix}.fc2"),
            config.d_model,
            config.decoder_ffn_dim,
        );
        add_norm(
            &mut expected,
            &format!("{prefix}.final_layer_norm"),
            config.d_model,
        );
    }
    let allowed = expected.into_iter().collect::<HashMap<_, _>>();
    let required = allowed
        .keys()
        .filter(|name| !name.contains(".embed_tokens.") && !name.contains(".embed_positions."))
        .cloned()
        .collect::<HashSet<_>>();
    for (name, tensor) in tensors {
        let Some(expected) = allowed.get(name) else {
            return Err(LegacyPytorchError::UnexpectedMarianWeight { name: name.clone() });
        };
        if &tensor.shape != expected {
            return Err(LegacyPytorchError::ShapeMismatch {
                name: name.clone(),
                expected: expected.clone(),
                actual: tensor.shape.clone(),
            });
        }
    }
    for name in required {
        if !tensors.contains_key(&name) {
            return Err(LegacyPytorchError::MissingMarianWeight { name });
        }
    }
    Ok(())
}

fn add_attention(weights: &mut Vec<(String, Vec<usize>)>, prefix: &str, model: usize) {
    for projection in ["q_proj", "k_proj", "v_proj", "out_proj"] {
        add_linear(weights, &format!("{prefix}.{projection}"), model, model);
    }
}

fn add_linear(weights: &mut Vec<(String, Vec<usize>)>, prefix: &str, output: usize, input: usize) {
    weights.push((format!("{prefix}.weight"), vec![output, input]));
    weights.push((format!("{prefix}.bias"), vec![output]));
}

fn add_norm(weights: &mut Vec<(String, Vec<usize>)>, prefix: &str, model: usize) {
    weights.push((format!("{prefix}.weight"), vec![model]));
    weights.push((format!("{prefix}.bias"), vec![model]));
}

fn is_contiguous(shape: &[usize], stride: &[usize]) -> bool {
    let mut expected = 1usize;
    for (&dim, &actual) in shape.iter().zip(stride).rev() {
        if dim > 1 && actual != expected {
            return false;
        }
        let Some(next) = expected.checked_mul(dim) else {
            return false;
        };
        expected = next;
    }
    true
}

fn storage_bound(info: &TensorInfo) -> usize {
    info.shape
        .iter()
        .try_fold(1usize, |total, dim| total.checked_mul(*dim))
        .and_then(|numel| info.storage_offset.checked_add(numel))
        .unwrap_or(usize::MAX)
}

fn tuple_usizes(object: Object, parameter: &str) -> Result<Vec<usize>, LegacyPytorchError> {
    let Object::Tuple(values) = object else {
        return Err(LegacyPytorchError::InvalidTensor {
            parameter: parameter.to_string(),
        });
    };
    values
        .iter()
        .map(|value| object_usize(value, parameter))
        .collect()
}

fn object_usize(object: &Object, parameter: &str) -> Result<usize, LegacyPytorchError> {
    let Some(value) = object_integer(object) else {
        return Err(LegacyPytorchError::InvalidTensor {
            parameter: parameter.to_string(),
        });
    };
    usize::try_from(value).map_err(|_| LegacyPytorchError::InvalidTensor {
        parameter: parameter.to_string(),
    })
}

fn object_integer(object: &Object) -> Option<i128> {
    match object {
        Object::Int(value) => Some(*value),
        _ => None,
    }
}

fn object_string(object: &Object) -> Option<&str> {
    match object {
        Object::Unicode(value) => Some(value),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiny_legacy_pickle_state_dict_loads_without_executing_python() {
        let fixture = fixture(
            &[
                tensor("final_logits_bias", "1", vec![1, 2], vec![2, 1], 2),
                tensor("model.shared.weight", "2", vec![2, 2], vec![2, 1], 4),
            ],
            &[("1", vec![0.0, 1.0]), ("2", vec![2.0, 3.0, 4.0, 5.0])],
        );
        let file = write_fixture(&fixture);

        let backend = LegacyPytorchTensors::read(file.path(), &tiny_config())
            .expect("supported tensor-only fixture");
        let tensor = backend
            .load("model.shared.weight")
            .expect("read tensor")
            .expect("known tensor");

        assert_eq!(tensor.dims(), &[2, 2]);
        assert_eq!(
            tensor.to_vec2::<f32>().unwrap(),
            vec![vec![2.0, 3.0], vec![4.0, 5.0]]
        );
    }

    #[test]
    fn zip_archive_signature_keeps_the_existing_candle_loader_selected() {
        let file = write_fixture(b"PK\x03\x04unchanged zip payload");

        assert!(is_zip_archive(file.path()).expect("read zip signature"));
    }

    #[test]
    fn malformed_pickle_is_rejected_with_an_offset() {
        let mut fixture = header();
        fixture.extend([0x80, 2, b'}', b'(', b'X', 1, 0, 0, 0, b'x', b'u', b'.']);

        let error = LegacyPytorchTensors::read(write_fixture(&fixture).path(), &tiny_config())
            .expect_err("odd SETITEMS payload must fail");

        assert!(matches!(error, LegacyPytorchError::MalformedPickle { .. }));
    }

    #[test]
    fn unsupported_opcode_is_rejected_before_any_payload_can_run() {
        let mut fixture = header();
        fixture.extend([0x80, 2, 0xff]);

        let error = LegacyPytorchTensors::read(write_fixture(&fixture).path(), &tiny_config())
            .expect_err("unknown opcode must fail closed");

        assert!(matches!(
            error,
            LegacyPytorchError::UnsupportedOpcode { opcode: 0xff, .. }
        ));
    }

    #[test]
    fn arbitrary_pickle_global_is_rejected_by_name() {
        let mut fixture = header();
        fixture.extend([0x80, 2, b'c']);
        fixture.extend(b"os\nsystem\n");

        let error = LegacyPytorchTensors::read(write_fixture(&fixture).path(), &tiny_config())
            .expect_err("arbitrary global must fail closed");

        assert_eq!(
            error,
            LegacyPytorchError::UnsupportedGlobal {
                module: "os".to_string(),
                name: "system".to_string(),
            }
        );
    }

    #[test]
    fn provider_error_retains_the_typed_legacy_rejection() {
        let rejection = LegacyPytorchError::UnsupportedGlobal {
            module: "os".to_string(),
            name: "system".to_string(),
        };

        let provider_error =
            super::super::TranslationModelError::from_legacy_pytorch(rejection.clone());

        assert_eq!(provider_error.legacy_pytorch_error(), Some(&rejection));
    }

    #[test]
    fn unsupported_tensor_dtype_is_rejected_with_a_typed_error() {
        let mut fixture = fixture(
            &[
                tensor("final_logits_bias", "1", vec![1, 2], vec![2, 1], 2),
                tensor("model.shared.weight", "2", vec![2, 2], vec![2, 1], 4),
            ],
            &[("1", vec![0.0, 1.0]), ("2", vec![2.0, 3.0, 4.0, 5.0])],
        );
        let start = fixture
            .windows(b"FloatStorage".len())
            .position(|window| window == b"FloatStorage")
            .expect("fixture storage class");
        fixture.splice(
            start..start + b"FloatStorage".len(),
            b"HalfStorage".iter().copied(),
        );

        let error = LegacyPytorchTensors::read(write_fixture(&fixture).path(), &tiny_config())
            .expect_err("non-F32 Marian weight must fail closed");

        assert_eq!(
            error,
            LegacyPytorchError::UnsupportedDType {
                parameter: "final_logits_bias".to_string(),
            }
        );
    }

    #[test]
    fn truncated_raw_storage_is_rejected_before_model_construction() {
        let mut fixture = fixture(
            &[
                tensor("final_logits_bias", "1", vec![1, 2], vec![2, 1], 2),
                tensor("model.shared.weight", "2", vec![2, 2], vec![2, 1], 4),
            ],
            &[("1", vec![0.0, 1.0]), ("2", vec![2.0, 3.0, 4.0, 5.0])],
        );
        fixture.pop();

        let error = LegacyPytorchTensors::read(write_fixture(&fixture).path(), &tiny_config())
            .expect_err("truncated storage must fail closed");

        assert!(matches!(error, LegacyPytorchError::TruncatedStorage { key } if key == "2"));
    }

    #[test]
    fn required_marian_shape_mismatch_is_typed() {
        let fixture = fixture(
            &[
                tensor("final_logits_bias", "1", vec![2], vec![1], 2),
                tensor("model.shared.weight", "2", vec![2, 2], vec![2, 1], 4),
            ],
            &[("1", vec![0.0, 1.0]), ("2", vec![2.0, 3.0, 4.0, 5.0])],
        );

        let error = LegacyPytorchTensors::read(write_fixture(&fixture).path(), &tiny_config())
            .expect_err("wrong Marian shape must fail before construction");

        assert_eq!(
            error,
            LegacyPytorchError::ShapeMismatch {
                name: "final_logits_bias".to_string(),
                expected: vec![1, 2],
                actual: vec![2],
            }
        );
    }

    #[test]
    fn tensor_shape_cannot_exceed_declared_storage() {
        let fixture = fixture(
            &[
                tensor("final_logits_bias", "1", vec![1, 2], vec![2, 1], 1),
                tensor("model.shared.weight", "2", vec![2, 2], vec![2, 1], 4),
            ],
            &[("1", vec![0.0]), ("2", vec![2.0, 3.0, 4.0, 5.0])],
        );

        let error = LegacyPytorchTensors::read(write_fixture(&fixture).path(), &tiny_config())
            .expect_err("shape beyond storage must fail before raw reads");

        assert_eq!(
            error,
            LegacyPytorchError::StorageBounds {
                parameter: "final_logits_bias".to_string(),
            }
        );
    }

    #[test]
    fn raw_storage_size_must_match_the_pickled_descriptor() {
        let fixture = fixture(
            &[
                tensor("final_logits_bias", "1", vec![1, 2], vec![2, 1], 2),
                tensor("model.shared.weight", "2", vec![2, 2], vec![2, 1], 4),
            ],
            &[("1", vec![0.0, 1.0, 2.0]), ("2", vec![2.0, 3.0, 4.0, 5.0])],
        );

        let error = LegacyPytorchTensors::read(write_fixture(&fixture).path(), &tiny_config())
            .expect_err("raw and declared storage sizes must agree");

        assert_eq!(
            error,
            LegacyPytorchError::InvalidStorageSize {
                key: "1".to_string()
            }
        );
    }

    #[test]
    fn duplicate_parameter_is_rejected() {
        let fixture = fixture(
            &[
                tensor("final_logits_bias", "1", vec![1, 2], vec![2, 1], 2),
                tensor("final_logits_bias", "1", vec![1, 2], vec![2, 1], 2),
                tensor("model.shared.weight", "2", vec![2, 2], vec![2, 1], 4),
            ],
            &[("1", vec![0.0, 1.0]), ("2", vec![2.0, 3.0, 4.0, 5.0])],
        );

        let error = LegacyPytorchTensors::read(write_fixture(&fixture).path(), &tiny_config())
            .expect_err("duplicate parameter must fail closed");

        assert_eq!(
            error,
            LegacyPytorchError::DuplicateParameter {
                name: "final_logits_bias".to_string(),
            }
        );
    }

    #[test]
    fn unsupported_marian_parameter_name_is_rejected() {
        let fixture = fixture(
            &[
                tensor("final_logits_bias", "1", vec![1, 2], vec![2, 1], 2),
                tensor("model.shared.weight", "2", vec![2, 2], vec![2, 1], 4),
                tensor("model.unexpected.weight", "3", vec![1], vec![1], 1),
            ],
            &[
                ("1", vec![0.0, 1.0]),
                ("2", vec![2.0, 3.0, 4.0, 5.0]),
                ("3", vec![6.0]),
            ],
        );

        let error = LegacyPytorchTensors::read(write_fixture(&fixture).path(), &tiny_config())
            .expect_err("unknown Marian parameter must fail closed");

        assert_eq!(
            error,
            LegacyPytorchError::UnexpectedMarianWeight {
                name: "model.unexpected.weight".to_string(),
            }
        );
    }

    #[test]
    fn missing_required_marian_parameter_is_rejected() {
        let fixture = fixture(
            &[tensor(
                "model.shared.weight",
                "2",
                vec![2, 2],
                vec![2, 1],
                4,
            )],
            &[("2", vec![2.0, 3.0, 4.0, 5.0])],
        );

        let error = LegacyPytorchTensors::read(write_fixture(&fixture).path(), &tiny_config())
            .expect_err("missing Marian parameter must fail before construction");

        assert_eq!(
            error,
            LegacyPytorchError::MissingMarianWeight {
                name: "final_logits_bias".to_string(),
            }
        );
    }

    fn tiny_config() -> candle_transformers::models::marian::Config {
        let mut config = candle_transformers::models::marian::Config::opus_mt_fr_en();
        config.vocab_size = 2;
        config.decoder_vocab_size = Some(2);
        config.d_model = 2;
        config.encoder_layers = 0;
        config.decoder_layers = 0;
        config
    }

    #[derive(Clone)]
    struct FixtureTensor {
        name: &'static str,
        storage_key: &'static str,
        shape: Vec<usize>,
        stride: Vec<usize>,
        storage_numel: usize,
    }

    fn tensor(
        name: &'static str,
        storage_key: &'static str,
        shape: Vec<usize>,
        stride: Vec<usize>,
        storage_numel: usize,
    ) -> FixtureTensor {
        FixtureTensor {
            name,
            storage_key,
            shape,
            stride,
            storage_numel,
        }
    }

    fn fixture(tensors: &[FixtureTensor], storages: &[(&str, Vec<f32>)]) -> Vec<u8> {
        let mut bytes = header();
        bytes.extend([0x80, 2, b'}', b'(']);
        for tensor in tensors {
            unicode(&mut bytes, tensor.name);
            global(&mut bytes, "torch._utils", "_rebuild_tensor_v2");
            bytes.push(b'(');
            bytes.push(b'(');
            unicode(&mut bytes, "storage");
            global(&mut bytes, "torch", "FloatStorage");
            unicode(&mut bytes, tensor.storage_key);
            unicode(&mut bytes, "cpu");
            integer(&mut bytes, tensor.storage_numel);
            bytes.push(b'N');
            bytes.push(b't');
            bytes.push(b'Q');
            integer(&mut bytes, 0);
            tuple(&mut bytes, &tensor.shape);
            tuple(&mut bytes, &tensor.stride);
            bytes.push(0x89);
            bytes.push(b'}');
            bytes.push(b't');
            bytes.push(b'R');
        }
        bytes.extend([b'u', b'.']);
        bytes.extend([0x80, 2, b']', b'(']);
        for (key, _) in storages {
            unicode(&mut bytes, key);
        }
        bytes.extend([b'e', b'.']);
        for (_, values) in storages {
            bytes.extend((values.len() as u64).to_le_bytes());
            for value in values {
                bytes.extend(value.to_le_bytes());
            }
        }
        bytes
    }

    fn header() -> Vec<u8> {
        let mut bytes = vec![0x80, 2, 0x8a, 10];
        bytes.extend(&MAGIC_NUMBER.to_le_bytes()[..10]);
        bytes.push(b'.');
        bytes.extend([0x80, 2, b'M']);
        bytes.extend((PROTOCOL_VERSION as u16).to_le_bytes());
        bytes.push(b'.');
        bytes.extend([0x80, 2, b'}', b'(']);
        unicode(&mut bytes, "protocol_version");
        integer(&mut bytes, PROTOCOL_VERSION as usize);
        unicode(&mut bytes, "little_endian");
        bytes.push(0x88);
        unicode(&mut bytes, "type_sizes");
        bytes.push(b'}');
        bytes.push(b'(');
        for (name, value) in [("short", 2), ("int", 4), ("long", 4)] {
            unicode(&mut bytes, name);
            integer(&mut bytes, value);
        }
        bytes.extend([b'u', b'u', b'.']);
        bytes
    }

    fn global(bytes: &mut Vec<u8>, module: &str, name: &str) {
        bytes.push(b'c');
        bytes.extend(module.as_bytes());
        bytes.push(b'\n');
        bytes.extend(name.as_bytes());
        bytes.push(b'\n');
    }

    fn unicode(bytes: &mut Vec<u8>, value: &str) {
        bytes.push(b'X');
        bytes.extend((value.len() as u32).to_le_bytes());
        bytes.extend(value.as_bytes());
    }

    fn integer(bytes: &mut Vec<u8>, value: usize) {
        if value <= u8::MAX as usize {
            bytes.extend([b'K', value as u8]);
        } else if value <= u16::MAX as usize {
            bytes.push(b'M');
            bytes.extend((value as u16).to_le_bytes());
        } else {
            bytes.push(b'J');
            bytes.extend((value as i32).to_le_bytes());
        }
    }

    fn tuple(bytes: &mut Vec<u8>, values: &[usize]) {
        bytes.push(b'(');
        for value in values {
            integer(bytes, *value);
        }
        bytes.push(b't');
    }

    fn write_fixture(bytes: &[u8]) -> tempfile::NamedTempFile {
        let mut file = tempfile::NamedTempFile::new().expect("temporary fixture");
        std::io::Write::write_all(&mut file, bytes).expect("write fixture");
        file
    }
}
