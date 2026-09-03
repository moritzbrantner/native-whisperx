import {
  env,
  pipeline,
} from "https://cdn.jsdelivr.net/npm/@huggingface/transformers@3.8.1";

const MODEL_ID = "onnx-community/whisper-tiny.en";
const SAMPLE_RATE_HZ = 16_000;
const RUNTIME_ID = "transformers.js-webgpu-reference";

env.allowLocalModels = false;
env.useBrowserCache = true;
env.useWasmCache = true;

const elements = {
  capability: document.querySelector("#webgpu-capability"),
  fileInput: document.querySelector("#audio-file"),
  fileFacts: document.querySelector("#file-facts"),
  fileName: document.querySelector("#file-name"),
  fileSize: document.querySelector("#file-size"),
  runButton: document.querySelector("#run-button"),
  status: document.querySelector("#status-copy"),
  progress: document.querySelector("#progress"),
  transcript: document.querySelector("#transcript"),
  downloads: document.querySelector("#download-actions"),
};

let webGpuReady = false;
let selectedFile = null;
let transcriberPromise = null;
let latestContract = null;

void initialize();

elements.fileInput.addEventListener("change", () => {
  selectedFile = elements.fileInput.files?.[0] ?? null;
  latestContract = null;
  elements.downloads.hidden = true;

  if (!selectedFile) {
    elements.fileFacts.hidden = true;
    elements.transcript.textContent = "No transcript yet.";
  } else {
    elements.fileFacts.hidden = false;
    elements.fileName.textContent = selectedFile.name;
    elements.fileSize.textContent = formatBytes(selectedFile.size);
    elements.transcript.textContent = "Ready to transcribe.";
  }

  updateRunButton();
});

elements.runButton.addEventListener("click", () => void runTranscription());

elements.downloads.addEventListener("click", (event) => {
  const button = event.target.closest("button[data-format]");
  if (!button || !latestContract || !selectedFile) {
    return;
  }
  downloadProjection(button.dataset.format, latestContract, selectedFile.name);
});

async function initialize() {
  try {
    webGpuReady = await supportsWebGpu();
  } catch (error) {
    console.error(error);
    webGpuReady = false;
  }

  if (webGpuReady) {
    elements.capability.textContent = "WebGPU ready";
    elements.status.textContent = "Select an audio file to begin.";
  } else {
    elements.capability.textContent = "WebGPU unavailable";
    elements.status.textContent =
      "This browser cannot provide the required WebGPU runtime. No fallback will be used.";
  }
  updateRunButton();
}

async function supportsWebGpu() {
  if (!("gpu" in navigator)) {
    return false;
  }
  const adapter = await navigator.gpu.requestAdapter();
  return adapter !== null;
}

function updateRunButton() {
  elements.runButton.disabled = !webGpuReady || !selectedFile;
}

async function runTranscription() {
  if (!webGpuReady || !selectedFile) {
    return;
  }

  elements.runButton.disabled = true;
  elements.downloads.hidden = true;
  latestContract = null;

  try {
    setStatus("Decoding and resampling audio to 16 kHz mono…", 4);
    const audio = await decodeAndResample(selectedFile);

    setStatus("Loading Whisper tiny.en. Model files are cached by the browser…", 10);
    const transcriber = await getTranscriber();

    setStatus("Running Whisper on WebGPU…", 92);
    const output = await transcriber(audio.samples, {
      chunk_length_s: 29,
      stride_length_s: 5,
      return_timestamps: true,
      task: "transcribe",
    });

    latestContract = toNativeContract(output, selectedFile, audio.durationSeconds);
    elements.transcript.textContent = latestContract.text ?? "";
    elements.downloads.hidden = false;
    setStatus(
      `Finished locally in the browser · ${latestContract.segments.length} timed segment${latestContract.segments.length === 1 ? "" : "s"}.`,
      100,
    );
  } catch (error) {
    console.error(error);
    elements.transcript.textContent = "No transcript produced.";
    setStatus(`Transcription failed: ${formatError(error)}`);
  } finally {
    updateRunButton();
  }
}

function getTranscriber() {
  if (!transcriberPromise) {
    transcriberPromise = pipeline(
      "automatic-speech-recognition",
      MODEL_ID,
      {
        device: "webgpu",
        progress_callback: handleModelProgress,
      },
    ).catch((error) => {
      transcriberPromise = null;
      throw error;
    });
  }
  return transcriberPromise;
}

function handleModelProgress(info) {
  if (!info || typeof info !== "object") {
    return;
  }

  if (info.status === "progress") {
    const progress = normalizeProgress(info.progress, info.loaded, info.total);
    const file = typeof info.file === "string" ? ` · ${shortFileName(info.file)}` : "";
    setStatus(`Downloading/caching model assets${file}`, 10 + progress * 0.78);
    return;
  }

  if (info.status === "done") {
    setStatus("Model assets ready. Preparing WebGPU inference…", 90);
  }
}

function normalizeProgress(progress, loaded, total) {
  if (Number.isFinite(progress)) {
    const value = progress > 1 ? progress / 100 : progress;
    return clamp(value, 0, 1);
  }
  if (Number.isFinite(loaded) && Number.isFinite(total) && total > 0) {
    return clamp(loaded / total, 0, 1);
  }
  return 0;
}

