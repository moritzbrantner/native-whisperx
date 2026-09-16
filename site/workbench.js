import {
  browserTranscriptionCapabilities,
  supportsBrowserTranscription,
  transcribeAudioBlob,
} from "./vendor/audio-analysis-transcription.js";

const elements = {
  webGpuDot: document.querySelector("#webgpu-dot"),
  webGpuCapability: document.querySelector("#webgpu-capability"),
  webGpuDetail: document.querySelector("#webgpu-detail"),
  dropZone: document.querySelector("#drop-zone"),
  fileInput: document.querySelector("#audio-file"),
  chooseFile: document.querySelector("#choose-file"),
  selectedFile: document.querySelector("#selected-file"),
  fileName: document.querySelector("#file-name"),
  fileSize: document.querySelector("#file-size"),
  audioPreview: document.querySelector("#audio-preview"),
  runBrowser: document.querySelector("#run-browser"),
  cancelBrowser: document.querySelector("#cancel-browser"),
  browserStatus: document.querySelector("#browser-status"),
  browserProgress: document.querySelector("#browser-progress"),
  transcript: document.querySelector("#transcript"),
  segmentTableWrap: document.querySelector("#segment-table-wrap"),
  segmentRows: document.querySelector("#segment-rows"),
  downloads: document.querySelector("#download-actions"),
  nativeInput: document.querySelector("#native-input"),
  nativeModel: document.querySelector("#native-model"),
  nativeLanguage: document.querySelector("#native-language"),
  nativeDevice: document.querySelector("#native-device"),
  nativeAlign: document.querySelector("#native-align"),
  nativeCharAlign: document.querySelector("#native-char-align"),
  charAlignmentRow: document.querySelector("#char-alignment-row"),
  nativeDiarize: document.querySelector("#native-diarize"),
  diarizationOptions: document.querySelector("#diarization-options"),
  minSpeakers: document.querySelector("#min-speakers"),
  maxSpeakers: document.querySelector("#max-speakers"),
  nativeTranslate: document.querySelector("#native-translate"),
  translationOptions: document.querySelector("#translation-options"),
  translationModel: document.querySelector("#translation-model"),
  translationSource: document.querySelector("#translation-source"),
  translationTarget: document.querySelector("#translation-target"),
  nativeCommand: document.querySelector("#native-command"),
  copyCommand: document.querySelector("#copy-command"),
  copyStatus: document.querySelector("#copy-status"),
  summaryAlign: document.querySelector("#summary-align"),
  summaryDiarize: document.querySelector("#summary-diarize"),
  summaryTranslate: document.querySelector("#summary-translate"),
};

const browserCapabilities = browserTranscriptionCapabilities();
let webGpuReady = false;
let selectedFile = null;
let previewUrl = null;
let latestContract = null;
let cancelRequested = false;

void initialize();
wireEvents();
updateNativeCommand();

async function initialize() {
  try {
    webGpuReady = await supportsBrowserTranscription();
  } catch (error) {
    console.error(error);
    webGpuReady = false;
  }

  if (webGpuReady) {
    elements.webGpuDot.classList.add("ready");
    elements.webGpuCapability.textContent = "WebGPU ready";
    elements.webGpuDetail.textContent = `${browserCapabilities.modelId} via ${browserCapabilities.runtime}; browser cache reuse is enabled upstream.`;
    setBrowserStatus("Choose an audio file to run the browser transcription capability.");
  } else {
    elements.webGpuDot.classList.add("unavailable");
    elements.webGpuCapability.textContent = "WebGPU unavailable";
    elements.webGpuDetail.textContent = "The browser transcription capability is disabled. The native workflow composer remains available.";
    setBrowserStatus("audio-analysis requires WebGPU for browser transcription. No server or CPU fallback will be used.");
  }
  updateBrowserButton();
}

