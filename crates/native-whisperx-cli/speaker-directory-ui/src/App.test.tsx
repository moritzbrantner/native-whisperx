import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { App } from "./App";
import {
  httpSpeakerDirectoryApi,
  type SpeakerDirectoryApi,
  type SpeakerDirectoryState,
  type SpeakerProfileEdit,
} from "./api";

const baseState: SpeakerDirectoryState = {
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
              {
                startSeconds: 5,
                endSeconds: 7.5,
                snippet: "Thanks for listening.",
              },
            ],
          },
        ],
      },
      {
        kind: "anonymous",
        anonymousLabel: "speaker_1",
        files: [
          {
            sourceFile: "/workspace/aside.json",
            segmentCount: 1,
            totalDuration: 1.25,
            spans: [
              {
                snippet: "Unknown background speaker.",
              },
            ],
          },
        ],
      },
    ],
    errors: [
      {
        sourceFile: "/workspace/broken.json",
        message: "expected value at line 1 column 1",
      },
    ],
  },
};

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  delete window.nativeWhisperxSessionToken;
});

describe("Speaker Directory UI", () => {
  it("renders Speaker Directory and Speaker Library summary state", async () => {
    render(<App api={createApi()} />);

    expect(await screen.findByRole("heading", { name: "Speaker Directory" })).toBeInTheDocument();
    expect(screen.getByText("/workspace/.native-whisperx/speakers")).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "Speaker Library" })).toBeInTheDocument();
    expect(screen.getByText("1 profile")).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "Scope" })).toBeInTheDocument();
    expect(screen.getByText("local")).toBeInTheDocument();
  });

  it("renders Speaker Trace groups, anonymous labels, files, spans, snippets, and malformed JSON errors", async () => {
    render(<App api={createApi()} />);

    expect(
      await screen.findByRole("heading", { name: "Enrolled Speaker Trace" }),
    ).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "Anonymous Speaker Labels" })).toBeInTheDocument();
    expect(screen.getAllByText("Speaker Library profile").length).toBeGreaterThan(0);
    expect(screen.getByText("speaker_1")).toBeInTheDocument();
    expect(screen.getByText("Trace data only")).toBeInTheDocument();
    expect(screen.getByText("/workspace/interview.json")).toBeInTheDocument();
    expect(screen.getByText("2 segments")).toBeInTheDocument();
    expect(screen.getByText("7.50 seconds")).toBeInTheDocument();
    expect(screen.getByText("0.00s - 2.40s")).toBeInTheDocument();
    expect(screen.getByText("Welcome to the native-whisperx demo.")).toBeInTheDocument();
    expect(screen.getByText("Thanks for listening.")).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "Malformed transcript JSON" })).toBeInTheDocument();
    expect(screen.getByText("/workspace/broken.json")).toBeInTheDocument();
    expect(screen.getByText("expected value at line 1 column 1")).toBeInTheDocument();
  });

  it("renders Speaker Library profiles with distinct stable ids, labels, and metadata", async () => {
    render(<App api={createApi()} />);

    expect((await screen.findAllByRole("heading", { name: "Speaker A" })).length).toBeGreaterThan(
      0,
    );
    expect(screen.getAllByText("Stable profile id").length).toBeGreaterThan(0);
    expect(
      screen.getAllByText("speaker-a").some((element) => element.classList.contains("profileId")),
    ).toBe(true);
    expect(screen.getByText("status")).toBeInTheDocument();
    expect(screen.getByText("confirmed")).toBeInTheDocument();
    expect(screen.getByText("note")).toBeInTheDocument();
    expect(screen.getByText("fixture")).toBeInTheDocument();
  });

  it("saves Speaker Library profile label and metadata edits", async () => {
    const updateProfile = vi.fn(async (_profileId: string, edit: SpeakerProfileEdit) => ({
      ...baseState,
      profiles: [{ id: "speaker-a", label: edit.label ?? "", metadata: edit.metadata ?? {} }],
    }));
    render(<App api={createApi({ updateProfile })} />);

    fireEvent.change(await screen.findByLabelText("speaker-a label"), {
      target: { value: "Interviewer" },
    });
    fireEvent.change(screen.getByLabelText("speaker-a metadata"), {
      target: { value: "status=confirmed\nrole=host" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Save profile" }));

    await waitFor(() =>
      expect(updateProfile).toHaveBeenCalledWith("speaker-a", {
        id: "speaker-a",
        label: "Interviewer",
        metadata: { status: "confirmed", role: "host" },
      }),
    );
    expect(await screen.findByRole("heading", { name: "Interviewer" })).toBeInTheDocument();
    expect(screen.getByText("host")).toBeInTheDocument();
  });

  it("rejects malformed Speaker Library metadata locally before saving", async () => {
    const updateProfile = vi.fn();
    render(<App api={createApi({ updateProfile })} />);

    fireEvent.change(await screen.findByLabelText("speaker-a metadata"), {
      target: { value: "status=confirmed\nmalformed" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Save profile" }));

    expect(await screen.findByRole("alert")).toHaveTextContent(
      "Speaker Library profile metadata line 2 must be key=value.",
    );
    expect(updateProfile).not.toHaveBeenCalled();
  });

  it("displays the server error when a Speaker Library profile edit fails", async () => {
    render(
      <App
        api={createApi({
          updateProfile: vi.fn(async () => {
            throw new Error("Speaker Library profile label must not be empty");
          }),
        })}
      />,
    );

    fireEvent.change(await screen.findByLabelText("speaker-a label"), {
      target: { value: "" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Save profile" }));

    expect(await screen.findByRole("alert")).toHaveTextContent(
      "Speaker Library profile label must not be empty",
    );
  });

  it("deletes Speaker Library profiles", async () => {
    const deleteProfile = vi.fn(async () => ({
      ...baseState,
      library: { ...baseState.library, profileCount: 0 },
      profiles: [],
    }));
    render(<App api={createApi({ deleteProfile })} />);

    fireEvent.click(await screen.findByRole("button", { name: "Delete profile" }));

    await waitFor(() => expect(deleteProfile).toHaveBeenCalledWith("speaker-a"));
    await waitFor(() => expect(screen.queryByLabelText("speaker-a label")).toBeNull());
    expect(screen.getByText("0")).toBeInTheDocument();
  });

  it("rebuilds Speaker Trace with an empty scan-root request and displays report stats", async () => {
    const rebuildTrace = vi.fn(async () => ({
      state: {
        ...baseState,
        trace: {
          ...baseState.trace,
          scanRoot: "/workspace",
        },
      },
      report: {
        tracePath: "/workspace/.native-whisperx/speakers/speaker-trace.json",
        trace: {
          version: 1,
          scanRoot: "/workspace",
          speakers: baseState.trace.speakers,
          errors: baseState.trace.errors,
        },
        stats: {
          scannedFiles: 4,
          acceptedEntries: 3,
          ignoredNonJsonFiles: 2,
          malformedJsonErrors: 1,
        },
      },
    }));
    render(<App api={createApi({ rebuildTrace })} />);

    fireEvent.click(await screen.findByRole("button", { name: "Rebuild Speaker Trace" }));

    await waitFor(() => expect(rebuildTrace).toHaveBeenCalledWith({}));
    expect(await screen.findByText("Rebuild report")).toBeInTheDocument();
    expect(screen.getByText("Scanned files")).toBeInTheDocument();
    expect(screen.getByText("4")).toBeInTheDocument();
    expect(screen.getByText("Accepted entries")).toBeInTheDocument();
    expect(screen.getByText("3")).toBeInTheDocument();
    expect(screen.getByText("Ignored non-JSON files")).toBeInTheDocument();
    expect(screen.getAllByText("2").length).toBeGreaterThan(0);
    expect(screen.getByText("Malformed JSON errors")).toBeInTheDocument();
    expect(screen.getAllByText("1").length).toBeGreaterThan(0);
  });

  it("rebuilds Speaker Trace with an explicit scan root", async () => {
    const rebuildTrace = vi.fn(async () => ({
      state: {
        ...baseState,
        trace: {
          ...baseState.trace,
          scanRoot: "/tmp/transcripts",
        },
      },
      report: {
        tracePath: "/workspace/.native-whisperx/speakers/speaker-trace.json",
        trace: {
          version: 1,
          scanRoot: "/tmp/transcripts",
          speakers: baseState.trace.speakers,
          errors: [],
        },
        stats: {
          scannedFiles: 1,
          acceptedEntries: 1,
          ignoredNonJsonFiles: 0,
          malformedJsonErrors: 0,
        },
      },
    }));
    render(<App api={createApi({ rebuildTrace })} />);

    fireEvent.change(await screen.findByLabelText("Trace rebuild scan root"), {
      target: { value: "/tmp/transcripts" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Rebuild Speaker Trace" }));

    await waitFor(() =>
      expect(rebuildTrace).toHaveBeenCalledWith({ scanRoot: "/tmp/transcripts" }),
    );
    expect((await screen.findAllByText("/tmp/transcripts")).length).toBeGreaterThan(0);
  });

  it("displays the server error when Speaker Trace rebuild fails", async () => {
    render(
      <App
        api={createApi({
          rebuildTrace: vi.fn(async () => {
            throw new Error("global Speaker Directory trace rebuild requires a scan root");
          }),
        })}
      />,
    );

    fireEvent.click(await screen.findByRole("button", { name: "Rebuild Speaker Trace" }));

    expect(await screen.findByRole("alert")).toHaveTextContent(
      "global Speaker Directory trace rebuild requires a scan root",
    );
  });

  it("displays the server error when a Speaker Library profile delete fails", async () => {
    render(
      <App
        api={createApi({
          deleteProfile: vi.fn(async () => {
            throw new Error("Speaker Library profile `speaker-a` was not found");
          }),
        })}
      />,
    );

    fireEvent.click(await screen.findByRole("button", { name: "Delete profile" }));

    expect(await screen.findByRole("alert")).toHaveTextContent(
      "Speaker Library profile `speaker-a` was not found",
    );
  });

  it("sends the current Speaker Directory session-token header for mutating requests", async () => {
    window.nativeWhisperxSessionToken = "test-session-token";
    const fetchMock = vi.fn(async (input: RequestInfo | URL) => {
      if (String(input) === "/api/state") {
        return jsonResponse(baseState);
      }
      return jsonResponse({});
    });
    vi.stubGlobal("fetch", fetchMock);

    await httpSpeakerDirectoryApi.updateProfile("speaker-a", {
      id: "speaker-a",
      label: "Speaker A",
      metadata: { status: "confirmed" },
    });
    await httpSpeakerDirectoryApi.deleteProfile("speaker-a");
    await httpSpeakerDirectoryApi.rebuildTrace({ scanRoot: "/tmp/transcripts" });
    await httpSpeakerDirectoryApi.rebuildTrace({});

    expect(fetchMock).toHaveBeenCalledWith(
      "/api/profiles/speaker-a",
      expect.objectContaining({
        headers: expect.objectContaining({
          "X-Native-Whisperx-Session-Token": "test-session-token",
        }),
        method: "PUT",
      }),
    );
    expect(fetchMock).toHaveBeenCalledWith(
      "/api/profiles/speaker-a",
      expect.objectContaining({
        headers: expect.objectContaining({
          "X-Native-Whisperx-Session-Token": "test-session-token",
        }),
        method: "DELETE",
      }),
    );
    expect(fetchMock).toHaveBeenCalledWith(
      "/api/trace/rebuild",
      expect.objectContaining({
        body: JSON.stringify({ scanRoot: "/tmp/transcripts" }),
        headers: expect.objectContaining({
          "X-Native-Whisperx-Session-Token": "test-session-token",
        }),
        method: "POST",
      }),
    );
    expect(fetchMock).toHaveBeenCalledWith(
      "/api/trace/rebuild",
      expect.objectContaining({
        body: JSON.stringify({}),
        headers: expect.objectContaining({
          "X-Native-Whisperx-Session-Token": "test-session-token",
        }),
        method: "POST",
      }),
    );
  });
});

function createApi(overrides: Partial<SpeakerDirectoryApi> = {}): SpeakerDirectoryApi {
  return {
    getState: vi.fn(async () => structuredClone(baseState)),
    updateProfile: vi.fn(async () => structuredClone(baseState)),
    deleteProfile: vi.fn(async () => structuredClone(baseState)),
    createProfile: vi.fn(),
    rebuildTrace: vi.fn(),
    ...overrides,
  };
}

function jsonResponse(value: unknown) {
  return {
    ok: true,
    json: async () => value,
  } as Response;
}
