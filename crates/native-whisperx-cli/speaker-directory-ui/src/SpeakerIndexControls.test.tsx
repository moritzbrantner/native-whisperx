import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";

import { type SpeakerDirectoryApi, type SpeakerDirectoryState } from "./api";
import { SpeakerIndexControls } from "./SpeakerIndexControls";
import { App } from "./App";

const state: SpeakerDirectoryState = {
  scope: "explicit",
  path: "/recordings/speakers",
  library: { path: "/recordings/speakers/library.json", status: "valid", profileCount: 2 },
  profiles: [
    { id: "confirmed-a", label: "Alice", metadata: { status: "confirmed" } },
    { id: "draft-b", label: "Anonymous B", metadata: { status: "draft" } },
  ],
  trace: {
    path: "/recordings/speakers/speaker-trace.json",
    status: "missing",
    speakers: [],
    errors: [],
  },
};
const unexpected = async (): Promise<never> => {
  throw new Error("unexpected profile operation");
};
const api: SpeakerDirectoryApi = {
  getState: async () => structuredClone(state),
  updateProfile: unexpected,
  deleteProfile: unexpected,
  createProfile: unexpected,
  rebuildTrace: unexpected,
};
function renderControls() {
  render(
    <QueryClientProvider
      client={new QueryClient({ defaultOptions: { queries: { retry: false } } })}
    >
      <SpeakerIndexControls api={api} manageHref="?view=speakers" />
    </QueryClientProvider>,
  );
}
beforeEach(() => {
  // Dialog layout/focus is verified in Chromium; these model the browser API in jsdom.
  Object.defineProperty(HTMLDialogElement.prototype, "showModal", {
    configurable: true,
    value: function (this: HTMLDialogElement) {
      this.open = true;
    },
  });
  Object.defineProperty(HTMLDialogElement.prototype, "close", {
    configurable: true,
    value: function (this: HTMLDialogElement) {
      this.open = false;
    },
  });
});
afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
  window.history.replaceState({}, "", "/");
});

it("requires an export preview and explicit voice-data consent before saving", async () => {
  const confirmations: unknown[] = [];
  vi.stubGlobal(
    "fetch",
    vi.fn(async (path: string, init?: RequestInit) => {
      let value: unknown;
      if (path.endsWith("/preview")) {
        value = {
          id: "1",
          kind: "export",
          summary: {
            source: state.library.path,
            destination: "/recordings/portable.json",
            profileCount: 2,
            confirmedProfiles: 1,
            draftProfiles: 1,
            unclassifiedProfiles: 0,
          },
        };
      } else if (path.endsWith("/confirm")) {
        confirmations.push(JSON.parse(String(init?.body)));
        value = {
          kind: "export",
          receipt: {
            path: "/recordings/portable.json",
            profileCount: 2,
            changed: true,
            backupPath: null,
          },
        };
      } else {
        value = { cancelled: true };
      }
      return new Response(JSON.stringify(value));
    }),
  );
  renderControls();
  await screen.findByText(state.library.path);
  fireEvent.click(screen.getByRole("button", { name: "Save…" }));
  fireEvent.change(screen.getByLabelText("Destination file"), {
    target: { value: "portable.json" },
  });
  fireEvent.click(screen.getByRole("button", { name: "Review" }));
  await screen.findByText("/recordings/portable.json");
  expect(confirmations).toHaveLength(0);
  expect(screen.getByRole("button", { name: "Save copy" })).toBeDisabled();
  fireEvent.click(
    screen.getByLabelText("I agree to save voice embeddings locally to this destination"),
  );
  fireEvent.click(screen.getByRole("button", { name: "Save copy" }));
  expect(await screen.findByText("Saved copy: /recordings/portable.json")).toBeInTheDocument();
  expect(confirmations).toEqual([{ id: "1", confirmed: true }]);
});

