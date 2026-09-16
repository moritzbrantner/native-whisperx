const BROWSER_RUN_EVIDENCE_KEY = "__nativeWhisperxBrowserRunEvidence";

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
    const windowRef = elements.frame.contentWindow;
    if (!windowRef) {
      throw new Error("The embedded workbench has not loaded yet.");
    }

    const completedRun = windowRef[BROWSER_RUN_EVIDENCE_KEY];
    if (!completedRun || typeof completedRun !== "object") {
      throw new Error(
        "No completed browser workflow evidence is available. Run the embedded workbench successfully before capturing acceptance evidence.",
      );
    }

    const availableFormats = Array.isArray(completedRun.availableFormats)
      ? completedRun.availableFormats.filter((format) => typeof format === "string").sort()
      : [];
    const translationRequested = completedRun.translationRequested === true;
    const translationCompleted =
      !translationRequested || completedRun.translationCompleted === true;
    const sourceTranscriptPreserved =
      !translationRequested || completedRun.sourceTranscriptPreserved === true;
    const translationProvenanceComplete =
      !translationRequested ||
      (nonEmpty(completedRun.translationModel) &&
        nonEmpty(completedRun.translationSourceLanguage) &&
        nonEmpty(completedRun.translationTargetLanguage) &&
        nonEmpty(completedRun.translationRuntime));

    const checks = {
      completedRunAvailable: true,
      webGpuReady: completedRun.webGpuCapability === "WebGPU ready",
      navigatorGpuAvailable: Boolean(windowRef.navigator?.gpu),
      localFileSelected: nonEmpty(completedRun.fileName),
      finishedLocally:
        typeof completedRun.browserStatus === "string" &&
        completedRun.browserStatus.startsWith("Finished locally"),
      transcriptProduced:
        Number.isInteger(completedRun.transcriptLength) && completedRun.transcriptLength > 0,
      timedSegmentsProduced:
        Number.isInteger(completedRun.timedSegmentCount) && completedRun.timedSegmentCount > 0,
      projectionsAvailable: completedRun.projectionsAvailable === true,
      nativeJsonAvailable: availableFormats.includes("native-json"),
      srtAvailable: availableFormats.includes("srt"),
      webVttAvailable: availableFormats.includes("vtt"),
      txtAvailable: availableFormats.includes("txt"),
      translationCompleted,
      translationProvenanceComplete,
      sourceTranscriptPreserved,
    };
    const passed = Object.values(checks).every(Boolean);

    latestEvidence = {
      schemaVersion: 3,
      capturedAt: new Date().toISOString(),
      completedAt: completedRun.completedAt ?? null,
      acceptancePageUrl: location.href,
      workbenchUrl: windowRef.location.href,
      userAgent: navigator.userAgent,
      webGpuCapability: completedRun.webGpuCapability ?? null,
      browserStatus: completedRun.browserStatus ?? null,
      translationRequested,
      translationCompleted,
      translationStatus: completedRun.translationStatus ?? null,
      translationModel: translationRequested ? completedRun.translationModel ?? null : null,
      translationSourceLanguage: translationRequested
        ? completedRun.translationSourceLanguage ?? null
        : null,
      translationTargetLanguage: translationRequested
        ? completedRun.translationTargetLanguage ?? null
        : null,
      translationRuntime: translationRequested ? completedRun.translationRuntime ?? null : null,
      detectedSourceLanguage: completedRun.detectedSourceLanguage ?? null,
      fileName: completedRun.fileName ?? null,
      fileSizeBytes: completedRun.fileSizeBytes ?? null,
      fileSizeLabel: completedRun.fileSizeLabel ?? null,
      transcriptLength: completedRun.transcriptLength ?? null,
      sourceTranscriptLength: completedRun.sourceTranscriptLength ?? 0,
      segmentCount: completedRun.segmentCount ?? null,
      timedSegmentCount: completedRun.timedSegmentCount ?? null,
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
        ? "PASS: the completed browser run produced and translated a local WebGPU transcript with valid timing and export projections."
        : "PASS: the completed browser run produced a local WebGPU transcript with timed segments and export projections."
      : "FAIL: one or more browser runtime acceptance checks are not satisfied yet. The JSON report identifies each check.";
  } catch (error) {
    latestEvidence = null;
    elements.download.disabled = true;
    elements.report.hidden = true;
    elements.result.className = "fail";
    elements.result.textContent = `Unable to capture acceptance evidence: ${formatError(error)}`;
  }
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

function nonEmpty(value) {
  return typeof value === "string" && value.trim().length > 0;
}

function formatError(error) {
  return error instanceof Error ? error.message : String(error);
}
