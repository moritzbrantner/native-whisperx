import { alignTranscriptAudioSamples, browserAlignmentCapabilities, supportsBrowserAlignment } from "./vendor/audio-analysis-alignment.js";
import { languageOptions, languageLabel } from "./vendor/workbench-languages.js";
import {
  decodeBrowserAudioBlob,
  supportsBrowserAudioDecode,
} from "./vendor/audio-analysis-io.js";
import {
  browserTranscriptionCapabilities,
  browserTranscriptionModels,
  browserTranscriptionLanguages,
  supportsBrowserTranscription,
  transcribeAudioSamples,
} from "./vendor/audio-analysis-transcription.js";
import {
  assignBrowserSpeakersToSegments,
} from "./vendor/audio-analysis-speakers.js";
import { browserCommunityDiarizationCapabilities, supportsBrowserCommunityDiarization, diarizeCommunityAudioSamples } from "./vendor/audio-analysis-community.js";
import { loadCommunityDiarizationResources } from "./vendor/audio-analysis-resources.js";
import {
  browserTranslationCapabilities,
  supportsBrowserTranslation,
  translateBrowserSegments,
} from "./vendor/browser-translation.js";

const elements = {
  browserAlign: document.querySelector("#browser-align"),
  browserAlignmentStage: document.querySelector("#browser-alignment-stage"),
  browserLanguage: document.querySelector("#browser-language"),
  languageProblem: document.querySelector("#browser-language-problem"),
  webGpuDot: document.querySelector("#webgpu-dot"),
  webGpuCapability: document.querySelector("#webgpu-capability"),
  webGpuDetail: document.querySelector("#webgpu-detail"),
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
  browserDiarize: document.querySelector("#browser-diarize"),
  browserDiarizationStage: document.querySelector("#browser-diarization-stage"),
  transcript: document.querySelector("#transcript"),
  segmentTableWrap: document.querySelector("#segment-table-wrap"),
  segmentRows: document.querySelector("#segment-rows"),
  downloads: document.querySelector("#download-actions"),
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
const alignmentCapabilities = browserAlignmentCapabilities();
let alignmentReady = false;
const diarizationCapabilities = browserCommunityDiarizationCapabilities();
const translationCapabilities = browserTranslationCapabilities();
let webGpuReady = false;
let audioDecodeReady = false;
let diarizationReady = false;
let communityBundle = null;
let translationReady = false;
let selectedFile = null;
let previewUrl = null;
let latestContract = null;
let activeBrowserRun = null;
let browserRunSequence = 0;

document.documentElement.dataset.audioDecodeCount = "0";
document.documentElement.dataset.diarizationRequested = "false";
document.documentElement.dataset.diarizationCompleted = "false";
document.documentElement.dataset.translationRequested = "false";
document.documentElement.dataset.translationCompleted = "false";
document.documentElement.dataset.translationTimingPreserved = "false";
document.documentElement.dataset.sourceTranscriptRetainedInSession = "false";

populateBrowserModelOptions();
populateLanguageOptions();
void initialize();
wireEvents();
restoreHfToken();
updateNativeCommand();

async function initialize() {
  alignmentReady = await supportsBrowserAlignment();
  updateBrowserAlignmentControls();
  audioDecodeReady = supportsBrowserAudioDecode();
  const [transcriptionSupport, diarizationSupport, translationSupport] = await Promise.allSettled([
    supportsBrowserTranscription(),
    supportsBrowserCommunityDiarization(),
    supportsBrowserTranslation(),
  ]);
  webGpuReady = settledSupport(transcriptionSupport, "browser transcription");
  diarizationReady =
    audioDecodeReady && settledSupport(diarizationSupport, "browser diarization");
  translationReady = settledSupport(translationSupport, "browser translation");

  if (webGpuReady) {
    elements.webGpuDot.classList.add("ready");
    elements.webGpuCapability.textContent = "WebGPU ready";
    elements.webGpuDetail.textContent = `${browserModels.length} curated Whisper model${browserModels.length === 1 ? "" : "s"} via ${browserCapabilities.runtime}; browser cache reuse is enabled upstream.`;
    setBrowserStatus("Choose an audio file.");
  } else {
    elements.webGpuDot.classList.add("unavailable");
    elements.webGpuCapability.textContent = "WebGPU unavailable";
    elements.webGpuDetail.textContent = "The browser transcription capability is disabled. The native workflow composer remains available.";
    setBrowserStatus("WebGPU is unavailable. Use a supported browser or the local app; audio will not be sent to a server.");
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
  updateBrowserDiarizationControls();
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
  elements.browserAlign.addEventListener("change", updateBrowserAlignmentControls);
  elements.browserDiarize.addEventListener("change", updateBrowserDiarizationControls);
  elements.browserTranslate.addEventListener("change", updateBrowserTranslationControls);
  elements.browserLanguage.addEventListener("change", () => {
    const matching = translationCapabilities.pairs.find(pair => pair.sourceLanguage === elements.browserLanguage.value);
    if (matching) elements.browserTranslationPair.value = `${matching.sourceLanguage}-${matching.targetLanguage}`;
    updateBrowserTranslationControls();
    updateBrowserAlignmentControls();
  });
  elements.browserTranslationPair.addEventListener("change", () => {
    elements.browserLanguage.value = selectedBrowserTranslationPair().sourceLanguage;
    updateBrowserTranslationControls();
    updateBrowserAlignmentControls();
  });
  elements.cancelBrowser.addEventListener("click", () => {
    if (!activeBrowserRun || activeBrowserRun.cancelRequested) {
      return;
    }
    activeBrowserRun.cancelRequested = true;
    activeBrowserRun.controller.abort();
    elements.cancelBrowser.disabled = true;
    setBrowserStatus("Cancellation requested. The current upstream browser step will finish before stopping.");
  });
  elements.downloads.addEventListener("click", (event) => {
    const button = event.target.closest("button[data-format]");
    if (!button || !latestContract?.source) {
      return;
    }
    downloadProjection(button.dataset.format, latestContract, latestContract.source);
  });

  for (const control of document.querySelectorAll("#native-workflow input, #native-workflow select")) {
    control.addEventListener("input", updateNativeCommand);
    control.addEventListener("change", updateNativeCommand);
  }
  elements.nativeAlign.addEventListener("change", updateNativeVisibility);
  elements.nativeDiarize.addEventListener("change", updateNativeVisibility);
  elements.nativeTranslate.addEventListener("change", updateNativeVisibility);
  elements.hfToken.addEventListener("input", updateBrowserButton);
  elements.clearHfToken.addEventListener("click", clearSavedHfToken);
  elements.copyCommand.addEventListener("click", () => void copyNativeCommand());
}

function selectFile(file) {
  if (activeBrowserRun !== null) return;
  selectedFile = file;
  elements.dropZone.classList.toggle("has-file", Boolean(file));
  elements.chooseFile.textContent = file ? "Choose another file" : "Choose audio file";
  latestContract = null;
  elements.transcript.hidden = false;
  elements.downloads.hidden = true;
  elements.segmentTableWrap.hidden = true;
  elements.segmentRows.replaceChildren();
  document.documentElement.dataset.alignmentRequested = "false";
  document.documentElement.dataset.alignmentCompleted = "false";
  resetBrowserDecodeEvidence();
  resetBrowserDiarizationEvidence();
  resetBrowserTranslationEvidence();

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
  elements.transcript.textContent = "Run to preview your transcript.";
  setBrowserStatus("Ready to run the selected tasks.");
  updateBrowserButton();
  updateNativeCommand();
}

function populateLanguageOptions() {
  elements.nativeLanguage.replaceChildren(new Option("Auto-detect", ""));
  for (const {code,label} of languageOptions(browserTranscriptionLanguages())) {
    elements.browserLanguage.add(new Option(label, code));
    elements.nativeLanguage.add(new Option(label, code));
  }
  elements.browserTranslationPair.replaceChildren(...translationCapabilities.pairs.map(pair =>
    new Option(`${languageLabel(pair.sourceLanguage)} → ${languageLabel(pair.targetLanguage)}`, `${pair.sourceLanguage}-${pair.targetLanguage}`)));
}

function translationLanguageMatches() {
  return !elements.browserTranslate.checked || elements.browserLanguage.value === selectedBrowserTranslationPair().sourceLanguage;
}

function alignmentLanguageMatches() {
  return !elements.browserAlign.checked || alignmentCapabilities.languages.includes(elements.browserLanguage.value);
}

function updateBrowserAlignmentControls() {
  elements.browserAlign.disabled = !alignmentReady || activeBrowserRun !== null;
  elements.browserAlignmentStage.textContent = !alignmentReady
    ? "Unavailable · WebAssembly required"
    : !alignmentLanguageMatches()
      ? "Choose English or German as the transcription language"
      : elements.browserAlign.checked
        ? "CPU WASM · download 117–361 MB"
        : "English / German · CPU WASM";
  updateBrowserRunLabel();
  updateBrowserButton();
}

function updateBrowserButton() {
  elements.runBrowser.disabled =
    !webGpuReady ||
    !audioDecodeReady ||
    !alignmentLanguageMatches() ||
    (elements.browserAlign.checked && !alignmentReady) ||
    !selectedFile ||
    activeBrowserRun !== null ||
    (elements.browserDiarize.checked && (!diarizationReady || (!communityBundle && !elements.hfToken.value.trim()))) ||
    (elements.browserTranslate.checked && !translationReady) || !translationLanguageMatches();
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

function updateBrowserDiarizationControls() {
  document.querySelector("#browser-pyannote-options").hidden = !elements.browserDiarize.checked;
  elements.browserDiarize.disabled = !diarizationReady || activeBrowserRun !== null;
  const requested = elements.browserDiarize.checked;
  elements.browserDiarizationStage.hidden = !requested && diarizationReady;
  elements.browserDiarizationStage.textContent = requested
    ? diarizationReady
      ? `Configured · ${diarizationCapabilities.modelId} · local`
      : "Unavailable · browser audio APIs required"
    : "Off · optional pyannote community-1";
  updateBrowserRunLabel();
  updateBrowserButton();
}

function updateBrowserTranslationControls() {
  if (elements.browserTranslate.checked && !elements.browserLanguage.value) {
    elements.browserLanguage.value = selectedBrowserTranslationPair().sourceLanguage;
  }
  const matching = translationLanguageMatches();
  elements.languageProblem.hidden = matching;
  elements.languageProblem.textContent = matching ? "" : "This translation pair does not match the transcription language. Choose a supported translation pair or turn Translate off.";

  elements.browserTranslate.disabled = !translationReady || activeBrowserRun !== null;
  const requested = elements.browserTranslate.checked;
  elements.browserTranslationStage.hidden = !requested;
  elements.translationCapability.hidden = translationReady;
  elements.browserTranslationOptions.hidden = !requested;
  if (requested) {
    const pair = selectedBrowserTranslationPair();
    elements.browserTranslationStage.textContent = translationReady
      ? `Configured · ${pair.sourceLanguage} → ${pair.targetLanguage} · WebGPU`
      : "Unavailable · WebGPU required";
  } else {
    elements.browserTranslationStage.textContent = "Off · optional post-ASR WebGPU step";
  }
  updateBrowserRunLabel();
  updateBrowserButton();
}

function updateBrowserRunLabel() {
  const stages = ["Transcribe"];
  if (elements.browserAlign.checked) stages.push("align");
  if (elements.browserDiarize.checked) {
    stages.push("diarize");
  }
  if (elements.browserTranslate.checked) {
    stages.push("translate");
  }
  elements.runBrowser.textContent =
    stages.length === 1 ? "Run browser transcription" : "Run selected tasks locally";
}

async function runBrowserPreview() {
  if (!webGpuReady || !selectedFile || activeBrowserRun !== null || !translationLanguageMatches() || !alignmentLanguageMatches()) {
    return;
  }

  const diarizationRequested = elements.browserDiarize.checked;
  const translationRequested = elements.browserTranslate.checked;
  const model = selectedBrowserModel();
  const run = {
    id: ++browserRunSequence,
    controller: new AbortController(),
    alignmentRequested: elements.browserAlign.checked,
    cancelRequested: false,
    inputFile: selectedFile,
    model,
    language: elements.browserLanguage.value || null,
    diarizationRequested,
    translationRequested,
    translationPair: translationRequested ? selectedBrowserTranslationPair() : null,
  };
  activeBrowserRun = run;
  latestContract = null;
  elements.transcript.hidden = false;
  elements.runBrowser.disabled = true;
  elements.browserModel.disabled = true;
  elements.browserLanguage.disabled = true;
  elements.browserAlign.disabled = true;
  elements.browserDiarize.disabled = true;
  elements.browserTranslate.disabled = true;
  elements.browserTranslationPair.disabled = true;
  elements.cancelBrowser.disabled = false;
  elements.cancelBrowser.hidden = false;
  elements.chooseFile.disabled = true;
  elements.fileInput.disabled = true;
  elements.downloads.hidden = true;
  elements.segmentTableWrap.hidden = true;
  resetBrowserDecodeEvidence();
  resetBrowserDiarizationEvidence();
  resetBrowserTranslationEvidence();
  document.documentElement.dataset.alignmentRequested = String(run.alignmentRequested);
  document.documentElement.dataset.alignmentCompleted = "false";
  document.documentElement.dataset.diarizationRequested = String(run.diarizationRequested);
  document.documentElement.dataset.translationRequested = String(run.translationRequested);

  try {
    if (run.diarizationRequested && !communityBundle) {
      setBrowserStatus("Preparing verified pyannote models…", 1);
      try {
        const bundle = await loadCommunityDiarizationResources({
          token: elements.hfToken.value.trim(),
          signal: run.controller.signal,
          onProgress: ({ message }) => { if (run === activeBrowserRun && !run.cancelRequested) setBrowserStatus(message, 1); },
        });
        throwIfCancelled(run);
        communityBundle = bundle;
        setHfTokenStatus("Verified models ready in this page session.");
      } finally { elements.hfToken.value = ""; }
    }
    throwIfCancelled(run);
    setBrowserStatus("Decoding local audio once through audio-analysis I/O…", 2);
    const decodedAudio = await decodeBrowserAudioBlob(run.inputFile, { sampleRateHz: 16_000 });
    document.documentElement.dataset.audioDecodeCount = "1";
    throwIfCancelled(run);

    setBrowserStatus(`Handing decoded PCM to ${run.model.label} in the audio-analysis browser provider…`, 5);
    let result = await transcribeAudioSamples(decodedAudio.samples, {
      source: run.inputFile.name,
      durationSeconds: decodedAudio.durationSeconds,
      modelId: run.model.id,
      ...(run.language ? { language: run.language } : {}),
      onProgress: (update) => handleBrowserProgress(run, update),
    });
    throwIfCancelled(run);

    if (run.alignmentRequested) {
      setBrowserStatus("Transcription finished. Aligning word timestamps locally…", 91);
      result = await alignTranscriptAudioSamples(decodedAudio.samples, result, {
        language: run.language,
        signal: run.controller.signal,
        onProgress: (update) => {
          if (run === activeBrowserRun && !run.cancelRequested) setBrowserStatus(update.message, 92);
        },
      });
      throwIfCancelled(run);
      document.documentElement.dataset.alignmentCompleted = "true";
    }

    let sourceContract = toNativeContract(result, run.inputFile);
    let observedSpeakerCount = null;
    throwIfCancelled(run);

    if (run.diarizationRequested) {
      setBrowserStatus("Transcription finished. Diarizing speakers locally…", 93);
      const diarization = await diarizeCommunityAudioSamples(decodedAudio.samples, communityBundle, {
        signal: run.controller.signal,
        onProgress: (update) => handleBrowserDiarizationProgress(run, update),
      });
      throwIfCancelled(run);
      sourceContract = applyBrowserDiarization(sourceContract, diarization);
      observedSpeakerCount = diarization.speakerCount;
      document.documentElement.dataset.diarizationCompleted = "true";
    }

    let publishedContract = sourceContract;
    if (run.translationRequested) {
      retainSourceTranscriptInSession(sourceContract);
      const pair = run.translationPair;
      setBrowserStatus(`Transcription finished. Translating ${pair.sourceLanguage} → ${pair.targetLanguage} locally…`, 94);
      publishedContract = await translateNativeContract(sourceContract, pair, run);
      throwIfCancelled(run);
      if (!hasMatchingSegmentIdentityAndTiming(sourceContract, publishedContract)) {
        throw new Error("Browser translation did not preserve source segment identity and timing.");
      }
      document.documentElement.dataset.translationCompleted = "true";
      document.documentElement.dataset.translationTimingPreserved = "true";
    }

    latestContract = publishedContract;
    renderBrowserResult(publishedContract);
    elements.downloads.hidden = false;
    setBrowserStatus(
      `Finished locally · ${publishedContract.segments.length} timed segment${publishedContract.segments.length === 1 ? "" : "s"}${run.alignmentRequested ? " · alignment completed" : ""}${run.diarizationRequested ? ` · ${observedSpeakerCount} speaker${observedSpeakerCount === 1 ? "" : "s"} (tentative automatic count)` : ""}${run.translationRequested ? " · translation completed" : ""}.`,
      100,
    );
  } catch (error) {
    if (run.cancelRequested || error instanceof BrowserCancellationError) {
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
    elements.cancelBrowser.hidden = true;
    elements.chooseFile.disabled = false;
    elements.fileInput.disabled = false;
    elements.browserModel.disabled = false;
    elements.browserLanguage.disabled = false;
    elements.browserTranslationPair.disabled = false;
    updateBrowserAlignmentControls();
    updateBrowserDiarizationControls();
    updateBrowserTranslationControls();
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
      alignment: result?.attributes?.alignment ?? "not-requested",
      diarization: "not-requested",
      translation: "not-requested",
    },
  };
}

function applyBrowserDiarization(sourceContract, diarization) {
  const segments = assignBrowserSpeakersToSegments(sourceContract.segments, diarization);
  return {
    ...sourceContract,
    segments,
    attributes: {
      ...sourceContract.attributes,
      diarization: "completed",
      diarizationDetails: {
        modelId: diarization.modelId,
        runtime: diarization.runtime,
        speakerCount: diarization.speakerCount,
        quality: diarization.attributes?.quality ?? diarizationCapabilities.quality,
        sourceRevision: diarization.attributes.sourceRevision,
        artifactSetSha256: diarization.attributes.artifactSetSha256,
        countStatus: "tentativeAutomatic",
        countUncertainty: "notReported",
      },
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
        sourceSegment.endSeconds === translatedSegment.endSeconds
      );
    })
  );
}

function settledSupport(result, capability) {
  if (result.status === "fulfilled") {
    return result.value;
  }
  console.error(`Unable to inspect ${capability} support.`, result.reason);
  return false;
}

function handleBrowserDiarizationProgress(run, update) {
  if (run !== activeBrowserRun || run.cancelRequested) {
    return;
  }
  const message = typeof update?.message === "string" ? update.message : "Running browser diarization…";
  setBrowserStatus(message, update?.stage === "decode" ? 93 : 95);
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

function resetBrowserDecodeEvidence() {
  document.documentElement.dataset.audioDecodeCount = "0";
}

function resetBrowserDiarizationEvidence() {
  document.documentElement.dataset.diarizationRequested = "false";
  document.documentElement.dataset.diarizationCompleted = "false";
}

function resetBrowserTranslationEvidence() {
  elements.sourceTranscript.hidden = true;
  elements.sourceTranscriptText.textContent = "";
  document.documentElement.dataset.translationRequested = "false";
  document.documentElement.dataset.translationCompleted = "false";
  document.documentElement.dataset.translationTimingPreserved = "false";
  document.documentElement.dataset.sourceTranscriptRetainedInSession = "false";
}

function renderBrowserResult(contract) {
  elements.transcript.textContent = contract.text || "No speech detected.";
  elements.transcript.hidden = contract.segments.length > 0;
  elements.segmentRows.replaceChildren();
  for (const segment of contract.segments) {
    const row = document.createElement("tr");
    const time = document.createElement("td");
    time.textContent = segmentTime(segment);
    const speaker = document.createElement("td");
    speaker.textContent = segment.speaker ?? "—";
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
  // Remove credentials saved by older versions; tokens now live only in the input.
  try { window.localStorage.removeItem(HF_TOKEN_STORAGE_KEY); } catch { /* Storage may be disabled. */ }
  setHfTokenStatus("Token is kept only for this setup request.");
}
function clearSavedHfToken() {
  elements.hfToken.value = "";
  setHfTokenStatus("Token cleared.");
  updateBrowserButton();
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
    downloadText(`${baseName}.txt`, renderPlainText(contract), "text/plain;charset=utf-8");
  } else if (format === "srt") {
    downloadText(`${baseName}.srt`, renderSrt(contract), "application/x-subrip;charset=utf-8");
  } else if (format === "vtt") {
    downloadText(`${baseName}.vtt`, renderVtt(contract), "text/vtt;charset=utf-8");
  }
}

function renderPlainText(contract) {
  const segments = Array.isArray(contract.segments) ? contract.segments : [];
  if (segments.some((segment) => segment.speaker)) {
    return `${segments.map(speakerDecoratedText).join("\n")}\n`;
  }
  return `${contract.text ?? ""}\n`;
}

function renderSrt(contract) {
  return timedSegments(contract)
    .map((segment, index) => `${index + 1}\n${formatTimestamp(segment.startSeconds, ",")} --> ${formatTimestamp(segment.endSeconds, ",")}\n${speakerDecoratedText(segment)}\n`)
    .join("\n");
}

function renderVtt(contract) {
  const cues = timedSegments(contract)
    .map((segment) => `${formatTimestamp(segment.startSeconds, ".")} --> ${formatTimestamp(segment.endSeconds, ".")}\n${speakerDecoratedText(segment)}`)
    .join("\n\n");
  return `WEBVTT\n\n${cues}${cues ? "\n" : ""}`;
}

function speakerDecoratedText(segment) {
  return segment.speaker ? `[${segment.speaker}]: ${segment.text}` : segment.text;
}

function timedSegments(contract) {
  return contract.segments.filter((segment) => Number.isFinite(segment.startSeconds) && Number.isFinite(segment.endSeconds) && segment.endSeconds >= segment.startSeconds);
}

function segmentTime(segment) {
  if (!Number.isFinite(segment.startSeconds) || !Number.isFinite(segment.endSeconds)) {
    return "untimed";
  }
  return `${formatTimestamp(segment.startSeconds, ".")} – ${formatTimestamp(segment.endSeconds, ".")}`;
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
