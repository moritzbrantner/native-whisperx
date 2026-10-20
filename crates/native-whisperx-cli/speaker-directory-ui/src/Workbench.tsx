import {
  languageOptions,
  transcriptionLanguages,
  translationLanguages,
} from "../../../../ui/languages";
import { TimedTranscript } from "./TimedTranscript";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect, useState, type ReactNode } from "react";

import {
  workbenchApi,
  parseWorkbenchDevice,
  type WorkbenchSettings,
  type WorkbenchFormat,
} from "./workbenchApi";
import { useWorkbenchConfiguration } from "./workbenchConfiguration";
import { WorkbenchPresets } from "./WorkbenchPresets";
import { jobsApi } from "./jobsApi";
import { jobsKey } from "./Jobs";

const outputFormats: WorkbenchFormat[] = [
  "json",
  "srt",
  "vtt",
  "txt",
  "tsv",
  "native-json",
  "aud",
  "all",
];

const jobKey = ["workbench", "current-job"];
const taskStates: Record<string, string> = {
  pending: "Waiting",
  running: "Running",
  completed: "Completed",
  failed: "Failed",
  cancelled: "Cancelled",
  notRun: "Not run",
};
const taskLabels: Record<string, string> = {
  decode: "Read media",
  vad: "Detect speech",
  transcribe: "Transcribe",
  align: "Align timestamps",
  identifySpeakers: "Identify speakers",
  translate: "Translate",
  writeOutputs: "Write outputs",
};

