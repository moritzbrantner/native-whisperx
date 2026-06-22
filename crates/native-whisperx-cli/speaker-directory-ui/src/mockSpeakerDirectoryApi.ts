import type {
  SpeakerDirectoryApi,
  SpeakerDirectoryState,
  SpeakerProfileEdit,
  SpeakerTraceRebuildResponse,
} from "./api";

const initialState: SpeakerDirectoryState = {
  scope: "local",
  path: "/workspace/.native-whisperx/speakers",
  library: {
    path: "/workspace/.native-whisperx/speakers/library.json",
    status: "valid",
    profileCount: 1,
  },
  profiles: [
    {
      id: "speaker-a",
      label: "Speaker A",
      metadata: {
        status: "confirmed",
        note: "fixture",
      },
    },
  ],
  trace: {
    path: "/workspace/.native-whisperx/speakers/speaker-trace.json",
    status: "valid",
    scanRoot: "/workspace",
    speakers: [
      {
        kind: "enrolled",
        profileId: "speaker-a",
        label: "Speaker A",
        files: [
          {
            sourceFile: "/workspace/interview.json",
            segmentCount: 2,
            totalDuration: 7.5,
            spans: [
              {
                startSeconds: 0,
                endSeconds: 2.4,
                snippet: "Welcome to the native-whisperx demo.",
              },
            ],
          },
        ],
      },
      {
        kind: "anonymous",
        anonymousLabel: "speaker_1",
        files: [],
      },
    ],
    errors: [],
  },
};

let state: SpeakerDirectoryState = structuredClone(initialState);

function nextState(update: SpeakerDirectoryState): SpeakerDirectoryState {
  state = structuredClone(update);
  return structuredClone(state);
}

export const mockSpeakerDirectoryApi: SpeakerDirectoryApi = {
  getState: async () => structuredClone(state),
  updateProfile: async (profileId: string, edit: SpeakerProfileEdit) => {
    const profiles = state.profiles.map((profile) =>
      profile.id === profileId
        ? {
            ...profile,
            label: edit.label ?? profile.label,
            metadata: edit.metadata ?? profile.metadata,
          }
        : profile,
    );
    return nextState({
      ...state,
      profiles,
      library: { ...state.library, profileCount: profiles.length },
    });
  },
  deleteProfile: async (profileId: string) => {
    const profiles = state.profiles.filter((profile) => profile.id !== profileId);
    return nextState({
      ...state,
      profiles,
      library: { ...state.library, profileCount: profiles.length },
    });
  },
  createProfile: async () => {
    throw new Error("creating draft Speaker Library profiles without embeddings is not supported");
  },
  rebuildTrace: async (request): Promise<SpeakerTraceRebuildResponse> => {
    const scanRoot = request.scanRoot || state.trace.scanRoot || state.path;
    const updated = nextState({
      ...state,
      trace: {
        ...state.trace,
        scanRoot,
      },
    });
    return {
      state: updated,
      report: {
        tracePath: updated.trace.path,
        trace: {
          version: 1,
          scanRoot,
          speakers: updated.trace.speakers,
          errors: updated.trace.errors,
        },
        stats: {
          scannedFiles: 3,
          acceptedEntries: 2,
          ignoredNonJsonFiles: 1,
          malformedJsonErrors: 0,
        },
      },
    };
  },
};
