import { mockSpeakerDirectoryApi } from "./mockSpeakerDirectoryApi";

declare const __NATIVE_WHISPERX_REAL_API_BASE_CONFIGURED__: boolean;

export type SpeakerDirectoryStateScope = "local" | "global" | "explicit";
export type ValidationStatus = "valid" | "missing" | "invalid";
export type SpeakerTraceSpeakerKind = "enrolled" | "anonymous";

export interface SpeakerDirectoryState {
  scope: SpeakerDirectoryStateScope;
  path: string;
  library: SpeakerLibraryState;
  profiles: SpeakerProfileState[];
  trace: SpeakerTraceState;
}

export interface SpeakerLibraryState {
  path: string;
  status: ValidationStatus;
  profileCount: number;
  message?: string;
}

export interface SpeakerProfileState {
  id: string;
  label: string;
  metadata: Record<string, string>;
}

export interface SpeakerProfileEdit {
  id?: string;
  label?: string;
  metadata?: Record<string, string>;
}

export interface SpeakerTraceState {
  path: string;
  status: ValidationStatus;
  scanRoot?: string;
  speakers: SpeakerTraceSpeaker[];
  errors: SpeakerTraceError[];
  message?: string;
}

export interface SpeakerTrace {
  version: number;
  scanRoot: string;
  speakers: SpeakerTraceSpeaker[];
  errors: SpeakerTraceError[];
}

export interface SpeakerTraceSpeaker {
  kind: SpeakerTraceSpeakerKind;
  profileId?: string;
  label?: string;
  anonymousLabel?: string;
  files: SpeakerTraceFile[];
}

export interface SpeakerTraceFile {
  sourceFile: string;
  segmentCount: number;
  totalDuration: number;
  spans: SpeakerTraceSpan[];
}

export interface SpeakerTraceSpan {
  startSeconds?: number;
  endSeconds?: number;
  snippet: string;
}

export interface SpeakerTraceError {
  sourceFile: string;
  message: string;
}

export interface SpeakerTraceRebuildRequest {
  scanRoot?: string;
}

export interface SpeakerTraceRebuildStats {
  scannedFiles: number;
  acceptedEntries: number;
  ignoredNonJsonFiles: number;
  malformedJsonErrors: number;
}

export interface SpeakerTraceRebuildReport {
  tracePath: string;
  trace: SpeakerTrace;
  stats: SpeakerTraceRebuildStats;
}

export interface SpeakerTraceRebuildResponse {
  state: SpeakerDirectoryState;
  report: SpeakerTraceRebuildReport;
}

export interface SpeakerDirectoryApi {
  getState(): Promise<SpeakerDirectoryState>;
  updateProfile(profileId: string, edit: SpeakerProfileEdit): Promise<SpeakerDirectoryState>;
  deleteProfile(profileId: string): Promise<SpeakerDirectoryState>;
  createProfile(): Promise<never>;
  rebuildTrace(request: SpeakerTraceRebuildRequest): Promise<SpeakerTraceRebuildResponse>;
}

const SESSION_TOKEN_HEADER = "X-Native-Whisperx-Session-Token";

declare global {
  interface Window {
    nativeWhisperxSessionToken?: string;
  }
}

async function requestJson<T>(path: string, init?: RequestInit): Promise<T> {
  const response = await fetch(path, {
    ...init,
    headers: {
      ...(init?.body ? { "Content-Type": "application/json" } : {}),
      ...(window.nativeWhisperxSessionToken
        ? { [SESSION_TOKEN_HEADER]: window.nativeWhisperxSessionToken }
        : {}),
      ...init?.headers,
    },
  });
  if (!response.ok) {
    throw new Error(await response.text());
  }
  return (await response.json()) as T;
}

export const httpSpeakerDirectoryApi: SpeakerDirectoryApi = {
  getState: () => requestJson<SpeakerDirectoryState>("/api/state"),
  updateProfile: async (profileId, edit) => {
    await requestJson<unknown>(`/api/profiles/${encodeURIComponent(profileId)}`, {
      method: "PUT",
      body: JSON.stringify(edit),
    });
    return httpSpeakerDirectoryApi.getState();
  },
  deleteProfile: async (profileId) => {
    await requestJson<unknown>(`/api/profiles/${encodeURIComponent(profileId)}`, {
      method: "DELETE",
    });
    return httpSpeakerDirectoryApi.getState();
  },
  createProfile: () =>
    requestJson<never>("/api/profiles", {
      method: "POST",
      body: JSON.stringify({}),
    }),
  rebuildTrace: (request) =>
    requestJson<SpeakerTraceRebuildResponse>("/api/trace/rebuild", {
      method: "POST",
      body: JSON.stringify(request.scanRoot ? request : {}),
    }),
};

export const speakerDirectoryApi: SpeakerDirectoryApi =
  import.meta.env.PROD || __NATIVE_WHISPERX_REAL_API_BASE_CONFIGURED__
    ? httpSpeakerDirectoryApi
    : mockSpeakerDirectoryApi;
