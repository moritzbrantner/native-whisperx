import type { WorkbenchRequest } from "./workbenchApi";

export type JobStatus =
  | "queued"
  | "running"
  | "completed"
  | "failed"
  | "cancelled"
  | "notRun"
  | "interrupted";
export type JobRow = {
  input: string;
  status: JobStatus;
  currentTask: string | null;
  seconds: number | null;
  error: string | null;
  outputs: { path: string; format: string }[];
  tasks: Record<string, { status: JobStatus }>;
};
export type JobBatch = {
  id: string;
  status: JobStatus;
  error: string | null;
  speakerDirectory: string;
  settings: {
    model: string;
    device: string;
    align: boolean;
    identifySpeakers: boolean;
    translate: boolean;
  };
  files: JobRow[];
};
export type JobHistory = { jobs: JobBatch[] };
function object(raw: unknown): Record<string, unknown> {
  if (!raw || typeof raw !== "object" || Array.isArray(raw))
    throw new Error("Invalid Jobs response.");
  return Object.fromEntries(Object.entries(raw));
}
function string(raw: unknown): string {
  if (typeof raw !== "string") throw new Error("Invalid Jobs text.");
  return raw;
}
function boolean(raw: unknown): boolean {
  if (typeof raw !== "boolean") throw new Error("Invalid Jobs setting.");
  return raw;
}
function nullableString(raw: unknown): string | null {
  return raw == null ? null : string(raw);
}
function array(raw: unknown): unknown[] {
  if (!Array.isArray(raw)) throw new Error("Invalid Jobs list.");
  return raw;
}
function status(raw: unknown): JobStatus {
  switch (raw) {
    case "queued":
    case "running":
    case "completed":
    case "failed":
    case "cancelled":
    case "notRun":
    case "interrupted":
      return raw;
    default:
      throw new Error("Unknown Jobs status.");
  }
}
function history(raw: unknown): JobHistory {
  return {
    jobs: array(object(raw).jobs).map((raw) => {
      const batch = object(raw);
      const settings = object(batch.settings);
      return {
        id: string(batch.id),
        status: status(batch.status),
        error: nullableString(batch.error),
        speakerDirectory: string(batch.speakerDirectory),
        settings: {
          model: string(settings.model),
          device: string(settings.device),
          align: boolean(settings.align),
          identifySpeakers: boolean(settings.identifySpeakers),
          translate: settings.translation != null,
        },
        files: array(batch.files).map((raw) => {
          const row = object(raw);
          if (
            row.seconds != null &&
            (typeof row.seconds !== "number" || !Number.isFinite(row.seconds) || row.seconds < 0)
          )
            throw new Error("Invalid job duration.");
          return {
            input: string(row.input),
            status: status(row.status),
            currentTask: nullableString(row.currentTask),
            seconds: row.seconds == null ? null : row.seconds,
            error: nullableString(row.error),
            tasks: Object.fromEntries(
              Object.entries(object(row.tasks)).map(([name, task]) => [
                name,
                { status: status(object(task).status) },
              ]),
            ),
            outputs: array(row.outputs).map((raw) => {
              const output = object(raw);
              return { path: string(output.path), format: string(output.format) };
            }),
          };
        }),
      };
    }),
  };
}
async function request(path: string, body?: unknown): Promise<unknown> {
  const response = await fetch(`/api/workbench/${path}`, {
    method: body === undefined ? "GET" : "POST",
    signal: AbortSignal.timeout(30_000),
    headers: {
      "X-Native-Whisperx-Session-Token": window.nativeWhisperxSessionToken ?? "",
      ...(body === undefined ? {} : { "Content-Type": "application/json" }),
    },
    ...(body === undefined ? {} : { body: JSON.stringify(body) }),
  });
  if (!response.ok) throw new Error(await response.text());
  return response.json();
}
export const jobsApi = {
  async history() {
    return history(await request("jobs"));
  },
  async queue(value: { inputs: string[]; settings: WorkbenchRequest }) {
    return history(await request("queue", value));
  },
  async retry(value: { batchId: string; fileIndex: number }) {
    return history(await request("retry", value));
  },
  async result(value: { batchId: string; fileIndex: number; outputIndex: number }) {
    const result = object(await request("job-result", value));
    return {
      path: string(result.path),
      text: string(result.text),
      content: string(result.content),
    };
  },
};