function wireEvents() {
  elements.chooseFile.addEventListener("click", () => elements.fileInput.click());
  elements.fileInput.addEventListener("change", () => selectFile(elements.fileInput.files?.[0] ?? null));

  for (const eventName of ["dragenter", "dragover"]) {
    elements.dropZone.addEventListener(eventName, (event) => {
      event.preventDefault();
      elements.dropZone.classList.add("is-dragging");
    });
  }
  for (const eventName of ["dragleave", "drop"]) {
    elements.dropZone.addEventListener(eventName, (event) => {
      event.preventDefault();
      elements.dropZone.classList.remove("is-dragging");
    });
  }
  elements.dropZone.addEventListener("drop", (event) => {
    const file = event.dataTransfer?.files?.[0] ?? null;
    if (file) {
      selectFile(file);
    }
  });

  elements.runBrowser.addEventListener("click", () => void runBrowserPreview());
  elements.cancelBrowser.addEventListener("click", () => {
    cancelRequested = true;
    elements.cancelBrowser.disabled = true;
    setBrowserStatus("Cancellation requested. The current upstream browser step will finish before stopping.");
  });
  elements.downloads.addEventListener("click", (event) => {
    const button = event.target.closest("button[data-format]");
    if (!button || !latestContract || !selectedFile) {
      return;
    }
    downloadProjection(button.dataset.format, latestContract, selectedFile.name);
  });

  for (const control of document.querySelectorAll("#native-workflow input, #native-workflow select")) {
    control.addEventListener("input", updateNativeCommand);
    control.addEventListener("change", updateNativeCommand);
  }
  elements.nativeAlign.addEventListener("change", updateNativeVisibility);
  elements.nativeDiarize.addEventListener("change", updateNativeVisibility);
  elements.nativeTranslate.addEventListener("change", updateNativeVisibility);
  elements.copyCommand.addEventListener("click", () => void copyNativeCommand());
}

function selectFile(file) {
  selectedFile = file;
  latestContract = null;
  elements.downloads.hidden = true;
  elements.segmentTableWrap.hidden = true;
  elements.segmentRows.replaceChildren();

  if (previewUrl) {
    URL.revokeObjectURL(previewUrl);
    previewUrl = null;
  }

  if (!file) {
    elements.selectedFile.hidden = true;
    elements.audioPreview.removeAttribute("src");
    elements.transcript.textContent = "No browser result yet.";
    updateBrowserButton();
    return;
  }

  previewUrl = URL.createObjectURL(file);
  elements.selectedFile.hidden = false;
  elements.fileName.textContent = file.name;
  elements.fileSize.textContent = formatBytes(file.size);
  elements.audioPreview.src = previewUrl;
  elements.nativeInput.value = file.name;
  elements.transcript.textContent = "Ready for upstream browser transcription.";
  updateBrowserButton();
  updateNativeCommand();
}

function updateBrowserButton() {
  elements.runBrowser.disabled = !webGpuReady || !selectedFile;
}

async function runBrowserPreview() {
  if (!webGpuReady || !selectedFile) {
    return;
  }

  cancelRequested = false;
  latestContract = null;
  elements.runBrowser.disabled = true;
  elements.cancelBrowser.disabled = false;
  elements.downloads.hidden = true;
  elements.segmentTableWrap.hidden = true;

  try {
    setBrowserStatus("Handing local audio to the audio-analysis browser transcription provider…", 2);
    const result = await transcribeAudioBlob(selectedFile, {
      source: selectedFile.name,
      onProgress: handleBrowserProgress,
    });
    throwIfCancelled();

    latestContract = toNativeContract(result, selectedFile);
    renderBrowserResult(latestContract);
    elements.downloads.hidden = false;
    setBrowserStatus(
      `Finished locally · ${latestContract.segments.length} timed segment${latestContract.segments.length === 1 ? "" : "s"}.`,
      100,
    );
  } catch (error) {
    if (error instanceof BrowserCancellationError) {
      elements.transcript.textContent = "Browser transcription cancelled.";
      setBrowserStatus("Browser transcription cancelled.");
    } else {
      console.error(error);
      elements.transcript.textContent = "No browser result produced.";
      setBrowserStatus(`Browser transcription failed: ${formatError(error)}`);
    }
  } finally {
    elements.cancelBrowser.disabled = true;
    updateBrowserButton();
  }
}

function handleBrowserProgress(update) {
  throwIfCancelled();
  if (!update || typeof update !== "object") {
    return;
  }
  const message = typeof update.message === "string" ? update.message : "Running browser transcription…";
  setBrowserStatus(message, browserProgressValue(update));
}