it("keeps merge errors visible, previews explicit replacement, and cancels without applying", async () => {
  const confirmations: unknown[] = [];
  const cancellations: unknown[] = [];
  vi.stubGlobal(
    "fetch",
    vi.fn(async (path: string, init?: RequestInit) => {
      const body: unknown = JSON.parse(String(init?.body));
      if (
        path.endsWith("/preview") &&
        typeof body === "object" &&
        body !== null &&
        "mode" in body &&
        body.mode === "merge"
      ) {
        return new Response("conflicting stable speaker id; choose explicit replacement", {
          status: 400,
        });
      }
      if (path.endsWith("/confirm")) {
        confirmations.push(body);
      }
      if (path.endsWith("/cancel")) {
        cancellations.push(body);
        return new Response(JSON.stringify({ cancelled: true }));
      }
      return new Response(
        JSON.stringify({
          id: "2",
          kind: "import",
          summary: {
            source: "/recordings/portable.json",
            destination: state.library.path,
            mode: "replace",
            existingProfiles: 2,
            incomingProfiles: 2,
            confirmedProfiles: 1,
            draftProfiles: 1,
            unclassifiedProfiles: 0,
            addedProfiles: 0,
            unchangedProfiles: 1,
            replacedProfiles: 1,
            removedProfiles: 0,
            resultingProfiles: 2,
            changed: true,
            backupPath: "/recordings/speakers/library.backup",
          },
        }),
      );
    }),
  );
  renderControls();
  await screen.findByText(state.library.path);
  fireEvent.click(screen.getByRole("button", { name: "Import…" }));
  fireEvent.change(screen.getByLabelText("Source file"), { target: { value: "portable.json" } });
  fireEvent.click(screen.getByRole("button", { name: "Review" }));
  expect(await screen.findByRole("alert")).toHaveTextContent("conflicting stable speaker id");
  fireEvent.change(screen.getByLabelText("Import mode"), { target: { value: "replace" } });
  fireEvent.click(screen.getByRole("button", { name: "Review" }));
  await screen.findByText("/recordings/speakers/library.backup");
  fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
  await waitFor(() => expect(cancellations).toEqual([{ id: "2" }]));
  expect(confirmations).toHaveLength(0);
  expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
});

it("reviewing an index export never starts the transcription form", async () => {
  window.history.replaceState({}, "", "?view=transcribe");
  const runs: unknown[] = [];
  vi.stubGlobal(
    "fetch",
    vi.fn(async (path: string, init?: RequestInit) => {
      if (path.endsWith("/capabilities")) {
        return new Response(
          JSON.stringify({
            native: true,
            cuda: false,
            align: true,
            identifySpeakers: true,
            translate: true,
            root: "/recordings",
            speakerDirectory: state.path,
          }),
        );
      }
      if (path.endsWith("/run")) {
        runs.push(init?.body);
      }
      if (path.endsWith("/preview")) {
        return new Response(
          JSON.stringify({
            id: "3",
            kind: "export",
            summary: {
              source: state.library.path,
              destination: "/recordings/portable.json",
              profileCount: 2,
              confirmedProfiles: 1,
              draftProfiles: 1,
              unclassifiedProfiles: 0,
            },
          }),
        );
      }
      return new Response(JSON.stringify({ status: "idle" }));
    }),
  );
  render(<App api={api} />);
  await screen.findByText(state.library.path);
  fireEvent.change(screen.getByLabelText("Audio or video file"), {
    target: { value: "recording.wav" },
  });
  fireEvent.click(screen.getByRole("button", { name: "Save…" }));
  fireEvent.change(screen.getByLabelText("Destination file"), {
    target: { value: "portable.json" },
  });
  fireEvent.click(screen.getByRole("button", { name: "Review" }));
  await screen.findByText("/recordings/portable.json");
  expect(runs).toHaveLength(0);
});