export function Workbench({ speakerIndex }: { speakerIndex?: ReactNode }) {
  const client = useQueryClient();
  const capabilities = useQuery({
    queryKey: ["workbench", "capabilities"],
    queryFn: workbenchApi.capabilities,
    retry: false,
  });
  const state = useQuery({
    queryKey: jobKey,
    queryFn: workbenchApi.state,
    retry: false,
    refetchInterval: (query) =>
      ["running", "cancelling"].includes(query.state.data?.status ?? "") ? 750 : false,
  });
  const [input, setInput] = useState("");
  const [multiple, setMultiple] = useState(false);
  const [additionalInputs, setAdditionalInputs] = useState("");
  const [browse, setBrowse] = useState(false);
  const files = useMutation({ mutationFn: workbenchApi.files });
  const start = useMutation({
    mutationFn: workbenchApi.run,
    onSuccess: (job) => {
      client.setQueryData(jobKey, job);
      result.reset();
    },
  });
  const queue = useMutation({
    mutationFn: jobsApi.queue,
    onSuccess: (history) => {
      client.setQueryData(jobsKey, history);
      void client.invalidateQueries({ queryKey: jobKey });
      result.reset();
    },
  });
  const cancel = useMutation({
    mutationFn: workbenchApi.cancel,
    onSuccess: (job) => client.setQueryData(jobKey, job),
  });
  const result = useMutation({ mutationFn: workbenchApi.result });
  useEffect(() => {
    if (["completed", "failed", "cancelled"].includes(state.data?.status ?? "")) {
      // QueryClient owns refetch/error lifetime; synchronize the externally updated index.
      void client.invalidateQueries({ queryKey: ["speaker-directory-state"] });
    }
  }, [client, state.data?.status]);
  const busy =
    start.isPending ||
    queue.isPending ||
    ["running", "cancelling"].includes(state.data?.status ?? "");
  const capability = capabilities.data;
  const configuration = useWorkbenchConfiguration(
    input,
    multiple,
    busy,
    capability?.native ?? false,
  );
  const { settings, setSettings } = configuration;
  const { align, identifySpeakers, autoUpdateSpeakers, model, device, formats } = settings;
  const language = settings.language ?? "";
  const modelDir = settings.modelDir ?? "";
  const outputDir = settings.outputDir ?? "";
  const vadBundle = settings.vadBundle ?? "";
  const diarizationBundle = settings.diarizationBundle ?? "";
  const translate = settings.translation !== null;
  const targetLanguage = settings.translation?.targetLanguage ?? "de";
  const translationModel = settings.translation?.modelId ?? "";
  const editingDisabled = busy || configuration.pending || !configuration.initialized;
  function setOption<K extends keyof WorkbenchSettings>(key: K, value: WorkbenchSettings[K]) {
    setSettings((settings) => ({ ...settings, [key]: value }));
  }
  function setIdentifySpeakers(checked: boolean) {
    setSettings((settings) => ({
      ...settings,
      identifySpeakers: checked,
      autoUpdateSpeakers: checked && settings.autoUpdateSpeakers,
    }));
  }
  function setTranslate(checked: boolean) {
    setSettings((settings) => {
      const source = settings.language || "en";
      const target = source === "de" ? "en" : "de";
      return {
        ...settings,
        language: checked ? source : settings.language,
        translation: checked
          ? {
              modelId: `Helsinki-NLP/opus-mt-${source}-${target}`,
              sourceLanguage: source,
              targetLanguage: target,
            }
          : null,
      };
    });
  }
  function setLanguage(value: string) {
    setSettings((settings) => ({
      ...settings,
      language: value || null,
      translation: settings.translation ? { ...settings.translation, sourceLanguage: value } : null,
    }));
  }
  function setTranslationField(key: "modelId" | "targetLanguage", value: string) {
    setSettings((settings) => ({
      ...settings,
      translation: settings.translation ? { ...settings.translation, [key]: value } : null,
    }));
  }
  function setFormat(format: WorkbenchFormat, checked: boolean) {
    setSettings((settings) => ({
      ...settings,
      formats: checked
        ? [...settings.formats, format]
        : settings.formats.filter((value) => value !== format),
    }));
  }

  const error =
    configuration.error ??
    capabilities.error ??
    state.error ??
    start.error ??
    queue.error ??
    cancel.error ??
    files.error ??
    result.error;

  return (
    <main className="page workbench">
      <h1>Transcribe</h1>
      {error ? <p role="alert">{error.message}</p> : null}
      {queue.isSuccess ? (
        <p role="status">
          Batch added. <a href="?view=jobs">View jobs</a>
        </p>
      ) : null}
      <WorkbenchPresets configuration={configuration} disabled={editingDisabled} />
      <form
        className="workbenchForm"
        onSubmit={(event) => {
          event.preventDefault();
          const request = { ...settings, input };
          if (multiple) {
            queue.mutate({
              inputs: [
                input,
                ...additionalInputs
                  .split("\n")
                  .map((line) => line.trim())
                  .filter(Boolean),
              ],
              settings: request,
            });
          } else {
            start.mutate(request);
          }
        }}
      >
        <fieldset className="inputOptions" disabled={editingDisabled || !capability}>
          <legend>Input</legend>
          <label>
            <input
              type="checkbox"
              checked={multiple}
              onChange={(event) => setMultiple(event.currentTarget.checked)}
            />
            Process several files or a folder
          </label>
          <label>
            Audio or video file
            <input
              required
              value={input}
              onChange={(event) => setInput(event.currentTarget.value)}
              placeholder="recording.wav"
            />
          </label>
          {multiple ? (
            <>
              <label>
                Additional files or folders (one per line)
                <textarea
                  value={additionalInputs}
                  onChange={(event) => setAdditionalInputs(event.currentTarget.value)}
                />
              </label>
              <p className="muted">
                Folders add supported media in filename order, without subfolders. Up to 100 files
                run sequentially. Outputs use fresh run folders beside each input or inside your
                selected output folder.
              </p>
            </>
          ) : null}
          <button
            type="button"
            onClick={() => {
              setBrowse(!browse);
              if (!browse) {
                files.mutate(".");
              }
            }}
          >
            Browse files
          </button>
          {browse && files.data ? (
            <div className="filePicker" aria-label="Local files">
              <button
                type="button"
                onClick={() =>
                  files.mutate(files.data?.directory ? `${files.data.directory}/..` : ".")
                }
              >
                Parent folder
              </button>
              {multiple ? (
                <button
                  type="button"
                  onClick={() => {
                    setInput(files.data?.directory || ".");
                    setBrowse(false);
                  }}
                >
                  Use this folder
                </button>
              ) : null}
              <ul>
                {files.data.entries.map((entry) => (
                  <li key={entry.path}>
                    <button
                      type="button"
                      onClick={() => {
                        if (entry.directory) {
                          files.mutate(entry.path);
                        } else {
                          setInput(entry.path);
                          setBrowse(false);
                        }
                      }}
                    >
                      {entry.name}
                      {entry.directory ? "/" : ""}
                    </button>
                  </li>
                ))}
              </ul>
              {files.data.truncated ? <p>More files exist. Enter a file path above.</p> : null}
            </div>
          ) : null}
        </fieldset>
        <fieldset className="taskOptions" disabled={editingDisabled || !capability}>
          <legend>Tasks · in order</legend>
          <p>1. Transcribe</p>
          <label>
            Source language
            <input
              list="source-language-choices"
              required={translate}
              value={language}
              onChange={(event) => setLanguage(event.currentTarget.value)}
              placeholder="Auto-detect"
            />
          </label>
          <datalist id="source-language-choices">
            {languageOptions(translate ? translationLanguages : transcriptionLanguages).map(
              ({ code, label }) => (
                <option key={code} value={code}>
                  {label}
                </option>
              ),
            )}
          </datalist>
          <label>
            <input
              type="checkbox"
              checked={align}
              disabled={!capability?.align && !align}
              onChange={(event) => setOption("align", event.currentTarget.checked)}
            />
            Align timestamps
          </label>
          <label>
            <input
              type="checkbox"
              checked={identifySpeakers}
              disabled={!capability?.identifySpeakers && !identifySpeakers}
              onChange={(event) => setIdentifySpeakers(event.currentTarget.checked)}
            />
            Identify speakers
          </label>
          {identifySpeakers ? (
            <label>
              <input
                type="checkbox"
                checked={autoUpdateSpeakers}
                onChange={(event) => setOption("autoUpdateSpeakers", event.currentTarget.checked)}
              />
              Auto-update with new speakers (save compatible voices locally as drafts)
            </label>
          ) : null}
          {!capability?.identifySpeakers ? (
            <p className="muted">Speaker identification is unavailable in this build.</p>
          ) : null}
          <label>
            <input
              type="checkbox"
              checked={translate}
              disabled={!capability?.translate && !translate}
              onChange={(event) => setTranslate(event.currentTarget.checked)}
            />
            Translate
          </label>
          {translate ? (
            <div className="settingsRow taskLanguages">
              <label>
                Target language
                <select
                  required
                  value={targetLanguage}
                  onChange={(event) =>
                    setTranslationField("targetLanguage", event.currentTarget.value)
                  }
                >
                  {languageOptions(translationLanguages).map(({ code, label }) => (
                    <option key={code} value={code}>
                      {label}
                    </option>
                  ))}
                </select>
              </label>
            </div>
          ) : null}
          {!speakerIndex ? (
            <p>
              Active Speaker Directory:{" "}
              <span className="mono">{capability?.speakerDirectory ?? "Loading…"}</span>
            </p>
          ) : null}
        </fieldset>
        <fieldset className="outputOptions" disabled={editingDisabled || !capability}>
          <legend>Output</legend>
          <p className="muted">
            A canonical Native JSON result is also saved for Review &amp; Edit.
          </p>
          <div className="formatChoices">
            {outputFormats.map((format) => (
              <label key={format}>
                <input
                  type="checkbox"
                  checked={formats.includes(format)}
                  onChange={(event) => setFormat(format, event.currentTarget.checked)}
                />
                {format.toUpperCase()}
              </label>
            ))}
          </div>
        </fieldset>
        <details className="advancedOptions">
          <summary>Advanced settings</summary>
          <div className="settingsRow">
            <label>
              Output folder
              <input
                value={outputDir}
                onChange={(event) => setOption("outputDir", event.currentTarget.value || null)}
                placeholder="Beside input"
              />
            </label>
            {capability ? (
              <p className="muted">Files and output folders in {capability.root}</p>
            ) : null}
            <p className="muted">Models must already be cached; this app does not download them.</p>

            {translate ? (
              <label>
                Translation model
                <input
                  disabled={editingDisabled}
                  required
                  value={translationModel}
                  onChange={(event) => setTranslationField("modelId", event.currentTarget.value)}
                />
              </label>
            ) : null}
            <label>
              Model cache folder
              <input
                disabled={editingDisabled}
                value={modelDir}
                onChange={(event) => setOption("modelDir", event.currentTarget.value || null)}
                placeholder="Standard model cache"
              />
            </label>
            <>
              <label>
                Verified VAD bundle
                <input
                  disabled={editingDisabled}
                  value={vadBundle}
                  onChange={(event) => setOption("vadBundle", event.currentTarget.value || null)}
                  placeholder="Auto-select cached bundle"
                />
              </label>
              <label>
                Verified diarization bundle
                <input
                  disabled={editingDisabled}
                  value={diarizationBundle}
                  onChange={(event) =>
                    setOption("diarizationBundle", event.currentTarget.value || null)
                  }
                  placeholder="Auto-select cached bundle"
                />
              </label>
            </>
            <label>
              Transcription model
              <input
                disabled={editingDisabled}
                required
                value={model}
                onChange={(event) => setOption("model", event.currentTarget.value)}
              />
            </label>
            <label>
              Device
              <select
                disabled={editingDisabled}
                value={device}
                onChange={(event) =>
                  setOption("device", parseWorkbenchDevice(event.currentTarget.value))
                }
              >
                <option value="auto">Automatic</option>
                <option value="cpu">CPU</option>
                <option value="cuda" disabled={!capability?.cuda}>
                  CUDA{capability?.cuda ? "" : " (unavailable)"}
                </option>
              </select>
            </label>
          </div>
        </details>
        <section aria-label="Resource preflight" className="resourcePreflight">
          {configuration.preflight.isFetching ? (
            <p role="status">Checking local resources…</p>
          ) : configuration.preflight.data?.ready ? (
            <p className="muted">
              Local resource checks passed. Model execution is validated when the run starts.
            </p>
          ) : null}
          {configuration.preflight.data && !configuration.preflight.data.ready ? (
            <div role="alert">
              <ul>
                {configuration.preflight.data.checks
                  .filter((check) => !check.available)
                  .map((check) => (
                    <li key={check.resource}>
                      {check.resource}: {check.message}
                    </li>
                  ))}
              </ul>
              <button
                type="button"
                disabled={configuration.preflight.isFetching || editingDisabled}
                onClick={() => void configuration.preflight.refetch()}
              >
                Recheck resources
              </button>
            </div>
          ) : null}
        </section>
        <button
          className="startRun"
          type="submit"
          disabled={
            editingDisabled ||
            !capability?.native ||
            !input.trim() ||
            formats.length === 0 ||
            !configuration.preflight.data?.ready ||
            configuration.preflight.isFetching
          }
        >
          Start
        </button>
        {!capability?.native && capability ? (
          <p role="alert">Native transcription is unavailable in this build.</p>
        ) : null}
      </form>
      {speakerIndex}
      {state.data && state.data.status !== "idle" ? (
        <section aria-label="Current transcription">
          <h2>{state.data.status}</h2>
          {state.data.input ? <p className="mono">{state.data.input}</p> : null}
          {state.data.currentTask ? (
            <p role="status">{taskLabels[state.data.currentTask] ?? state.data.currentTask}</p>
          ) : null}
          {state.data.activity && busy ? <p>{state.data.activity}</p> : null}
          <ol>
            {Object.keys(taskLabels)
              .filter((task) => state.data?.tasks[task])
              .map((task) => (
                <li key={task}>
                  {taskLabels[task] ?? task}:{" "}
                  {taskStates[state.data?.tasks[task]?.status ?? ""] ?? "Unknown"}
                </li>
              ))}
          </ol>
          {state.data.error ? <p role="alert">{state.data.error}</p> : null}
          {busy ? (
            <button
              type="button"
              disabled={cancel.isPending || state.data.status === "cancelling"}
              onClick={() => cancel.mutate()}
            >
              Cancel
            </button>
          ) : null}
          {state.data.status === "cancelling" ? <p>Stopping at the next safe boundary.</p> : null}
          {state.data.status === "completed"
            ? state.data.outputs.map((output, index) => (
                <p key={output.path}>
                  <span className="mono">{output.path}</span>{" "}
                  <button type="button" onClick={() => result.mutate(index)}>
                    {index === 0 ? "Open result" : `Open ${output.format.toUpperCase()}`}
                  </button>
                </p>
              ))
            : null}
        </section>
      ) : null}
      {state.data?.status === "completed" && state.data.reviewId ? (
        <p>
          <a href={`?view=review&result=${encodeURIComponent(state.data.reviewId)}`}>
            Review &amp; Edit this result
          </a>
        </p>
      ) : null}
      {result.data ? (
        <section aria-label="Transcript result">
          <h2>Result</h2>
          <TimedTranscript transcript={result.data.transcript} fallbackText={result.data.text} />
          <details>
            <summary>{result.data.path}</summary>
            <pre>{result.data.content}</pre>
          </details>
        </section>
      ) : null}
    </main>
  );
}