function browserProgressValue(update) {
  if (update.stage === "decode") {
    return 5;
  }
  if (update.stage === "transcribe") {
    return 92;
  }
  if (update.stage !== "model") {
    return null;
  }

  const detail = update.detail;
  if (!detail || typeof detail !== "object") {
    return 12;
  }
  if (detail.status === "done" || detail.status === "ready") {
    return 90;
  }

  const normalized = normalizeProgress(detail.progress, detail.loaded, detail.total);
  return 12 + normalized * 0.76;
}

function toNativeContract(result, file) {
  const segments = Array.isArray(result?.segments)
    ? result.segments.map((segment, index) => ({
        ...segment,
        index: Number.isInteger(segment?.index) ? segment.index : index,
        text: String(segment?.text ?? "").trim(),
        words: Array.isArray(segment?.words) ? segment.words : [],
        chars: Array.isArray(segment?.chars) ? segment.chars : [],
        attributes: { ...(segment?.attributes ?? {}) },
      })).filter((segment) => segment.text.length > 0)
    : [];

  return {
    text: String(result?.text ?? segments.map((segment) => segment.text).join(" ")).trim(),
    language: result?.language ?? null,
    segments,
    source: file.name,
    attributes: {
      ...(result?.attributes ?? {}),
      alignment: "not-run-in-browser-preview",
      diarization: "not-run-in-browser-preview",
      translation: "not-run-in-browser-preview",
    },
  };
}

function renderBrowserResult(contract) {
  elements.transcript.textContent = contract.text || "No speech detected.";
  elements.segmentRows.replaceChildren();
  for (const segment of contract.segments) {
    const row = document.createElement("tr");
    const time = document.createElement("td");
    time.textContent = segmentTime(segment);
    const text = document.createElement("td");
    text.textContent = segment.text;
    row.append(time, text);
    elements.segmentRows.append(row);
  }
  elements.segmentTableWrap.hidden = contract.segments.length === 0;
}

function updateNativeVisibility() {
  elements.charAlignmentRow.hidden = !elements.nativeAlign.checked;
  elements.nativeCharAlign.disabled = !elements.nativeAlign.checked;
  elements.diarizationOptions.hidden = !elements.nativeDiarize.checked;
  elements.translationOptions.hidden = !elements.nativeTranslate.checked;
  updateNativeCommand();
}

function updateNativeCommand() {
  updateNativeVisibilityOnly();
  const args = ["native-whisperx", "transcribe", shellQuote(elements.nativeInput.value.trim() || "input.wav")];
  pushOption(args, "--model", elements.nativeModel.value.trim() || "small");
  pushOption(args, "--device", elements.nativeDevice.value);
  if (elements.nativeLanguage.value) {
    pushOption(args, "--language", elements.nativeLanguage.value);
  }

  if (!elements.nativeAlign.checked) {
    args.push("--no-align");
  } else if (elements.nativeCharAlign.checked) {
    args.push("--return-char-alignments");
  }

  if (elements.nativeDiarize.checked) {
    args.push("--diarize");
    pushNumberOption(args, "--min-speakers", elements.minSpeakers.value);
    pushNumberOption(args, "--max-speakers", elements.maxSpeakers.value);
  }

  if (elements.nativeTranslate.checked) {
    args.push("--task", "translate");
    pushOption(args, "--translation-model", elements.translationModel.value.trim() || "Helsinki-NLP/opus-mt-de-en");
    pushOption(args, "--translation-source-language", elements.translationSource.value.trim() || "de");
    pushOption(args, "--translation-target-language", elements.translationTarget.value.trim() || "en");
  }

  const formats = [...document.querySelectorAll('input[name="native-format"]:checked')].map((input) => input.value);
  for (const format of formats.length > 0 ? formats : ["json"]) {
    args.push("--format", format);
  }

  elements.nativeCommand.textContent = wrapCommand(args);
  elements.summaryAlign.textContent = elements.nativeAlign.checked ? "Enabled" : "Off";
  elements.summaryDiarize.textContent = elements.nativeDiarize.checked ? "Enabled" : "Off";
  elements.summaryTranslate.textContent = elements.nativeTranslate.checked
    ? `${elements.translationSource.value.trim() || "de"} → ${elements.translationTarget.value.trim() || "en"}`
    : "Off";
}

