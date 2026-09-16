import {
  browserTranscriptionCapabilities,
  supportsBrowserTranscription,
  transcribeAudioBlob,
} from "./vendor/audio-analysis-transcription.js";
import {
  browserTranslationCapabilities,
  resolveBrowserTranslationPair,
  supportsBrowserTranslation,
  translateBrowserSegments,
} from "./vendor/platform-browser-translation.js";

const BROWSER_RUN_EVIDENCE_KEY = "__nativeWhisperxBrowserRunEvidence";

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
  browserTranslate: document.querySelector("#browser-translate"),
  browserTranslationOptions: document.querySelector("#browser-translation-options"),
  browserTranslationModel: document.querySelector("#browser-translation-model"),
  browserTranslationSource: document.querySelector("#browser-translation-source"),
  browserTranslationTarget: document.querySelector("#browser-translation-target"),
  browserTranslationStatus: document.querySelector("#browser-translation-status"),
  transcriptTitle: document.querySelector("#transcript-title"),
  transcript: document.querySelector("#transcript"),
  sourceTranscriptWrap: document.querySelector("#source-transcript-wrap"),
  sourceTranscript: document.querySelector("#source-transcript"),
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
const translationCapabilities = browserTranslationCapabilities();
let webGpuReady = false;
let translationReady = false;
let selectedFile = null;
let previewUrl = null;
let latestContract = null;
let latestSourceContract = null;
let cancelRequested = false;
let browserRunActive = false;

clearCompletedBrowserRunEvidence();
void initialize();
wireEvents();
updateBrowserTranslationVisibility();
updateNativeCommand();

async function initialize() {
  const [transcriptionResult, translationResult] = await Promise.allSettled([
    supportsBrowserTranscription(),
    supportsBrowserTranslation(),
  ]);
  webGpuReady = transcriptionResult.status === "fulfilled" && transcriptionResult.value === true;
  translationReady = translationResult.status === "fulfilled" && translationResult.value === true;

  if (transcriptionResult.status === "rejected") console.error(transcriptionResult.reason);
  if (translationResult.status === "rejected") console.error(translationResult.reason);

  if (webGpuReady) {
    elements.webGpuDot.classList.add("ready");
    elements.webGpuCapability.textContent = "WebGPU ready";
    elements.webGpuDetail.textContent = `${browserCapabilities.modelId} via ${browserCapabilities.runtime}; optional post-ASR translation uses ${translationCapabilities.runtime}. Browser cache reuse is enabled by both upstream providers.`;
    setBrowserStatus("Choose an audio file to run local browser transcription.");
  } else {
    elements.webGpuDot.classList.add("unavailable");
    elements.webGpuCapability.textContent = "WebGPU unavailable";
    elements.webGpuDetail.textContent = "The browser transcription capability is disabled. The native workflow composer remains available.";
    setBrowserStatus("audio-analysis requires WebGPU for browser transcription. No server or CPU fallback will be used.");
  }
  updateBrowserTranslationPair();
  if (!translationReady) {
    setTranslationStatus("Browser translation is unavailable because its WebGPU requirement is not satisfied.");
  }
  updateBrowserButton();
}

