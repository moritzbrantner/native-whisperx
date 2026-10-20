export type WorkbenchDevice = "auto" | "cpu" | "cuda";
export type WorkbenchFormat =
  | "json"
  | "native-json"
  | "srt"
  | "vtt"
  | "txt"
  | "tsv"
  | "aud"
  | "all";
export type WorkbenchSpeakerReference = {
  scope: "auto" | "local" | "global";
  explicitPath: string | null;
};
export type WorkbenchSettings = {
  align: boolean;
  identifySpeakers: boolean;
  autoUpdateSpeakers: boolean;
  translation: { modelId: string; sourceLanguage: string; targetLanguage: string } | null;
  model: string;
  language: string | null;
  device: WorkbenchDevice;
  modelDir: string | null;
  vadBundle: string | null;
  diarizationBundle: string | null;
  outputDir: string | null;
  formats: WorkbenchFormat[];
  speakerDirectory: WorkbenchSpeakerReference | null;
};
export type WorkbenchRequest = WorkbenchSettings & { input: string };
export type WorkbenchPreset = { name: string; settings: WorkbenchSettings };
export type SavedWorkbenchPreset = { id: string; preset: WorkbenchPreset };
export type WorkbenchPresets = {
  presets: SavedWorkbenchPreset[];
  activeId: string | null;
  problem: string | null;
  canReset: boolean;
  builtins: (SavedWorkbenchPreset & { supported: boolean; note: string })[];
  backup: string | null;
};
export type WorkbenchPreflight = {
  ready: boolean;
  checks: { resource: string; available: boolean; message: string }[];
};
export function defaultWorkbenchSettings(): WorkbenchSettings {
  return {
    align: false,
    identifySpeakers: false,
    autoUpdateSpeakers: false,
    translation: null,
    model: "small",
    language: null,
    device: "auto",
    modelDir: null,
    vadBundle: null,
    diarizationBundle: null,
    outputDir: null,
    formats: ["json"],
    speakerDirectory: null,
  };
}
export type WorkbenchJob = {
  reviewId: string | null;
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
function nullableString(value: unknown): string | null {
  return value == null ? null : string(value);
}
export function parseWorkbenchDevice(value: unknown): WorkbenchDevice {
  switch (value) {
    case "auto":
    case "cpu":
    case "cuda":
      return value;
    default:
      throw new Error("Invalid saved device choice.");
  }
}
function format(value: unknown): WorkbenchFormat {
  switch (value) {
    case "json":
    case "native-json":
    case "srt":
    case "vtt":
    case "txt":
    case "tsv":
    case "aud":
    case "all":
      return value;
    default:
      throw new Error("Invalid saved output format.");
  }
}
function settings(raw: unknown): WorkbenchSettings {
  const value = object(raw);
  const translation = value.translation == null ? null : object(value.translation);
  const reference = value.speakerDirectory == null ? null : object(value.speakerDirectory);
  let speakerDirectory: WorkbenchSpeakerReference | null = null;
  if (reference) {
    const scope = reference.scope;
    if (scope !== "auto" && scope !== "local" && scope !== "global")
      throw new Error("Invalid saved speaker index scope.");
    speakerDirectory = { scope, explicitPath: nullableString(reference.explicitPath) };
  }
  if (!Array.isArray(value.formats)) throw new Error("Invalid saved output formats.");
  return {
    align: boolean(value.align),
    identifySpeakers: boolean(value.identifySpeakers),
    autoUpdateSpeakers: boolean(value.autoUpdateSpeakers),
    translation: translation
      ? {
          modelId: string(translation.modelId),
          sourceLanguage: string(translation.sourceLanguage),
          targetLanguage: string(translation.targetLanguage),
        }
      : null,
    model: string(value.model),
    language: nullableString(value.language),
    device: parseWorkbenchDevice(value.device),
    modelDir: nullableString(value.modelDir),
    vadBundle: nullableString(value.vadBundle),
    diarizationBundle: nullableString(value.diarizationBundle),
    outputDir: nullableString(value.outputDir),
    formats: value.formats.map(format),
    speakerDirectory,
  };
}
function savedPreset(raw: unknown): SavedWorkbenchPreset {
  const value = object(raw);
  const preset = object(value.preset);
  return {
    id: string(value.id),
    preset: { name: string(preset.name), settings: settings(preset.settings) },
  };
}
function presets(raw: unknown): WorkbenchPresets {
  const value = object(raw);
  if (!Array.isArray(value.presets) || !Array.isArray(value.builtins))
    throw new Error("Invalid saved presets response.");
  return {
    presets: value.presets.map(savedPreset),
    activeId: nullableString(value.activeId),
    problem: nullableString(value.problem),
    canReset: boolean(value.canReset),
    backup: nullableString(value.backup),
    builtins: value.builtins.map((raw: unknown) => {
      const value = object(raw);
      return { ...savedPreset(raw), supported: boolean(value.supported), note: string(value.note) };
    }),
  };
}
function preflight(raw: unknown): WorkbenchPreflight {
  const value = object(raw);
  if (!Array.isArray(value.checks)) throw new Error("Invalid preflight checks.");
  return {
    ready: boolean(value.ready),
    checks: value.checks.map((raw: unknown) => {
      const check = object(raw);
      return {
        resource: string(check.resource),
        available: boolean(check.available),
        message: string(check.message),
      };
    }),
  };
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
    reviewId: value.reviewId == null ? null : string(value.reviewId),
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
  async presets() {
    return presets(await request("presets"));
  },
  async savePreset(value: { id?: string; preset: WorkbenchPreset }) {
    return presets(await request("preset-save", value));
  },
  async selectPreset(id: string | null) {
    return presets(await request("preset-select", { id }));
  },
  async deletePreset(id: string) {
    return presets(await request("preset-delete", { id }));
  },
  async resetPresets() {
    return presets(await request("preset-reset", {}));
  },
  async preflight(value: WorkbenchRequest) {
    return preflight(await request("preflight", value));
  },
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
      root: string(value.root),
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