function updateNativeVisibilityOnly() {
  elements.charAlignmentRow.hidden = !elements.nativeAlign.checked;
  elements.nativeCharAlign.disabled = !elements.nativeAlign.checked;
  elements.diarizationOptions.hidden = !elements.nativeDiarize.checked;
  elements.translationOptions.hidden = !elements.nativeTranslate.checked;
}

async function copyNativeCommand() {
  const text = elements.nativeCommand.textContent.replace(/ \\\n  /g, " ");
  try {
    await navigator.clipboard.writeText(text);
    elements.copyStatus.textContent = "Command copied.";
  } catch (error) {
    console.error(error);
    elements.copyStatus.textContent = "Could not access the clipboard. Select and copy the command manually.";
  }
}

function wrapCommand(args) {
  return args.map((arg, index) => (index === 0 ? arg : `\\\n  ${arg}`)).join(" ");
}

function pushOption(args, flag, value) {
  args.push(flag, shellQuote(value));
}

function pushNumberOption(args, flag, rawValue) {
  const value = Number.parseInt(rawValue, 10);
  if (Number.isInteger(value) && value > 0) {
    args.push(flag, String(value));
  }
}

function shellQuote(value) {
  const text = String(value);
  if (/^[A-Za-z0-9_./:@+-]+$/.test(text)) {
    return text;
  }
  return `'${text.replaceAll("'", `'"'"'`)}'`;
}

function downloadProjection(format, contract, inputName) {
  const baseName = stripExtension(inputName) || "transcript";
  if (format === "native-json") {
    downloadText(`${baseName}.native.json`, `${JSON.stringify(contract, null, 2)}\n`, "application/json;charset=utf-8");
  } else if (format === "txt") {
    downloadText(`${baseName}.txt`, `${contract.text ?? ""}\n`, "text/plain;charset=utf-8");
  } else if (format === "srt") {
    downloadText(`${baseName}.srt`, renderSrt(contract), "application/x-subrip;charset=utf-8");
  } else if (format === "vtt") {
    downloadText(`${baseName}.vtt`, renderVtt(contract), "text/vtt;charset=utf-8");
  }
}

function renderSrt(contract) {
  return timedSegments(contract)
    .map((segment, index) => `${index + 1}\n${formatTimestamp(segment.startSeconds, ",")} --> ${formatTimestamp(segment.endSeconds, ",")}\n${segment.text}\n`)
    .join("\n");
}

function renderVtt(contract) {
  const cues = timedSegments(contract)
    .map((segment) => `${formatTimestamp(segment.startSeconds, ".")} --> ${formatTimestamp(segment.endSeconds, ".")}\n${segment.text}`)
    .join("\n\n");
  return `WEBVTT\n\n${cues}${cues ? "\n" : ""}`;
}

function timedSegments(contract) {
  return contract.segments.filter((segment) => Number.isFinite(segment.startSeconds) && Number.isFinite(segment.endSeconds) && segment.endSeconds >= segment.startSeconds);
}

function segmentTime(segment) {
  if (!Number.isFinite(segment.startSeconds) || !Number.isFinite(segment.endSeconds)) {
    return "untimed";
  }
  return `${shortSeconds(segment.startSeconds)} – ${shortSeconds(segment.endSeconds)}`;
}

function shortSeconds(value) {
  const minutes = Math.floor(value / 60);
  const seconds = value - minutes * 60;
  return `${String(minutes).padStart(2, "0")}:${seconds.toFixed(1).padStart(4, "0")}`;
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

function setBrowserStatus(message, progress = null) {
  elements.browserStatus.textContent = message;
  if (Number.isFinite(progress)) {
    elements.browserProgress.hidden = false;
    elements.browserProgress.value = clamp(progress, 0, 100);
  } else {
    elements.browserProgress.hidden = true;
    elements.browserProgress.value = 0;
  }
}

function throwIfCancelled() {
  if (cancelRequested) {
    throw new BrowserCancellationError();
  }
}

class BrowserCancellationError extends Error {}

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
  return error instanceof Error && error.message ? error.message : String(error);
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