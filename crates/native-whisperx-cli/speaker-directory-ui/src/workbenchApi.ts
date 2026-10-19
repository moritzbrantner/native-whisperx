export type WorkbenchRequest = {
  input: string;
  align: boolean;
  identifySpeakers: boolean;
  translation: { modelId: string; sourceLanguage: string; targetLanguage: string } | null;
  language: string | null;
  model: string;
  device: string;
  outputDir: string | null;
  modelDir: string | null;
  vadBundle: string | null;
  diarizationBundle: string | null;
  formats: string[];
};
export type WorkbenchJob = {
  status: string;
  currentTask: string | null;
  input: string | null;
  activity: string;
  error: string;
  tasks: Record<string, { status: string }>;
  outputs: { path: string; format: string }[];
};
export type WorkbenchCapabilities = {
  native: boolean;
  align: boolean;
  identifySpeakers: boolean;
  translate: boolean;
  cuda: boolean;
  root: string;
  speakerDirectory: string;
};

function object(value: unknown): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("Invalid local workbench response.");
  }
  return Object.fromEntries(Object.entries(value));
}
function string(value: unknown): string {
  if (typeof value !== "string") {
    throw new Error("Invalid workbench text.");
  }
  return value;
}
function boolean(value: unknown): boolean {
  if (typeof value !== "boolean") {
    throw new Error("Invalid workbench capability.");
  }
  return value;
}
async function request(path: string, body?: unknown): Promise<unknown> {
  const response = await fetch(`/api/workbench/${path}`, {
    method: body === undefined ? "GET" : "POST",
    headers: {
      "X-Native-Whisperx-Session-Token": window.nativeWhisperxSessionToken ?? "",
      ...(body === undefined ? {} : { "Content-Type": "application/json" }),
    },
    ...(body === undefined ? {} : { body: JSON.stringify(body) }),
  });
  if (!response.ok) {
    throw new Error(await response.text());
  }
  return response.json();
}
function job(raw: unknown): WorkbenchJob {
  const value = object(raw);
  const status = string(value.status);
  if (!["idle", "running", "cancelling", "completed", "failed", "cancelled"].includes(status)) {
    throw new Error("Unknown workbench job state.");
  }
  const tasks = Object.fromEntries(
    Object.entries(object(value.tasks ?? {})).map(([key, task]) => [
      key,
      { status: string(object(task).status) },
    ]),
  );
  const outputs = value.outputs ?? [];
  if (!Array.isArray(outputs)) {
    throw new Error("Invalid workbench outputs.");
  }
  return {
    status,
    tasks,
    outputs: outputs.map((raw: unknown) => {
      const output = object(raw);
      return { path: string(output.path), format: string(output.format) };
    }),
    input: value.input == null ? null : string(object(value.input).path),
    currentTask: value.currentTask == null ? null : string(value.currentTask),
    activity: value.activity == null ? "" : string(value.activity),
    error: value.error == null ? "" : string(value.error),
  };
}
export const workbenchApi = {
  async capabilities(): Promise<WorkbenchCapabilities> {
    const value = object(await request("capabilities"));
    return {
      native: boolean(value.native),
      align: boolean(value.align),
      identifySpeakers: boolean(value.identifySpeakers),
      translate: boolean(value.translate),
      cuda: boolean(value.cuda),
      root: string(value.root),
      speakerDirectory: string(value.speakerDirectory),
    };
  },
  async state() {
    return job(await request("state"));
  },
  async run(value: WorkbenchRequest) {
    return job(await request("run", value));
  },
  async cancel() {
    return job(await request("cancel", {}));
  },
  async result(index: number) {
    const value = object(await request("result", { index }));
    return { text: string(value.text), content: string(value.content), path: string(value.path) };
  },
  async files(directory: string) {
    const value = object(await request("files", { directory }));
    if (!Array.isArray(value.entries)) {
      throw new Error("Invalid workbench file list.");
    }
    return {
      directory: string(value.directory),
      truncated: boolean(value.truncated),
      entries: value.entries.map((raw: unknown) => {
        const entry = object(raw);
        return {
          path: string(entry.path),
          name: string(entry.name),
          directory: boolean(entry.directory),
        };
      }),
    };
  },
};
