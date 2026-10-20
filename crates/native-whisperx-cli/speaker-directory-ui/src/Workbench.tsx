import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect, useState, type ReactNode } from "react";

import { workbenchApi } from "./workbenchApi";
import { jobsApi } from "./jobsApi";
import { jobsKey } from "./Jobs";

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
  const [align, setAlign] = useState(false);
  const [identifySpeakers, setIdentifySpeakers] = useState(false);
  const [autoUpdateSpeakers, setAutoUpdateSpeakers] = useState(false);
  const [translate, setTranslate] = useState(false);
  const [language, setLanguage] = useState("");
  const [targetLanguage, setTargetLanguage] = useState("de");
  const [translationModel, setTranslationModel] = useState("Helsinki-NLP/opus-mt-en-de");
  const [model, setModel] = useState("small");
  const [device, setDevice] = useState("auto");
  const [outputDir, setOutputDir] = useState("");
  const [modelDir, setModelDir] = useState("");
  const [vadBundle, setVadBundle] = useState("");
  const [diarizationBundle, setDiarizationBundle] = useState("");
  const [formats, setFormats] = useState(["json"]);
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
  const error =
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
      {speakerIndex}
      <form
        onSubmit={(event) => {
          event.preventDefault();
          const request = {
            input,
            align,
            identifySpeakers,
            autoUpdateSpeakers: identifySpeakers && autoUpdateSpeakers,
            translation: translate
              ? { modelId: translationModel, sourceLanguage: language, targetLanguage }
              : null,
            language: language || null,
            model,
            device,
            outputDir: outputDir || null,
            modelDir: modelDir || null,
            vadBundle: vadBundle || null,
            diarizationBundle: diarizationBundle || null,
            formats,
          };
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
        <fieldset disabled={busy || !capability}>
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
          {capability ? (
            <p className="muted">Files and output folders in {capability.root}</p>
          ) : null}
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
        <fieldset disabled={busy || !capability}>
          <legend>Tasks · fixed execution order</legend>
          <p>1. Transcribe</p>
          <label>
            <input
              type="checkbox"
              checked={align}
              disabled={!capability?.align}
              onChange={(event) => setAlign(event.currentTarget.checked)}
            />
            Align timestamps
          </label>
          <label>
            <input
              type="checkbox"
              checked={identifySpeakers}
              disabled={!capability?.identifySpeakers}
              onChange={(event) => setIdentifySpeakers(event.currentTarget.checked)}
            />
            Identify speakers
          </label>
          {identifySpeakers ? (
            <label>
              <input
                type="checkbox"
                checked={autoUpdateSpeakers}
                onChange={(event) => setAutoUpdateSpeakers(event.currentTarget.checked)}
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
              disabled={!capability?.translate}
              onChange={(event) => setTranslate(event.currentTarget.checked)}
            />
            Translate
          </label>
          {translate ? (
            <div className="settingsRow">
              <label>
                Source language
                <input
                  required
                  value={language}
                  onChange={(event) => setLanguage(event.currentTarget.value)}
                />
              </label>
              <label>
                Target language
                <input
                  required
                  value={targetLanguage}
                  onChange={(event) => setTargetLanguage(event.currentTarget.value)}
                />
              </label>
              <label>
                Translation model
                <input
                  required
                  value={translationModel}
                  onChange={(event) => setTranslationModel(event.currentTarget.value)}
                />
              </label>
            </div>
          ) : null}
          <p className="muted">
            Outputs are written after the selected tasks. Models must already be cached.
          </p>
          {!speakerIndex ? (
            <p>
              Active Speaker Directory:{" "}
              <span className="mono">{capability?.speakerDirectory ?? "Loading…"}</span>
            </p>
          ) : null}
        </fieldset>
        <fieldset disabled={busy || !capability}>
          <legend>Output</legend>
          <label>
            Output folder
            <input
              value={outputDir}
              onChange={(event) => setOutputDir(event.currentTarget.value)}
              placeholder="Beside input"
            />
          </label>
          <div className="formatChoices">
            {["json", "srt", "vtt", "txt", "tsv"].map((format) => (
              <label key={format}>
                <input
                  type="checkbox"
                  checked={formats.includes(format)}
                  onChange={(event) =>
                    setFormats(
                      event.currentTarget.checked
                        ? [...formats, format]
                        : formats.filter((value) => value !== format),
                    )
                  }
                />
                {format.toUpperCase()}
              </label>
            ))}
          </div>
        </fieldset>
        <details>
          <summary>Model, language and device</summary>
          <div className="settingsRow">
            <label>
              Model cache folder
              <input
                disabled={busy}
                value={modelDir}
                onChange={(event) => setModelDir(event.currentTarget.value)}
                placeholder="Standard model cache"
              />
            </label>
            {identifySpeakers ? (
              <>
                <label>
                  Verified VAD bundle
                  <input
                    disabled={busy}
                    value={vadBundle}
                    onChange={(event) => setVadBundle(event.currentTarget.value)}
                    placeholder="Auto-select cached bundle"
                  />
                </label>
                <label>
                  Verified diarization bundle
                  <input
                    disabled={busy}
                    value={diarizationBundle}
                    onChange={(event) => setDiarizationBundle(event.currentTarget.value)}
                    placeholder="Auto-select cached bundle"
                  />
                </label>
              </>
            ) : null}
            <label>
              Transcription model
              <input
                disabled={busy}
                required
                value={model}
                onChange={(event) => setModel(event.currentTarget.value)}
              />
            </label>
            {!translate ? (
              <label>
                Source language
                <input
                  disabled={busy}
                  required={translate}
                  value={language}
                  onChange={(event) => setLanguage(event.currentTarget.value)}
                  placeholder="Auto-detect"
                />
              </label>
            ) : null}
            <label>
              Device
              <select
                disabled={busy}
                value={device}
                onChange={(event) => setDevice(event.currentTarget.value)}
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
        <button
          type="submit"
          disabled={busy || !capability?.native || !input.trim() || formats.length === 0}
        >
          Start
        </button>
        {!capability?.native && capability ? (
          <p role="alert">Native transcription is unavailable in this build.</p>
        ) : null}
      </form>
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
      {result.data ? (
        <section aria-label="Transcript result">
          <h2>Result</h2>
          <p>{result.data.text}</p>
          <details>
            <summary>{result.data.path}</summary>
            <pre>{result.data.content}</pre>
          </details>
        </section>
      ) : null}
    </main>
  );
}