function wireEvents() {
  elements.chooseFile.addEventListener("click", () => {
    if (!browserRunActive) elements.fileInput.click();
  });
  elements.fileInput.addEventListener("change", () => selectFile(elements.fileInput.files?.[0] ?? null));

  for (const eventName of ["dragenter", "dragover"]) {
    elements.dropZone.addEventListener(eventName, (event) => {
      event.preventDefault();
      if (!browserRunActive) elements.dropZone.classList.add("is-dragging");
    });
  }
  for (const eventName of ["dragleave", "drop"]) {
    elements.dropZone.addEventListener(eventName, (event) => {
      event.preventDefault();
      elements.dropZone.classList.remove("is-dragging");
    });
  }
  elements.dropZone.addEventListener("drop", (event) => {
    if (browserRunActive) return;
    const file = event.dataTransfer?.files?.[0] ?? null;
    if (file) selectFile(file);
  });

  elements.runBrowser.addEventListener("click", () => void runBrowserPreview());
  elements.cancelBrowser.addEventListener("click", () => {
    cancelRequested = true;
    elements.cancelBrowser.disabled = true;
    setBrowserStatus("Cancellation requested. The current upstream browser step will finish before stopping.");
  });
  elements.browserTranslate.addEventListener("change", () => {
    updateBrowserTranslationVisibility();
    updateBrowserButton();
  });
  for (const control of [elements.browserTranslationSource, elements.browserTranslationTarget]) {
    control.addEventListener("change", () => {
      updateBrowserTranslationPair();
      updateBrowserButton();
    });
  }
  elements.downloads.addEventListener("click", (event) => {
    const button = event.target.closest("button[data-format]");
    if (!button || !latestContract || !selectedFile) return;
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
  if (browserRunActive) return;
  selectedFile = file;
  latestContract = null;
  latestSourceContract = null;
  clearCompletedBrowserRunEvidence();
  elements.downloads.hidden = true;
  elements.segmentTableWrap.hidden = true;
  elements.segmentRows.replaceChildren();
  elements.sourceTranscriptWrap.hidden = true;
  elements.sourceTranscript.textContent = "";
  elements.transcriptTitle.textContent = "Transcript";

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

function updateBrowserTranslationVisibility() {
  elements.browserTranslationOptions.hidden = !elements.browserTranslate.checked;
  updateBrowserTranslationPair();
  if (!elements.browserTranslate.checked) {
    setTranslationStatus("Translation is off. Browser ASR will return the source-language transcript.");
  }
}

function updateBrowserTranslationPair() {
  try {
    const pair = currentBrowserTranslationPair();
    elements.browserTranslationModel.value = pair.modelId;
    if (elements.browserTranslate.checked && translationReady) {
      setTranslationStatus(`${pair.sourceLanguage} → ${pair.targetLanguage} will run locally with ${pair.modelId}.`);
    }
    return true;
  } catch (error) {
    elements.browserTranslationModel.value = "";
    if (elements.browserTranslate.checked) {
      setTranslationStatus(`Translation configuration is invalid: ${formatError(error)}`);
    }
    return false;
  }
}

function currentBrowserTranslationPair() {
  return resolveBrowserTranslationPair(
    elements.browserTranslationSource.value,
    elements.browserTranslationTarget.value,
  );
}

function browserRunConfiguration() {
  const translationRequested = elements.browserTranslate.checked;
  const translationPair = translationRequested ? currentBrowserTranslationPair() : null;
  return Object.freeze({
    translationRequested,
    translationPair: translationPair ? Object.freeze({ ...translationPair }) : null,
  });
}

function setBrowserRunControlsDisabled(disabled) {
  browserRunActive = disabled;
  elements.chooseFile.disabled = disabled;
  elements.fileInput.disabled = disabled;
  elements.browserTranslate.disabled = disabled;
  elements.browserTranslationSource.disabled = disabled;
  elements.browserTranslationTarget.disabled = disabled;
  elements.dropZone.setAttribute("aria-disabled", disabled ? "true" : "false");
}

function updateBrowserButton() {
  const translationUnavailable =
    elements.browserTranslate.checked && (!translationReady || !updateBrowserTranslationPair());
  elements.runBrowser.disabled = browserRunActive || !webGpuReady || !selectedFile || translationUnavailable;
}

async function runBrowserPreview() {
  if (browserRunActive || !webGpuReady || !selectedFile) return;

  let runConfig;
  try {
    runConfig = browserRunConfiguration();
  } catch (error) {
    setTranslationStatus(`Translation configuration is invalid: ${formatError(error)}`);
    updateBrowserButton();
    return;
  }
  if (runConfig.translationRequested && !translationReady) return;

  const runFile = selectedFile;
  cancelRequested = false;
  latestContract = null;
  latestSourceContract = null;
  clearCompletedBrowserRunEvidence();
  setBrowserRunControlsDisabled(true);
  elements.runBrowser.disabled = true;
  elements.cancelBrowser.disabled = false;
  elements.downloads.hidden = true;
  elements.segmentTableWrap.hidden = true;
  elements.sourceTranscriptWrap.hidden = true;
  elements.transcriptTitle.textContent = "Transcript";

  try {
    setBrowserStatus("Handing local audio to the audio-analysis browser transcription provider…", 2);
    const result = await transcribeAudioBlob(runFile, {
      source: runFile.name,
      onProgress: handleBrowserProgress,
    });
    throwIfCancelled();

    latestSourceContract = toNativeContract(result, runFile);
    latestContract = latestSourceContract;

    if (runConfig.translationRequested) {
      const pair = runConfig.translationPair;
      if (!pair) throw new Error("Browser translation pair was not captured for this run.");
      if (latestSourceContract.language && latestSourceContract.language !== pair.sourceLanguage) {
        throw new Error(
          `Browser ASR reported ${latestSourceContract.language}, but translation is configured for ${pair.sourceLanguage} → ${pair.targetLanguage}.`,
        );
      }
      setTranslationStatus("Transcription complete. Starting local post-ASR translation…");
      const units = translationUnits(latestSourceContract);
      const translated = await translateBrowserSegments(units, {
        modelId: pair.modelId,
        sourceLanguage: pair.sourceLanguage,
        targetLanguage: pair.targetLanguage,
        maxNewTokens: 256,
        onProgress: handleBrowserTranslationProgress,
      });
      throwIfCancelled();
      latestContract = applyBrowserTranslation(latestSourceContract, translated);
      elements.transcriptTitle.textContent = "Translated transcript";
      elements.sourceTranscriptWrap.hidden = false;
      elements.sourceTranscript.textContent = latestSourceContract.text || "No speech detected.";
      setTranslationStatus(
        `Translated locally · ${translated.sourceLanguage} → ${translated.targetLanguage} · ${translated.attributes.modelId}.`,
      );
    } else {
      setTranslationStatus("Translation was not requested for this run.");
    }

    renderBrowserResult(latestContract);
    elements.downloads.hidden = false;
    const timedCount = timedSegments(latestContract).length;
    const suffix = runConfig.translationRequested ? " · post-ASR translation complete" : "";
    const finalStatus = `Finished locally · ${timedCount} timed segment${timedCount === 1 ? "" : "s"}${suffix}.`;
    setBrowserStatus(finalStatus, 100);
    publishCompletedBrowserRunEvidence(runConfig, runFile, latestSourceContract, latestContract, finalStatus);
  } catch (error) {
    clearCompletedBrowserRunEvidence();
    if (error instanceof BrowserCancellationError) {
      elements.transcript.textContent = "Browser workflow cancelled.";
      setBrowserStatus("Browser workflow cancelled.");
      setTranslationStatus("Translation did not publish a partial completed result.");
    } else {
      console.error(error);
      elements.transcript.textContent = "No browser result produced.";
      setBrowserStatus(`Browser workflow failed: ${formatError(error)}`);
      if (runConfig.translationRequested) setTranslationStatus(`Translation failed: ${formatError(error)}`);
    }
  } finally {
    elements.cancelBrowser.disabled = true;
    setBrowserRunControlsDisabled(false);
    updateBrowserTranslationPair();
    updateBrowserButton();
  }
}

function publishCompletedBrowserRunEvidence(runConfig, file, sourceContract, contract, browserStatus) {
  const translationRequested = runConfig.translationRequested;
  const translationPair = runConfig.translationPair;
  const availableFormats = Array.from(
    elements.downloads.querySelectorAll("button[data-format]"),
    (button) => button.dataset.format,
  )
    .filter(Boolean)
    .sort();
  const sourceTranscriptLength = translationRequested ? sourceContract.text.length : 0;
  const translationCompleted =
    !translationRequested || contract.attributes.translation === "browser-post-asr";
  const sourceTranscriptPreserved = !translationRequested || sourceTranscriptLength > 0;

  window[BROWSER_RUN_EVIDENCE_KEY] = Object.freeze({
    schemaVersion: 1,
    completedAt: new Date().toISOString(),
    webGpuCapability: elements.webGpuCapability.textContent?.trim() ?? "",
    browserStatus,
    translationRequested,
    translationCompleted,
    translationStatus: elements.browserTranslationStatus.textContent?.trim() ?? "",
    translationModel: translationPair?.modelId ?? null,
    translationSourceLanguage: translationPair?.sourceLanguage ?? null,
    translationTargetLanguage: translationPair?.targetLanguage ?? null,
    detectedSourceLanguage: sourceContract.language ?? null,
    translationRuntime: translationRequested ? contract.attributes.translationRuntime ?? null : null,
    fileName: file.name,
    fileSizeBytes: file.size,
    fileSizeLabel: formatBytes(file.size),
    transcriptLength: contract.text.length,
    sourceTranscriptLength,
    sourceTranscriptPreserved,
    segmentCount: contract.segments.length,
    timedSegmentCount: timedSegments(contract).length,
    projectionsAvailable: !elements.downloads.hidden,
    availableFormats,
  });
}

function clearCompletedBrowserRunEvidence() {
  delete window[BROWSER_RUN_EVIDENCE_KEY];
}

function handleBrowserProgress(update) {
  throwIfCancelled();
  if (!update || typeof update !== "object") return;
  const message = typeof update.message === "string" ? update.message : "Running browser transcription…";
  setBrowserStatus(message, browserProgressValue(update));
}

function handleBrowserTranslationProgress(update) {
  throwIfCancelled();
  if (!update || typeof update !== "object") return;
  const message = typeof update.message === "string" ? update.message : "Running browser translation…";
  setTranslationStatus(message);
  setBrowserStatus(message, translationProgressValue(update));
}

function browserProgressValue(update) {
  if (update.stage === "decode") return 5;
  if (update.stage === "transcribe") return 74;
  if (update.stage !== "model") return null;
  const detail = update.detail;
  if (!detail || typeof detail !== "object") return 12;
  if (detail.status === "done" || detail.status === "ready") return 70;
  const normalized = normalizeProgress(detail.progress, detail.loaded, detail.total);
  return 12 + normalized * 0.56;
}

function translationProgressValue(update) {
  if (update.stage === "translate") return update.detail?.status === "done" ? 99 : 94;
  if (update.stage !== "model") return null;
  if (update.detail?.status === "ready" || update.detail?.status === "cached") return 92;
  const normalized = normalizeProgress(update.detail?.progress, update.detail?.loaded, update.detail?.total);
  return 78 + normalized * 0.12;
}

function toNativeContract(result, file) {
  const segments = Array.isArray(result?.segments)
    ? result.segments
        .map((segment, index) => ({
          ...segment,
          index: Number.isInteger(segment?.index) ? segment.index : index,
          text: String(segment?.text ?? "").trim(),
          words: Array.isArray(segment?.words) ? segment.words : [],
          chars: Array.isArray(segment?.chars) ? segment.chars : [],
          attributes: { ...(segment?.attributes ?? {}) },
        }))
        .filter((segment) => segment.text.length > 0)
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
      translation: "not-requested-in-browser-preview",
    },
  };
}

function translationUnits(contract) {
  if (contract.segments.length > 0) {
    return contract.segments.map((segment) => ({ id: segment.index, text: segment.text }));
  }
  if (contract.text) return [{ id: "document", text: contract.text }];
  return [];
}

function applyBrowserTranslation(sourceContract, translated) {
  const translatedById = new Map(translated.segments.map((segment) => [String(segment.id), segment.text]));
  const segments = sourceContract.segments.map((segment) => {
    const text = translatedById.get(String(segment.index));
    if (!text) throw new Error(`Browser translation omitted source segment ${segment.index}.`);
    return {
      ...segment,
      text,
      words: [],
      chars: [],
      attributes: {
        ...segment.attributes,
        translation: "browser-post-asr",
        sourceWordAlignment: "not-projected-onto-translated-text",
      },
    };
  });
  const documentTranslation = translatedById.get("document");
  return {
    ...sourceContract,
    text: segments.length > 0 ? segments.map((segment) => segment.text).join(" ").trim() : (documentTranslation ?? ""),
    language: translated.targetLanguage,
    segments,
    attributes: {
      ...sourceContract.attributes,
      translation: "browser-post-asr",
      translationRuntime: translated.attributes.runtime,
      translationModel: translated.attributes.modelId,
      translationAcceleration: translated.attributes.acceleration,
      translationModelProvisioning: translated.attributes.modelProvisioning,
      translationSourceLanguage: translated.sourceLanguage,
      translationTargetLanguage: translated.targetLanguage,
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
  if (elements.nativeLanguage.value) pushOption(args, "--language", elements.nativeLanguage.value);

  if (!elements.nativeAlign.checked) args.push("--no-align");
  else if (elements.nativeCharAlign.checked) args.push("--return-char-alignments");

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
  for (const format of formats.length > 0 ? formats : ["json"]) args.push("--format", format);

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
  if (Number.isInteger(value) && value > 0) args.push(flag, String(value));
}

function shellQuote(value) {
  const text = String(value);
  if (/^[A-Za-z0-9_./:@+-]+$/.test(text)) return text;
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
  return contract.segments.filter(
    (segment) => Number.isFinite(segment.startSeconds) && Number.isFinite(segment.endSeconds) && segment.endSeconds >= segment.startSeconds,
  );
}

function segmentTime(segment) {
  if (!Number.isFinite(segment.startSeconds) || !Number.isFinite(segment.endSeconds)) return "untimed";
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

function setTranslationStatus(message) {
  elements.browserTranslationStatus.textContent = message;
}

function throwIfCancelled() {
  if (cancelRequested) throw new BrowserCancellationError();
}

class BrowserCancellationError extends Error {}

function normalizeProgress(progress, loaded, total) {
  if (Number.isFinite(progress)) {
    const value = progress > 1 ? progress / 100 : progress;
    return clamp(value, 0, 1);
  }
  if (Number.isFinite(loaded) && Number.isFinite(total) && total > 0) return clamp(loaded / total, 0, 1);
  return 0;
}

function formatBytes(bytes) {
  if (!Number.isFinite(bytes) || bytes < 1024) return `${bytes ?? 0} B`;
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
