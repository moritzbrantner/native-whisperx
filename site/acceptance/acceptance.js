const elements = {
  frame: document.querySelector("#workbench"),
  capture: document.querySelector("#capture"),
  download: document.querySelector("#download"),
  result: document.querySelector("#result"),
  report: document.querySelector("#report"),
};

let latestEvidence = null;

elements.capture.addEventListener("click", captureEvidence);
elements.download.addEventListener("click", downloadEvidence);

function captureEvidence() {
  try {
    const documentRef = elements.frame.contentDocument;
    const windowRef = elements.frame.contentWindow;
    if (!documentRef || !windowRef) {
      throw new Error("The embedded workbench has not loaded yet.");
    }

    const webGpuCapability = text(documentRef, "#webgpu-capability");
    const browserStatus = text(documentRef, "#browser-status");
    const translationStatus = text(documentRef, "#browser-translation-status");
    const translationRequested = documentRef.querySelector("#browser-translate")?.checked === true;
    const translationModel = value(documentRef, "#browser-translation-model");
    const translationSourceLanguage = value(documentRef, "#browser-translation-source");
    const translationTargetLanguage = value(documentRef, "#browser-translation-target");
    const fileName = text(documentRef, "#file-name");
    const fileSizeLabel = text(documentRef, "#file-size");
    const transcript = text(documentRef, "#transcript");
    const sourceTranscript = text(documentRef, "#source-transcript");
    const sourceTranscriptWrap = documentRef.querySelector("#source-transcript-wrap");
    const segmentRows = Array.from(documentRef.querySelectorAll("#segment-rows tr"));
    const segmentCount = segmentRows.length;
    const timedSegmentCount = segmentRows.filter(hasValidRenderedTiming).length;
    const downloads = documentRef.querySelector("#download-actions");
    const availableFormats = Array.from(
      documentRef.querySelectorAll("#download-actions button[data-format]"),
      (button) => button.dataset.format,
    )
      .filter(Boolean)
      .sort();

    const translationCompleted =
      !translationRequested || translationStatus.startsWith("Translated locally");
    const sourceTranscriptPreserved =
      !translationRequested || Boolean(sourceTranscriptWrap && !sourceTranscriptWrap.hidden && sourceTranscript.length > 0);

    const checks = {
      webGpuReady: webGpuCapability === "WebGPU ready",
      navigatorGpuAvailable: Boolean(windowRef.navigator?.gpu),
      localFileSelected: fileName.length > 0,
      finishedLocally: browserStatus.startsWith("Finished locally"),
      transcriptProduced:
        transcript.length > 0 &&
        transcript !== "No browser result yet." &&
        transcript !== "No browser result produced." &&
        transcript !== "Browser workflow cancelled.",
      timedSegmentsProduced: timedSegmentCount > 0,
      projectionsAvailable: Boolean(downloads && !downloads.hidden),
      nativeJsonAvailable: availableFormats.includes("native-json"),
      srtAvailable: availableFormats.includes("srt"),
      webVttAvailable: availableFormats.includes("vtt"),
      txtAvailable: availableFormats.includes("txt"),
      translationCompleted,
      sourceTranscriptPreserved,
    };
    const passed = Object.values(checks).every(Boolean);

    latestEvidence = {
      schemaVersion: 2,
      capturedAt: new Date().toISOString(),
      acceptancePageUrl: location.href,
      workbenchUrl: windowRef.location.href,
      userAgent: navigator.userAgent,
      webGpuCapability,
      browserStatus,
      translationRequested,
      translationStatus,
      translationModel: translationRequested ? translationModel : null,
      translationSourceLanguage: translationRequested ? translationSourceLanguage : null,
      translationTargetLanguage: translationRequested ? translationTargetLanguage : null,
      fileName,
      fileSizeLabel,
      transcriptLength: transcript.length,
      sourceTranscriptLength: translationRequested ? sourceTranscript.length : 0,
      segmentCount,
      timedSegmentCount,
      availableFormats,
      checks,
      passed,
    };

    elements.report.hidden = false;
    elements.report.textContent = JSON.stringify(latestEvidence, null, 2);
    elements.download.disabled = false;
    elements.result.className = passed ? "pass" : "fail";
    elements.result.textContent = passed
      ? translationRequested
        ? "PASS: the deployed workbench produced and translated a local WebGPU transcript with valid timing and export projections."
        : "PASS: the deployed workbench produced a local WebGPU transcript with timed segments and export projections."
      : "FAIL: one or more browser runtime acceptance checks are not satisfied yet. The JSON report identifies each check.";
  } catch (error) {
    latestEvidence = null;
    elements.download.disabled = true;
    elements.report.hidden = true;
    elements.result.className = "fail";
    elements.result.textContent = `Unable to capture acceptance evidence: ${formatError(error)}`;
  }
}

function hasValidRenderedTiming(row) {
  const value = row.cells?.[0]?.textContent?.trim() ?? "";
  const match = /^(\d+):(\d+(?:\.\d+)?)\s+–\s+(\d+):(\d+(?:\.\d+)?)$/.exec(value);
  if (!match) return false;

  const startSeconds = Number(match[1]) * 60 + Number(match[2]);
  const endSeconds = Number(match[3]) * 60 + Number(match[4]);
  return Number.isFinite(startSeconds) && Number.isFinite(endSeconds) && endSeconds >= startSeconds;
}

function downloadEvidence() {
  if (!latestEvidence) return;

  const blob = new Blob([`${JSON.stringify(latestEvidence, null, 2)}\n`], {
    type: "application/json",
  });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  const timestamp = latestEvidence.capturedAt.replaceAll(":", "-");
  anchor.href = url;
  anchor.download = `native-whisperx-browser-acceptance-${timestamp}.json`;
  anchor.click();
  setTimeout(() => URL.revokeObjectURL(url), 0);
}

function text(documentRef, selector) {
  return documentRef.querySelector(selector)?.textContent?.trim() ?? "";
}

function value(documentRef, selector) {
  const candidate = documentRef.querySelector(selector)?.value;
  return typeof candidate === "string" ? candidate.trim() : "";
}

function formatError(error) {
  return error instanceof Error ? error.message : String(error);
}
