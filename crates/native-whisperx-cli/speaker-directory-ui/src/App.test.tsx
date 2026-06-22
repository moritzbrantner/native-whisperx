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
        files: [],
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

  it("renders Speaker Library profiles with distinct stable ids, labels, and metadata", async () => {
    render(<App api={createApi()} />);

    expect((await screen.findAllByRole("heading", { name: "Speaker A" })).length).toBeGreaterThan(
      0,
    );
    expect(screen.getByText("Stable profile id")).toBeInTheDocument();
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

  it("sends the current Speaker Directory session-token header for profile mutations", async () => {
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
