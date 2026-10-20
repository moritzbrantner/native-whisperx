import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { jobsApi, type JobStatus } from "./jobsApi";
import { workbenchApi } from "./workbenchApi";

export const jobsKey = ["workbench", "jobs"];
const labels: Record<JobStatus, string> = {
  queued: "Queued",
  running: "Running",
  completed: "Completed",
  failed: "Failed",
  cancelled: "Cancelled",
  notRun: "Not run",
  interrupted: "Interrupted",
};
const tasks: Record<string, string> = {
  decode: "Read media",
  vad: "Detect speech",
  transcribe: "Transcribe",
  align: "Align timestamps",
  identifySpeakers: "Identify speakers",
  translate: "Translate",
  writeOutputs: "Write outputs",
};
function basename(path: string) {
  return path.split(/[\\/]/).at(-1) ?? path;
}
function errorSummary(error: string): string {
  if (error.length <= 180) return error;
  if (/decode|WAV|RIFF/i.test(error))
    return "Could not read this media file. Check its format before retrying.";
  if (/model/i.test(error))
    return "A required model could not be loaded. Check local model resources before retrying.";
  return "Native processing failed. Open error details before retrying.";
}
function JobError({ message }: { message: string }) {
  const summary = errorSummary(message);
  return (
    <>
      <p>{summary}</p>
      {summary !== message ? (
        <details>
          <summary>Error details</summary>
          <pre className="resultContent">{message}</pre>
        </details>
      ) : null}
    </>
  );
}
export function Jobs() {
  const client = useQueryClient();
  const [notice, setNotice] = useState("");
  const [visibleBatches, setVisibleBatches] = useState(20);
  const query = useQuery({
    queryKey: jobsKey,
    queryFn: jobsApi.history,
    retry: false,
    refetchInterval: (query) =>
      query.state.data?.jobs.some(
        (batch) => batch.status === "running" || batch.status === "queued",
      )
        ? 750
        : false,
  });
  const retry = useMutation({
    mutationFn: jobsApi.retry,
    onSuccess: (history) => client.setQueryData(jobsKey, history),
  });
  const cancel = useMutation({
    mutationFn: workbenchApi.cancel,
    onSuccess: () => {
      setNotice("Cancellation requested; waiting for a native safe boundary.");
      void client.invalidateQueries({ queryKey: jobsKey });
    },
  });
  const result = useMutation({ mutationFn: jobsApi.result });
  const busy =
    query.data?.jobs.some((batch) => batch.status === "running" || batch.status === "queued") ??
    false;
  const error = query.error ?? retry.error ?? cancel.error ?? result.error;
  return (
    <main className="page jobsPage">
      <header>
        <h1>Jobs</h1>
        <p>
          Files run sequentially with the task settings recorded below. Completed outputs remain
          available after reopening this local workbench.
        </p>
      </header>
      {error ? <p role="alert">{error.message}</p> : null}
      {notice ? <p role="status">{notice}</p> : null}
      {query.isPending ? <p>Loading jobs…</p> : null}
      {query.data?.jobs.length === 0 ? (
        <p>
          No jobs yet. <a href="?view=transcribe">Select several files or a folder</a> to start a
          batch.
        </p>
      ) : null}
      {busy ? (
        <button type="button" disabled={cancel.isPending} onClick={() => cancel.mutate()}>
          Cancel current batch
        </button>
      ) : null}
      {query.data?.jobs.slice(0, visibleBatches).map((batch) => (
        <section className="jobBatch" key={batch.id} aria-label={`Batch ${batch.id}`}>
          <h2>{labels[batch.status]}</h2>
          <p className="muted">
            {batch.settings.model} · {batch.settings.device} · Transcribe
            {batch.settings.align ? " → Align" : ""}
            {batch.settings.identifySpeakers ? " → Identify speakers" : ""}
            {batch.settings.translate ? " → Translate" : ""}
          </p>
          <p className="path">Speaker index: {batch.speakerDirectory}</p>
          {batch.error && !batch.files.some((file) => file.error) ? (
            <JobError message={batch.error} />
          ) : null}
          <div className="jobsTableScroll">
            <table>
              <thead>
                <tr>
                  <th>File</th>
                  <th>Status / stage</th>
                  <th>Run time</th>
                  <th>Outputs / action</th>
                </tr>
              </thead>
              <tbody>
                {batch.files.map((file, fileIndex) => (
                  <tr key={`${fileIndex}:${file.input}`}>
                    <th scope="row" className="path">
                      {file.input}
                    </th>
                    <td>
                      <span role={file.status === "running" ? "status" : undefined}>
                        {labels[file.status]}
                        {file.currentTask
                          ? ` · ${tasks[file.currentTask] ?? file.currentTask}…`
                          : ""}
                      </span>
                      {file.error ? <JobError message={file.error} /> : null}
                      {Object.keys(file.tasks).length ? (
                        <details>
                          <summary>Task outcomes</summary>
                          <ul>
                            {Object.entries(file.tasks).map(([name, task]) => (
                              <li key={name}>
                                {tasks[name] ?? name}: {labels[task.status]}
                              </li>
                            ))}
                          </ul>
                        </details>
                      ) : null}
                    </td>
                    <td>{file.seconds == null ? "—" : `${file.seconds.toFixed(2)} s`}</td>
                    <td>
                      {file.status === "completed"
                        ? file.outputs.map((output, outputIndex) => (
                            <button
                              type="button"
                              key={`${outputIndex}:${output.path}`}
                              disabled={result.isPending}
                              onClick={() =>
                                result.mutate({ batchId: batch.id, fileIndex, outputIndex })
                              }
                            >
                              Open {basename(output.path)}
                            </button>
                          ))
                        : null}
                      {["failed", "cancelled", "interrupted", "notRun"].includes(file.status) ? (
                        <button
                          type="button"
                          disabled={busy || retry.isPending}
                          onClick={() => {
                            setNotice(`Retry requested for ${basename(file.input)}.`);
                            retry.mutate({ batchId: batch.id, fileIndex });
                          }}
                        >
                          {file.status === "notRun" ? "Run" : "Retry"} {basename(file.input)}
                        </button>
                      ) : null}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
          <p className="muted">
            Retry runs only the selected file with its recorded task settings and a new output
            folder. The original batch stays unchanged.
          </p>
        </section>
      ))}
      {query.data && query.data.jobs.length > visibleBatches ? (
        <button type="button" onClick={() => setVisibleBatches((count) => count + 20)}>
          Show older batches
        </button>
      ) : null}
      {result.data ? (
        <section aria-label="Job result">
          <h2>Result</h2>
          <p className="path">{result.data.path}</p>
          <p>{result.data.text}</p>
          <details>
            <summary>Exact output file</summary>
            <pre className="resultContent">{result.data.content}</pre>
          </details>
        </section>
      ) : null}
    </main>
  );
}
