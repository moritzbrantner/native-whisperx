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
    const fileName = text(documentRef, "#file-name");
    const fileSizeLabel = text(documentRef, "#file-size");
    const transcript = text(documentRef, "#transcript");
    const translationRequested = documentRef.documentElement.dataset.translationRequested === "true";
    const translationCompleted = documentRef.documentElement.dataset.translationCompleted === "true";
    const translationTimingPreserved =
      documentRef.documentElement.dataset.translationTimingPreserved === "true";
    const sourceTranscriptRetainedInSession =
      documentRef.documentElement.dataset.sourceTranscriptRetainedInSession === "true";
    const sourceTranscript = documentRef.querySelector("#source-transcript");
    const sourceTranscriptLength = text(documentRef, "#source-transcript-text").length;
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

    const checks = {
      webGpuReady: webGpuCapability === "WebGPU ready",
      navigatorGpuAvailable: Boolean(windowRef.navigator?.gpu),
      localFileSelected: fileName.length > 0,
      finishedLocally: browserStatus.startsWith("Finished locally"),
      transcriptProduced:
        transcript.length > 0 &&
        transcript !== "No browser result yet." &&
        transcript !== "No browser result produced." &&
        transcript !== "Browser transcription cancelled.",
      timedSegmentsProduced: timedSegmentCount > 0,
      projectionsAvailable: Boolean(downloads && !downloads.hidden),
      nativeJsonAvailable: availableFormats.includes("native-json"),
      srtAvailable: availableFormats.includes("srt"),
      webVttAvailable: availableFormats.includes("vtt"),
      txtAvailable: availableFormats.includes("txt"),
      translationCompleted: !translationRequested || translationCompleted,
      translationTimingPreserved: !translationRequested || translationTimingPreserved,
      sourceTranscriptRetainedInSession:
        !translationRequested ||
        (sourceTranscriptRetainedInSession && Boolean(sourceTranscript && !sourceTranscript.hidden) && sourceTranscriptLength > 0),
    };
    const passed = Object.values(checks).every(Boolean);

    latestEvidence = {
      schemaVersion: 1,
      capturedAt: new Date().toISOString(),
      acceptancePageUrl: location.href,
      workbenchUrl: windowRef.location.href,
      userAgent: navigator.userAgent,
      webGpuCapability,
      browserStatus,
      fileName,
      fileSizeLabel,
      transcriptLength: transcript.length,
      translationRequested,
      translationCompleted,
      translationTimingPreserved,
      sourceTranscriptRetainedInSession,
      sourceTranscriptLength,
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
      ? "PASS: the deployed workbench produced a local WebGPU transcript with timed segments and export projections."
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
  if (!match) {
    return false;
  }

  const startSeconds = Number(match[1]) * 60 + Number(match[2]);
  const endSeconds = Number(match[3]) * 60 + Number(match[4]);
  return Number.isFinite(startSeconds) && Number.isFinite(endSeconds) && endSeconds >= startSeconds;
}

function downloadEvidence() {
  if (!latestEvidence) {
    return;
  }

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

function formatError(error) {
  return error instanceof Error ? error.message : String(error);
}