it("updates the active canonical library only after confirming New", async () => {
  const next: SpeakerDirectoryState = {
    scope: "explicit",
    path: "/recordings/new-index",
    library: { path: "/recordings/new-index/library.json", status: "valid", profileCount: 0 },
    profiles: [],
    trace: {
      path: "/recordings/new-index/speaker-trace.json",
      status: "missing",
      speakers: [],
      errors: [],
    },
  };
  const confirmations: unknown[] = [];
  vi.stubGlobal(
    "fetch",
    vi.fn(async (path: string, init?: RequestInit) => {
      if (path.endsWith("/preview")) {
        return new Response(
          JSON.stringify({
            id: "4",
            kind: "new",
            summary: { directory: next.path, destination: next.library.path, profileCount: 0 },
          }),
        );
      }
      confirmations.push(JSON.parse(String(init?.body)));
      return new Response(
        JSON.stringify({
          kind: "new",
          state: next,
          receipt: { path: next.library.path, profileCount: 0, changed: true, backupPath: null },
        }),
      );
    }),
  );
  renderControls();
  await screen.findByText(state.library.path);
  fireEvent.click(screen.getByRole("button", { name: "New…" }));
  fireEvent.change(screen.getByLabelText("Directory"), { target: { value: "new-index" } });
  fireEvent.click(screen.getByRole("button", { name: "Review" }));
  await screen.findByText(next.library.path);
  expect(confirmations).toHaveLength(0);
  expect(screen.getByRole("button", { name: "Create empty library" })).toBeDisabled();
  fireEvent.click(screen.getByLabelText("Create this empty Speaker Library"));
  fireEvent.click(screen.getByRole("button", { name: "Create empty library" }));
  await screen.findByText(`Created: ${next.library.path}`);
  expect(screen.getByText(next.library.path)).toBeInTheDocument();
  expect(confirmations).toEqual([{ id: "4", confirmed: true }]);
});

it("replaces editor drafts with the explicitly imported canonical identities", async () => {
  const corrected: SpeakerDirectoryState = structuredClone(state);
  const first = corrected.profiles[0];
  if (!first) {
    throw new Error("fixture profile missing");
  }
  first.label = "Corrected Alice";
  vi.stubGlobal(
    "fetch",
    vi.fn(async (path: string) => {
      if (path.endsWith("/preview")) {
        return new Response(
          JSON.stringify({
            id: "5",
            kind: "import",
            summary: {
              source: "/recordings/portable.json",
              destination: state.library.path,
              mode: "replace",
              existingProfiles: 2,
              incomingProfiles: 2,
              confirmedProfiles: 1,
              draftProfiles: 1,
              unclassifiedProfiles: 0,
              addedProfiles: 0,
              unchangedProfiles: 1,
              replacedProfiles: 1,
              removedProfiles: 0,
              resultingProfiles: 2,
              changed: true,
              backupPath: "/recordings/speakers/library.backup",
            },
          }),
        );
      }
      return new Response(
        JSON.stringify({
          kind: "import",
          state: corrected,
          receipt: {
            path: state.library.path,
            profileCount: 2,
            changed: true,
            backupPath: "/recordings/speakers/library.backup",
          },
        }),
      );
    }),
  );
  render(<App api={api} />);
  expect(await screen.findByLabelText("confirmed-a label")).toHaveValue("Alice");
  fireEvent.click(screen.getByRole("button", { name: "Import…" }));
  fireEvent.change(screen.getByLabelText("Source file"), { target: { value: "portable.json" } });
  fireEvent.change(screen.getByLabelText("Import mode"), { target: { value: "replace" } });
  fireEvent.click(screen.getByRole("button", { name: "Review" }));
  await screen.findByText("/recordings/speakers/library.backup");
  fireEvent.click(
    screen.getByLabelText("I agree to import voice embeddings into this Speaker Library"),
  );
  fireEvent.click(screen.getByRole("button", { name: "Import library" }));
  await screen.findByText(`Imported: ${state.library.path}`);
  expect(screen.getByLabelText("confirmed-a label")).toHaveValue("Corrected Alice");
});
