import { type SpeakerDirectoryState } from "./api";
import { parseSpeakerDirectoryState, record } from "./speakerDirectoryState";

export type DirectoryChoice =
  | { scope: "auto" | "local" | "global" }
  | { scope: "explicit"; path: string };
export type LibraryPreviewRequest =
  | { kind: "new"; selection: DirectoryChoice }
  | { kind: "export"; destination: string }
  | { kind: "import"; source: string; mode: "merge" | "replace" };
type ExportSummary = {
  source: string;
  destination: string;
  profileCount: number;
  confirmedProfiles: number;
  draftProfiles: number;
  unclassifiedProfiles: number;
};
type ImportSummary = {
  source: string;
  destination: string;
  mode: "merge" | "replace";
  existingProfiles: number;
  incomingProfiles: number;
  confirmedProfiles: number;
  draftProfiles: number;
  unclassifiedProfiles: number;
  addedProfiles: number;
  unchangedProfiles: number;
  replacedProfiles: number;
  removedProfiles: number;
  resultingProfiles: number;
  changed: boolean;
  backupPath: string | null;
};
export type LibraryPreview =
  | {
      id: string;
      kind: "new";
      summary: { directory: string; destination: string; profileCount: number };
    }
  | { id: string; kind: "export"; summary: ExportSummary }
  | { id: string; kind: "import"; summary: ImportSummary };
export type LibraryReceipt = {
  path: string;
  profileCount: number;
  changed: boolean;
  backupPath: string | null;
};
export type LibraryApplied =
  | { kind: "export"; receipt: LibraryReceipt }
  | { kind: "new" | "import"; receipt: LibraryReceipt; state: SpeakerDirectoryState }
  | { kind: "load"; state: SpeakerDirectoryState };

function object(value: unknown): Record<string, unknown> {
  if (!record(value)) {
    throw new Error("Invalid Speaker Library response.");
  }
  return value;
}
function text(value: unknown): string {
  if (typeof value !== "string") {
    throw new Error("Invalid Speaker Library path or identifier.");
  }
  return value;
}
function count(value: unknown): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0) {
    throw new Error("Invalid Speaker Library profile count.");
  }
  return value;
}
function boolean(value: unknown): boolean {
  if (typeof value !== "boolean") {
    throw new Error("Invalid Speaker Library operation state.");
  }
  return value;
}
function optionalPath(value: unknown): string | null {
  return value === null ? null : text(value);
}
function receipt(raw: unknown): LibraryReceipt {
  const value = object(raw);
  return {
    path: text(value.path),
    profileCount: count(value.profileCount),
    changed: boolean(value.changed),
    backupPath: optionalPath(value.backupPath),
  };
}
async function request(path: string, body: unknown): Promise<unknown> {
  const response = await fetch(`/api/library/${path}`, {
    method: "POST",
    signal: AbortSignal.timeout(30_000),
    headers: {
      "Content-Type": "application/json",
      "X-Native-Whisperx-Session-Token": window.nativeWhisperxSessionToken ?? "",
    },
    body: JSON.stringify(body),
  });
  if (!response.ok) {
    throw new Error(await response.text());
  }
  return response.json();
}
function preview(raw: unknown): LibraryPreview {
  const value = object(raw);
  const summary = object(value.summary);
  const id = text(value.id);
  switch (value.kind) {
    case "new":
      return {
        id,
        kind: "new",
        summary: {
          directory: text(summary.directory),
          destination: text(summary.destination),
          profileCount: count(summary.profileCount),
        },
      };
    case "export":
      return {
        id,
        kind: "export",
        summary: {
          source: text(summary.source),
          destination: text(summary.destination),
          profileCount: count(summary.profileCount),
          confirmedProfiles: count(summary.confirmedProfiles),
          draftProfiles: count(summary.draftProfiles),
          unclassifiedProfiles: count(summary.unclassifiedProfiles),
        },
      };
    case "import": {
      if (summary.mode !== "merge" && summary.mode !== "replace") {
        throw new Error("Unknown Speaker Library import mode.");
      }
      return {
        id,
        kind: "import",
        summary: {
          source: text(summary.source),
          destination: text(summary.destination),
          mode: summary.mode,
          existingProfiles: count(summary.existingProfiles),
          incomingProfiles: count(summary.incomingProfiles),
          confirmedProfiles: count(summary.confirmedProfiles),
          draftProfiles: count(summary.draftProfiles),
          unclassifiedProfiles: count(summary.unclassifiedProfiles),
          addedProfiles: count(summary.addedProfiles),
          unchangedProfiles: count(summary.unchangedProfiles),
          replacedProfiles: count(summary.replacedProfiles),
          removedProfiles: count(summary.removedProfiles),
          resultingProfiles: count(summary.resultingProfiles),
          changed: boolean(summary.changed),
          backupPath: optionalPath(summary.backupPath),
        },
      };
    }
    default:
      throw new Error("Unknown Speaker Library preview.");
  }
}
export const speakerLibraryApi = {
  async preview(body: LibraryPreviewRequest): Promise<LibraryPreview> {
    const result = preview(await request("preview", body));
    if (result.kind !== body.kind) {
      throw new Error("Speaker Library preview does not match the requested operation.");
    }
    return result;
  },
  async confirm(id: string): Promise<LibraryApplied> {
    const value = object(await request("confirm", { id, confirmed: true }));
    switch (value.kind) {
      case "export":
        return { kind: "export", receipt: receipt(value.receipt) };
      case "new":
      case "import":
        return {
          kind: value.kind,
          receipt: receipt(value.receipt),
          state: parseSpeakerDirectoryState(value.state),
        };
      default:
        throw new Error("Unknown Speaker Library confirmation result.");
    }
  },
  async load(selection: DirectoryChoice): Promise<Extract<LibraryApplied, { kind: "load" }>> {
    const value = object(await request("load", { selection }));
    if (value.kind !== "load") {
      throw new Error("Invalid selected Speaker Library.");
    }
    return { kind: "load", state: parseSpeakerDirectoryState(value.state) };
  },
  async cancel(id: string): Promise<void> {
    const value = object(await request("cancel", { id }));
    boolean(value.cancelled);
  },
};