async function decodeAndResample(file) {
  const arrayBuffer = await file.arrayBuffer();
  const decodeContext = new AudioContext();

  try {
    const decoded = await decodeContext.decodeAudioData(arrayBuffer.slice(0));
    const outputLength = Math.max(1, Math.ceil(decoded.duration * SAMPLE_RATE_HZ));
    const offline = new OfflineAudioContext(1, outputLength, SAMPLE_RATE_HZ);
    const source = offline.createBufferSource();
    source.buffer = decoded;
    source.connect(offline.destination);
    source.start(0);
    const rendered = await offline.startRendering();
    const samples = rendered.getChannelData(0).slice();
    return {
      samples,
      durationSeconds: samples.length / SAMPLE_RATE_HZ,
    };
  } finally {
    await decodeContext.close();
  }
}

function toNativeContract(output, file, durationSeconds) {
  const text = String(output?.text ?? "").trim();
  const rawChunks = Array.isArray(output?.chunks) ? output.chunks : [];
  const chunks = rawChunks.length > 0
    ? rawChunks
    : [{ text, timestamp: text ? [0, durationSeconds] : [null, null] }];

  const segments = chunks
    .map((chunk, index) => {
      const segmentText = String(chunk?.text ?? "").trim();
      const timestamp = Array.isArray(chunk?.timestamp) ? chunk.timestamp : [];
      return {
        index,
        startSeconds: finiteOrNull(timestamp[0]),
        endSeconds: finiteOrNull(timestamp[1]),
        text: segmentText,
        language: "en",
        speaker: null,
        confidence: null,
        isFinal: true,
        words: [],
        chars: [],
        attributes: {
          modelId: MODEL_ID,
          runtime: RUNTIME_ID,
        },
      };
    })
    .filter((segment) => segment.text.length > 0);

  return {
    text: text || segments.map((segment) => segment.text).join(" "),
    language: "en",
    segments,
    source: file.name,
    attributes: {
      acceleration: "webgpu",
      modelId: MODEL_ID,
      requiredChannels: "1",
      requiredSampleRateHz: String(SAMPLE_RATE_HZ),
      runtime: RUNTIME_ID,
    },
  };
}

function downloadProjection(format, contract, inputName) {
  const baseName = stripExtension(inputName) || "transcript";
  switch (format) {
    case "native-json":
      downloadText(
        `${baseName}.native.json`,
        `${JSON.stringify(contract, null, 2)}\n`,
        "application/json;charset=utf-8",
      );
      break;
    case "txt":
      downloadText(`${baseName}.txt`, `${contract.text ?? ""}\n`, "text/plain;charset=utf-8");
      break;
    case "srt":
      downloadText(`${baseName}.srt`, renderSrt(contract), "application/x-subrip;charset=utf-8");
      break;
    case "vtt":
      downloadText(`${baseName}.vtt`, renderVtt(contract), "text/vtt;charset=utf-8");
      break;
    default:
      throw new Error(`Unsupported download format: ${format}`);
  }
}

function renderSrt(contract) {
  return timedSegments(contract)
    .map(
      (segment, index) =>
        `${index + 1}\n${formatTimestamp(segment.startSeconds, ",")} --> ${formatTimestamp(segment.endSeconds, ",")}\n${segment.text}\n`,
    )
    .join("\n");
}

function renderVtt(contract) {
  const cues = timedSegments(contract)
    .map(
      (segment) =>
        `${formatTimestamp(segment.startSeconds, ".")} --> ${formatTimestamp(segment.endSeconds, ".")}\n${segment.text}`,
    )
    .join("\n\n");
  return `WEBVTT\n\n${cues}${cues ? "\n" : ""}`;
}

function timedSegments(contract) {
  return contract.segments.filter(
    (segment) =>
      Number.isFinite(segment.startSeconds) &&
      Number.isFinite(segment.endSeconds) &&
      segment.endSeconds >= segment.startSeconds,
  );
}

function formatTimestamp(seconds, decimalSeparator) {
  const millis = Math.max(0, Math.round(seconds * 1000));
  const hours = Math.floor(millis / 3_600_000);
  const minutes = Math.floor((millis % 3_600_000) / 60_000);
  const secs = Math.floor((millis % 60_000) / 1000);
  const ms = millis % 1000;
  return `${pad2(hours)}:${pad2(minutes)}:${pad2(secs)}${decimalSeparator}${String(ms).padStart(3, "0")}`;
}

function downloadText(fileName, content, type) {
  const blob = new Blob([content], { type });
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = fileName;
  document.body.append(link);
  link.click();
  link.remove();
  URL.revokeObjectURL(url);
}

function setStatus(message, progress = null) {
  elements.status.textContent = message;
  if (Number.isFinite(progress)) {
    elements.progress.hidden = false;
    elements.progress.value = clamp(progress, 0, 100);
  } else {
    elements.progress.hidden = true;
    elements.progress.value = 0;
  }
}

function formatBytes(bytes) {
  if (!Number.isFinite(bytes) || bytes < 1024) {
    return `${bytes ?? 0} B`;
  }
  const units = ["KB", "MB", "GB"];
  let value = bytes / 1024;
  let unit = units[0];
  for (let index = 1; index < units.length && value >= 1024; index += 1) {
    value /= 1024;
    unit = units[index];
  }
  return `${value.toFixed(value >= 10 ? 1 : 2)} ${unit}`;
}

function formatError(error) {
  if (error instanceof Error && error.message) {
    return error.message;
  }
  return String(error);
}

function shortFileName(value) {
  return value.split("/").pop() || value;
}

function finiteOrNull(value) {
  return Number.isFinite(value) ? value : null;
}

function stripExtension(value) {
  return value.replace(/\.[^.]+$/, "");
}

function pad2(value) {
  return String(value).padStart(2, "0");
}

function clamp(value, min, max) {
  return Math.min(max, Math.max(min, value));
}
