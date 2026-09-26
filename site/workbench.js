import {
  browserTranscriptionCapabilities,
  browserTranscriptionModels,
  decodeBrowserAudioBlob,
  supportsBrowserTranscription,
  transcribeAudioSamples,
} from "./vendor/audio-analysis-transcription.js";
import {
  assignBrowserDiarizationToTranscript,
  browserDiarizationCapabilities,
  diarizeBrowserAudioSamples,
  supportsBrowserDiarization,
} from "./vendor/audio-analysis-speakers.js";
import {
  browserTranslationCapabilities,
  supportsBrowserTranslation,
  translateBrowserSegments,
} from "./vendor/browser-translation.js";

const elements = {
  webGpuDot: document.querySelector("#webgpu-dot"),
  webGpuCapability: document.querySelector("#webgpu-capability"),
  webGpuDetail: document.querySelector("#webgpu-detail"),
  diarizationDot: document.querySelector("#diarization-dot"),
  diarizationCapability: document.querySelector("#diarization-capability"),
  diarizationDetail: document.querySelector("#diarization-detail"),
  translationDot: document.querySelector("#translation-dot"),
  translationCapability: document.querySelector("#translation-capability"),
  translationDetail: document.querySelector("#translation-detail"),
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
  browserModel: document.querySelector("#browser-model"),
  browserModelDescription: document.querySelector("#browser-model-description"),
  browserTranscriptionStage: document.querySelector("#browser-transcription-stage"),
  transcript: document.querySelector("#transcript"),
  segmentTableWrap: document.querySelector("#segment-table-wrap"),
  segmentRows: document.querySelector("#segment-rows"),
  downloads: document.querySelector("#download-actions"),
  browserDiarize: document.querySelector("#browser-diarize"),
  browserDiarizationStage: document.querySelector("#browser-diarization-stage"),
  browserTranslate: document.querySelector("#browser-translate"),
  browserTranslationOptions: document.querySelector("#browser-translation-options"),
  browserTranslationPair: document.querySelector("#browser-translation-pair"),
  browserTranslationStage: document.querySelector("#browser-translation-stage"),
  sourceTranscript: document.querySelector("#source-transcript"),
  sourceTranscriptText: document.querySelector("#source-transcript-text"),
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
  hfToken: document.querySelector("#hf-token"),
  clearHfToken: document.querySelector("#clear-hf-token"),
  hfTokenStatus: document.querySelector("#hf-token-status"),
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

const HF_TOKEN_STORAGE_KEY = "native-whisperx:hf-token";
const browserCapabilities = browserTranscriptionCapabilities();
const browserModels = browserTranscriptionModels();
const diarizationCapabilities = browserDiarizationCapabilities();
const translationCapabilities = browserTranslationCapabilities();
let webGpuReady = false;
let diarizationReady = false;
let translationReady = false;
let selectedFile = null;
let previewUrl = null;
let latestContract = null;
let activeBrowserRun = null;
let browserRunSequence = 0;

document.documentElement.dataset.diarizationRequested = "false";
document.documentElement.dataset.diarizationCompleted = "false";
document.documentElement.dataset.diarizationSpeakerCount = "0";
document.documentElement.dataset.translationRequested = "false";
document.documentElement.dataset.translationCompleted = "false";
document.documentElement.dataset.translationTimingPreserved = "false";
document.documentElement.dataset.sourceTranscriptRetainedInSession = "false";

populateBrowserModelOptions();
void initialize();
wireEvents();
restoreHfToken();
updateNativeCommand();

async function initialize() {
  const [transcriptionSupport, diarizationSupport, translationSupport] =
    await Promise.allSettled([
      supportsBrowserTranscription(),
      supportsBrowserDiarization(),
      supportsBrowserTranslation(),
    ]);
  webGpuReady = settledSupport(transcriptionSupport, "browser transcription");
  diarizationReady = settledSupport(diarizationSupport, "browser diarization");
  translationReady = settledSupport(translationSupport, "browser translation");

  if (webGpuReady) {
    elements.webGpuDot.classList.add("ready");
    elements.webGpuCapability.textContent = "WebGPU ASR ready";
    elements.webGpuDetail.textContent = `${browserModels.length} curated Whisper model${browserModels.length === 1 ? "" : "s"} via ${browserCapabilities.runtime}; browser cache reuse is enabled upstream.`;
    setBrowserStatus("Choose an audio file to run the browser-local workflow.");
  } else {
    elements.webGpuDot.classList.add("unavailable");
    elements.webGpuCapability.textContent = "WebGPU ASR unavailable";
    elements.webGpuDetail.textContent = "The browser transcription capability is disabled. The native workflow composer remains available.";
    setBrowserStatus("audio-analysis requires WebGPU for browser transcription. No server or CPU fallback will be used.");
  }

  if (diarizationReady) {
    elements.diarizationDot.classList.add("ready");
    elements.diarizationCapability.textContent = "Diarization ready";
    elements.diarizationDetail.textContent =
      `Local WASM speaker pipeline via ${diarizationCapabilities.runtime}; segmentation and speaker-embedding models load lazily from the browser cache.`;
  } else {
    elements.diarizationDot.classList.add("unavailable");
    elements.diarizationCapability.textContent = "Diarization unavailable";
    elements.diarizationDetail.textContent =
      "The local browser speaker pipeline is unavailable. No server or Python fallback will be used.";
  }

  if (translationReady) {
    elements.translationDot.classList.add("ready");
    elements.translationCapability.textContent = "Translation ready";
    elements.translationDetail.textContent = `${translationCapabilities.pairs.length} curated pair${translationCapabilities.pairs.length === 1 ? "" : "s"} via ${translationCapabilities.runtime}; models load lazily from the browser cache.`;
  } else {
    elements.translationDot.classList.add("unavailable");
    elements.translationCapability.textContent = "Translation unavailable";
    elements.translationDetail.textContent = "The WebGPU translation step is disabled. No server, Python, or CPU fallback will be used.";
  }
  updateBrowserTranslationControls();
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
  elements.browserModel.addEventListener("change", updateBrowserModelControls);
  elements.browserDiarize.addEventListener("change", updateBrowserTranslationControls);
  elements.browserTranslate.addEventListener("change", updateBrowserTranslationControls);
  elements.browserTranslationPair.addEventListener("change", updateBrowserTranslationControls);
  elements.cancelBrowser.addEventListener("click", () => {
    if (!activeBrowserRun || activeBrowserRun.cancelRequested) {
      return;
    }
    activeBrowserRun.cancelRequested = true;
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
  elements.hfToken.addEventListener("input", persistHfToken);
  elements.clearHfToken.addEventListener("click", clearSavedHfToken);
  elements.copyCommand.addEventListener("click", () => void copyNativeCommand());
}

function selectFile(file) {
  selectedFile = file;
  latestContract = null;
  elements.downloads.hidden = true;
  elements.segmentTableWrap.hidden = true;
  elements.segmentRows.replaceChildren();
  resetBrowserEvidence();

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
  elements.runBrowser.disabled =
    !webGpuReady ||
    !selectedFile ||
    activeBrowserRun !== null ||
    (elements.browserDiarize.checked && !diarizationReady) ||
    (elements.browserTranslate.checked && !translationReady);
}

function populateBrowserModelOptions() {
  elements.browserModel.replaceChildren();
  for (const model of browserModels) {
    const option = document.createElement("option");
    option.value = model.id;
    option.textContent = model.label;
    option.selected = model.id === browserCapabilities.modelId;
    elements.browserModel.append(option);
  }
  updateBrowserModelControls();
}

function selectedBrowserModel() {
  const model = browserModels.find((candidate) => candidate.id === elements.browserModel.value);
  if (!model) {
    throw new Error("The selected browser transcription model is not supported by the pinned adapter.");
  }
  return model;
}

function updateBrowserModelControls() {
  const model = selectedBrowserModel();
  elements.browserModelDescription.textContent = model.description;
  elements.browserTranscriptionStage.textContent = `audio-analysis · ${model.label} · WebGPU`;
}

function updateBrowserTranslationControls() {
  const diarizationRequested = elements.browserDiarize.checked;
  const translationRequested = elements.browserTranslate.checked;

  elements.browserDiarizationStage.textContent = diarizationRequested
    ? diarizationReady
      ? "Configured · speaker segmentation + embeddings · local WASM"
      : "Unavailable · local WASM runtime required"
    : "Off · optional local WASM step";

  elements.browserTranslationOptions.hidden = !translationRequested;
  if (translationRequested) {
    const pair = selectedBrowserTranslationPair();
    elements.browserTranslationStage.textContent = translationReady
      ? `Configured · ${pair.sourceLanguage} → ${pair.targetLanguage} · WebGPU`
      : "Unavailable · WebGPU required";
  } else {
    elements.browserTranslationStage.textContent = "Off · optional post-ASR WebGPU step";
  }

  if (diarizationRequested && translationRequested) {
    elements.runBrowser.textContent = "Transcribe, diarize and translate locally";
  } else if (diarizationRequested) {
    elements.runBrowser.textContent = "Transcribe and diarize locally";
  } else if (translationRequested) {
    elements.runBrowser.textContent = "Transcribe, then translate locally";
  } else {
    elements.runBrowser.textContent = "Run browser transcription";
  }
  updateBrowserButton();
}

async function runBrowserPreview() {
  if (!webGpuReady || !selectedFile || activeBrowserRun !== null) {
    return;
  }

  const diarizationRequested = elements.browserDiarize.checked;
  const translationRequested = elements.browserTranslate.checked;
  const model = selectedBrowserModel();
  const run = {
    id: ++browserRunSequence,
    cancelRequested: false,
    model,
    diarizationRequested,
    translationRequested,
    translationPair: translationRequested ? selectedBrowserTranslationPair() : null,
  };
  activeBrowserRun = run;
  latestContract = null;
  elements.runBrowser.disabled = true;
  elements.browserModel.disabled = true;
  elements.browserDiarize.disabled = true;
  elements.browserTranslate.disabled = true;
  elements.browserTranslationPair.disabled = true;
  elements.cancelBrowser.disabled = false;
  elements.downloads.hidden = true;
  elements.segmentTableWrap.hidden = true;
  resetBrowserEvidence();
  document.documentElement.dataset.diarizationRequested = String(run.diarizationRequested);
  document.documentElement.dataset.translationRequested = String(run.translationRequested);

  try {
    setBrowserStatus("Decoding local audio once for the browser workflow…", 2);
    const audio = await decodeBrowserAudioBlob(selectedFile, {
      onProgress: (update) => handleBrowserProgress(run, update),
    });
    throwIfCancelled(run);

    setBrowserStatus(`Handing decoded audio to ${run.model.label} in the audio-analysis browser provider…`, 8);
    const result = await transcribeAudioSamples(audio.samples, {
      source: selectedFile.name,
      durationSeconds: audio.durationSeconds,
      modelId: run.model.id,
      onProgress: (update) => handleBrowserProgress(run, update),
    });
    throwIfCancelled(run);

    let sourceContract = toNativeContract(result, selectedFile);
    if (run.diarizationRequested) {
      setBrowserStatus("Transcription finished. Diarizing speakers locally in the browser…", 80);
      const diarization = await diarizeBrowserAudioSamples(audio.samples, {
        sampleRateHz: 16_000,
        onProgress: (update) => handleBrowserDiarizationProgress(run, update),
      });
      throwIfCancelled(run);
      sourceContract = assignBrowserDiarizationToTranscript(sourceContract, diarization);
      sourceContract = {
        ...sourceContract,
        attributes: {
          ...sourceContract.attributes,
          diarization: "completed",
          diarizationRuntime: diarization.runtime,
          diarizationModelId: diarization.modelId,
          diarizationSpeakerCount: String(diarization.speakerCount),
        },
      };
      document.documentElement.dataset.diarizationCompleted = "true";
      document.documentElement.dataset.diarizationSpeakerCount = String(diarization.speakerCount);
    }
    throwIfCancelled(run);

    let publishedContract = sourceContract;
    if (run.translationRequested) {
      retainSourceTranscriptInSession(sourceContract);
      const pair = run.translationPair;
      setBrowserStatus(`Transcription finished. Translating ${pair.sourceLanguage} → ${pair.targetLanguage} locally…`, 96);
      publishedContract = await translateNativeContract(sourceContract, pair, run);
      throwIfCancelled(run);
      if (!hasMatchingSegmentIdentityAndTiming(sourceContract, publishedContract)) {
        throw new Error("Browser translation did not preserve source segment identity, timing, and speaker labels.");
      }
      document.documentElement.dataset.translationCompleted = "true";
      document.documentElement.dataset.translationTimingPreserved = "true";
    }

    latestContract = publishedContract;
    renderBrowserResult(publishedContract);
    elements.downloads.hidden = false;
    const completedStages = [
      run.diarizationRequested ? "diarization completed" : null,
      run.translationRequested ? "translation completed" : null,
    ].filter(Boolean);
    setBrowserStatus(
      `Finished locally · ${publishedContract.segments.length} timed segment${publishedContract.segments.length === 1 ? "" : "s"}${completedStages.length > 0 ? ` · ${completedStages.join(" · ")}` : ""}.`,
      100,
    );
  } catch (error) {
    if (error instanceof BrowserCancellationError) {
      elements.transcript.textContent = "Browser workflow cancelled.";
      setBrowserStatus("Browser workflow cancelled.");
    } else {
      console.error(error);
      elements.transcript.textContent = "No browser result produced.";
      setBrowserStatus(`Browser workflow failed: ${formatError(error)}`);
    }
  } finally {
    if (activeBrowserRun === run) {
      activeBrowserRun = null;
    }
    elements.cancelBrowser.disabled = true;
    elements.browserModel.disabled = false;
    elements.browserDiarize.disabled = false;
    elements.browserTranslate.disabled = false;
    elements.browserTranslationPair.disabled = false;
    updateBrowserButton();
  }
}

function handleBrowserProgress(run, update) {
  if (run !== activeBrowserRun || run.cancelRequested) {
    return;
  }
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
      diarization: "not-requested",
      translation: "not-requested",
    },
  };
}

async function translateNativeContract(sourceContract, pair, run) {
  const translated = await translateBrowserSegments(
    sourceContract.segments.map((segment) => ({ id: segment.index, text: segment.text })),
    {
      sourceLanguage: pair.sourceLanguage,
      targetLanguage: pair.targetLanguage,
      modelId: pair.modelId,
      onProgress: (update) => handleBrowserTranslationProgress(run, update),
    },
  );
  throwIfCancelled(run);

  const translatedById = new Map(translated.segments.map((segment) => [segment.id, segment.text]));
  if (translatedById.size !== sourceContract.segments.length) {
    throw new Error("Browser translation returned duplicate or unexpected segment identities.");
  }
  const segments = sourceContract.segments.map((segment) => {
    const translatedText = translatedById.get(segment.index);
    if (typeof translatedText !== "string" || translatedText.length === 0) {
      throw new Error(`Browser translation did not return segment ${segment.index}.`);
    }
    const { words: _words, chars: _chars, characters: _characters, ...preserved } = segment;
    return { ...preserved, text: translatedText, words: [], chars: [] };
  });

  return {
    ...sourceContract,
    text: segments.map((segment) => segment.text).join(" "),
    language: translated.targetLanguage,
    segments,
    attributes: {
      ...sourceContract.attributes,
      translation: "completed",
      translationDetails: { ...translated.attributes },
      translationSourceLanguage: translated.sourceLanguage,
      translationTargetLanguage: translated.targetLanguage,
      sourceTranscriptRetainedInSession: true,
      sourceTranscriptProvenance: {
        source: sourceContract.source,
        language: sourceContract.language,
        segmentCount: sourceContract.segments.length,
        textLength: sourceContract.text.length,
      },
    },
  };
}

function hasMatchingSegmentIdentityAndTiming(sourceContract, translatedContract) {
  return (
    sourceContract.segments.length === translatedContract.segments.length &&
    sourceContract.segments.every((sourceSegment, index) => {
      const translatedSegment = translatedContract.segments[index];
      return (
        sourceSegment.index === translatedSegment.index &&
        sourceSegment.startSeconds === translatedSegment.startSeconds &&
        sourceSegment.endSeconds === translatedSegment.endSeconds &&
        (sourceSegment.speaker ?? null) === (translatedSegment.speaker ?? null)
      );
    })
  );
}

function handleBrowserDiarizationProgress(run, update) {
  if (run !== activeBrowserRun || run.cancelRequested) {
    return;
  }
  const message =
    typeof update?.message === "string" ? update.message : "Running browser diarization…";
  const progressByStage = {
    model: 83,
    segment: 88,
    embed: 92,
    cluster: 95,
  };
  setBrowserStatus(message, progressByStage[update?.stage] ?? 86);
}

function settledSupport(result, capability) {
  if (result.status === "fulfilled") {
    return result.value;
  }
  console.error(`Unable to inspect ${capability} support.`, result.reason);
  return false;
}

function handleBrowserTranslationProgress(run, update) {
  if (run !== activeBrowserRun || run.cancelRequested) {
    return;
  }
  const message = typeof update?.message === "string" ? update.message : "Running browser translation…";
  setBrowserStatus(message, update?.stage === "model" ? 96 : 99);
}

function selectedBrowserTranslationPair() {
  const [sourceLanguage, targetLanguage] = elements.browserTranslationPair.value.split("-");
  const pair = translationCapabilities.pairs.find(
    (candidate) => candidate.sourceLanguage === sourceLanguage && candidate.targetLanguage === targetLanguage,
  );
  if (!pair) {
    throw new Error("The selected browser translation pair is not supported by the pinned adapter.");
  }
  return pair;
}

function retainSourceTranscriptInSession(contract) {
  elements.sourceTranscript.hidden = false;
  elements.sourceTranscriptText.textContent = contract.text || "No speech detected.";
  document.documentElement.dataset.sourceTranscriptRetainedInSession = "true";
}

function resetBrowserEvidence() {
  elements.sourceTranscript.hidden = true;
  elements.sourceTranscriptText.textContent = "";
  document.documentElement.dataset.diarizationRequested = "false";
  document.documentElement.dataset.diarizationCompleted = "false";
  document.documentElement.dataset.diarizationSpeakerCount = "0";
  document.documentElement.dataset.translationRequested = "false";
  document.documentElement.dataset.translationCompleted = "false";
  document.documentElement.dataset.translationTimingPreserved = "false";
  document.documentElement.dataset.sourceTranscriptRetainedInSession = "false";
}

function renderBrowserResult(contract) {
  elements.transcript.textContent = contract.text || "No speech detected.";
  elements.segmentRows.replaceChildren();
  for (const segment of contract.segments) {
    const row = document.createElement("tr");
    const time = document.createElement("td");
    time.textContent = segmentTime(segment);
    const speaker = document.createElement("td");
    speaker.dataset.speaker = typeof segment.speaker === "string" ? segment.speaker : "";
    speaker.textContent = speaker.dataset.speaker || "—";
    const text = document.createElement("td");
    text.textContent = segment.text;
    row.append(time, speaker, text);
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

function restoreHfToken() {
  try {
    const storedToken = window.localStorage.getItem(HF_TOKEN_STORAGE_KEY);
    if (storedToken) {
      elements.hfToken.value = storedToken;
      setHfTokenStatus("Saved token restored from this browser.");
    }
  } catch {
    setHfTokenStatus("Browser storage is unavailable. The token will only stay in this page session.");
  }
}

function persistHfToken() {
  const token = elements.hfToken.value.trim();
  try {
    if (token) {
      window.localStorage.setItem(HF_TOKEN_STORAGE_KEY, token);
      setHfTokenStatus("Saved in this browser.");
    } else {
      window.localStorage.removeItem(HF_TOKEN_STORAGE_KEY);
      setHfTokenStatus("No token is saved.");
    }
  } catch {
    setHfTokenStatus("Could not save the token in this browser.");
  }
}

function clearSavedHfToken() {
  elements.hfToken.value = "";
  try {
    window.localStorage.removeItem(HF_TOKEN_STORAGE_KEY);
    setHfTokenStatus("Saved token cleared.");
  } catch {
    setHfTokenStatus("Token cleared from this page; browser storage could not be updated.");
  }
  updateNativeCommand();
  elements.hfToken.focus();
}

function setHfTokenStatus(message) {
  elements.hfTokenStatus.textContent = message;
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
    downloadText(`${baseName}.txt`, `${renderTxt(contract)}\n`, "text/plain;charset=utf-8");
  } else if (format === "srt") {
    downloadText(`${baseName}.srt`, renderSrt(contract), "application/x-subrip;charset=utf-8");
  } else if (format === "vtt") {
    downloadText(`${baseName}.vtt`, renderVtt(contract), "text/vtt;charset=utf-8");
  }
}

function renderTxt(contract) {
  if (!contract.segments.some((segment) => typeof segment.speaker === "string" && segment.speaker)) {
    return contract.text ?? "";
  }
  return contract.segments
    .map((segment) => `${segment.speaker || "speaker_unknown"}: ${segment.text}`)
    .join("\n");
}

function renderSrt(contract) {
  return timedSegments(contract)
    .map((segment, index) => `${index + 1}\n${formatTimestamp(segment.startSeconds, ",")} --> ${formatTimestamp(segment.endSeconds, ",")}\n${renderSegmentText(segment)}\n`)
    .join("\n");
}

function renderVtt(contract) {
  const cues = timedSegments(contract)
    .map((segment) => `${formatTimestamp(segment.startSeconds, ".")} --> ${formatTimestamp(segment.endSeconds, ".")}\n${renderSegmentText(segment)}`)
    .join("\n\n");
  return `WEBVTT\n\n${cues}${cues ? "\n" : ""}`;
}

function renderSegmentText(segment) {
  return typeof segment.speaker === "string" && segment.speaker
    ? `[${segment.speaker}] ${segment.text}`
    : segment.text;
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

function throwIfCancelled(run) {
  if (run.cancelRequested) {
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
