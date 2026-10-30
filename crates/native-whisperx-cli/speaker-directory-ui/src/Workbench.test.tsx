import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { Workbench } from "./Workbench";

function defaultWorkbenchResponse(path: string): Response | undefined {
  if (path.endsWith("/presets"))
    return new Response(
      JSON.stringify({ presets: [], builtins: [], activeId: null, problem: null, canReset: false }),
    );
  if (path.endsWith("/preflight")) return new Response(JSON.stringify({ ready: true, checks: [] }));
  return undefined;
}

afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
});
it("submits selected tasks once and opens the completed native result", async () => {
  const requests: unknown[] = [];
  vi.stubGlobal(
    "fetch",
    vi.fn(async (path: string, init?: RequestInit) => {
      const configured = defaultWorkbenchResponse(path);
      if (configured) return configured;
      let value: unknown;
      if (path.endsWith("capabilities")) {
        value = {
          native: true,
          cuda: false,
          align: true,
          identifySpeakers: true,
          translate: true,
          root: "/recordings",
          speakerDirectory: "/recordings/speakers",
        };
      } else if (path.endsWith("/run")) {
        requests.push(JSON.parse(String(init?.body)));
        value = {
          status: "completed",
          tasks: {},
          outputs: [{ path: "/recordings/interview.json", format: "json" }],
        };
      } else if (path.endsWith("/result")) {
        value = {
          text: "A completed conversation.",
          path: "/recordings/interview.json",
          content: "{ ... }",
          transcript: {
            language: "en",
            segments: [
              {
                id: "0",
                text: "A completed conversation.",
                startSeconds: 0.125,
                endSeconds: 2.75,
                speaker: "SPEAKER_00",
                wordCount: 0,
                overlapsPrevious: false,
              },
            ],
          },
        };
      } else {
        value = { status: "idle" };
      }
      return new Response(JSON.stringify(value), {
        headers: { "Content-Type": "application/json" },
      });
    }),
  );
  render(
    <QueryClientProvider
      client={new QueryClient({ defaultOptions: { queries: { retry: false } } })}
    >
      <Workbench />
    </QueryClientProvider>,
  );
  await screen.findByText("/recordings/speakers");
  await waitFor(() => expect(screen.getByRole("combobox", { name: "Preset" })).toBeEnabled());
  fireEvent.change(screen.getByLabelText("Audio or video file"), {
    target: { value: "interview.wav" },
  });
  fireEvent.click(screen.getByLabelText("Align timestamps"));
  fireEvent.click(screen.getByLabelText("Diarize speakers (pyannote)"));
  fireEvent.click(screen.getByLabelText("Translate"));
  expect(screen.getByRole("option", { name: "French · fr" })).toBeInTheDocument();
  fireEvent.change(screen.getByLabelText("Source language"), { target: { value: "en" } });
  await waitFor(() => expect(screen.getByRole("button", { name: "Start" })).toBeEnabled());
  fireEvent.click(screen.getByRole("button", { name: "Start" }));
  await screen.findByText("completed");
  expect(requests).toHaveLength(1);
  expect(requests[0]).toMatchObject({
    input: "interview.wav",
    align: true,
    identifySpeakers: true,
    translation: { sourceLanguage: "en", targetLanguage: "de" },
  });
  fireEvent.click(screen.getByRole("button", { name: "Open result" }));
  expect(await screen.findByText("A completed conversation.")).toBeInTheDocument();
  expect(screen.getByText("00:00:00.125 → 00:00:02.750")).toBeInTheDocument();
  expect(screen.getByText("SPEAKER_00")).toBeInTheDocument();
});

it("disables unavailable stages and displays a native failure without success", async () => {
  vi.stubGlobal(
    "fetch",
    vi.fn(async (path: string) => {
      const configured = defaultWorkbenchResponse(path);
      if (configured) return configured;
      if (path.endsWith("/run")) {
        return new Response("required model is unavailable", { status: 400 });
      }
      const value = path.endsWith("capabilities")
        ? {
            native: true,
            cuda: false,
            align: false,
            identifySpeakers: false,
            translate: false,
            root: "/recordings",
            speakerDirectory: "/recordings/speakers",
          }
        : { status: "idle" };
      return new Response(JSON.stringify(value));
    }),
  );
  render(
    <QueryClientProvider
      client={new QueryClient({ defaultOptions: { queries: { retry: false } } })}
    >
      <Workbench />
    </QueryClientProvider>,
  );
  await screen.findByText("/recordings/speakers");
  await waitFor(() => expect(screen.getByRole("combobox", { name: "Preset" })).toBeEnabled());
  expect(screen.getByRole("button", { name: "Start" })).toBeDisabled();
  expect(screen.getByLabelText("Align timestamps")).toBeDisabled();
  expect(screen.getByLabelText("Diarize speakers (pyannote)")).toBeDisabled();
  expect(screen.getByLabelText("Translate")).toBeDisabled();
  fireEvent.change(screen.getByLabelText("Audio or video file"), {
    target: { value: "interview.wav" },
  });
  await waitFor(() => expect(screen.getByRole("button", { name: "Start" })).toBeEnabled());
  fireEvent.click(screen.getByRole("button", { name: "Start" }));
  expect(await screen.findByRole("alert")).toHaveTextContent("required model is unavailable");
  expect(screen.queryByRole("button", { name: "Open result" })).not.toBeInTheDocument();
});

it("queues several inputs with one task selection and exposes the Jobs destination", async () => {
  const queued: unknown[] = [];
  let singleRuns = 0;
  vi.stubGlobal(
    "fetch",
    vi.fn(async (path: string, init?: RequestInit) => {
      const configured = defaultWorkbenchResponse(path);
      if (configured) return configured;
      let value: unknown = { status: "idle" };
      if (path.endsWith("capabilities"))
        value = {
          native: true,
          cuda: false,
          align: true,
          identifySpeakers: true,
          translate: true,
          root: "/recordings",
          speakerDirectory: "/recordings/speakers",
        };
      if (path.endsWith("/queue")) {
        queued.push(JSON.parse(String(init?.body)));
        value = { jobs: [] };
      }
      if (path.endsWith("/run")) singleRuns += 1;
      return new Response(JSON.stringify(value), {
        headers: { "Content-Type": "application/json" },
      });
    }),
  );
  render(
    <QueryClientProvider
      client={new QueryClient({ defaultOptions: { queries: { retry: false } } })}
    >
      <Workbench />
    </QueryClientProvider>,
  );
  await screen.findByText("/recordings/speakers");
  await waitFor(() => expect(screen.getByRole("combobox", { name: "Preset" })).toBeEnabled());
  fireEvent.click(screen.getByLabelText("Process several files or a folder"));
  fireEvent.change(screen.getByLabelText("Audio or video file"), {
    target: { value: "first.wav" },
  });
  fireEvent.change(screen.getByLabelText("Additional files or folders (one per line)"), {
    target: { value: "second.wav\ninterviews" },
  });
  fireEvent.click(screen.getByLabelText("Align timestamps"));
  await waitFor(() => expect(screen.getByRole("button", { name: "Start" })).toBeEnabled());
  fireEvent.click(screen.getByRole("button", { name: "Start" }));
  await screen.findByRole("link", { name: "View jobs" });
  expect(queued).toHaveLength(1);
  expect(queued[0]).toMatchObject({
    inputs: ["first.wav", "second.wav", "interviews"],
    settings: { align: true, identifySpeakers: false },
  });
  expect(singleRuns).toBe(0);
});

it("applies, edits, saves and restores one preset configuration for the next run", async () => {
  const initialSettings = {
    align: false,
    identifySpeakers: true,
    autoUpdateSpeakers: false,
    translation: null,
    model: "base",
    language: null,
    device: "cpu",
    outputDir: null,
    modelDir: null,
    vadBundle: null,
    diarizationBundle: null,
    formats: ["json"],
    speakerDirectory: null,
  };
  let stored: unknown = {
    presets: [],
    activeId: null,
    problem: null,
    canReset: false,
    builtins: [
      {
        id: "builtin:interview",
        supported: true,
        note: "Check local models",
        preset: { name: "Interview", settings: initialSettings },
      },
    ],
  };
  const runs: unknown[] = [];
  const saves: unknown[] = [];
  vi.stubGlobal(
    "fetch",
    vi.fn(async (path: string, init?: RequestInit) => {
      let value: unknown = { status: "idle" };
      if (path.endsWith("capabilities"))
        value = {
          native: true,
          cuda: false,
          align: true,
          identifySpeakers: true,
          translate: true,
          root: "/recordings",
          speakerDirectory: "/recordings/speakers",
        };
      if (path.endsWith("/presets")) value = stored;
      if (path.endsWith("/preflight")) value = { ready: true, checks: [] };
      if (path.endsWith("/preset-select")) value = stored;
      if (path.endsWith("/preset-save")) {
        const body: unknown = JSON.parse(String(init?.body));
        saves.push(body);
        if (body && typeof body === "object" && "preset" in body)
          stored = {
            presets: [{ id: "0".repeat(64), preset: body.preset }],
            activeId: "0".repeat(64),
            problem: null,
            canReset: false,
            builtins: [],
          };
        value = stored;
      }
      if (path.endsWith("/run")) {
        runs.push(JSON.parse(String(init?.body)));
        value = { status: "completed", tasks: {}, outputs: [] };
      }
      return new Response(JSON.stringify(value));
    }),
  );
  const mount = () =>
    render(
      <QueryClientProvider
        client={new QueryClient({ defaultOptions: { queries: { retry: false } } })}
      >
        <Workbench />
      </QueryClientProvider>,
    );
  let view = mount();
  await waitFor(() => expect(screen.getByRole("combobox", { name: "Preset" })).toBeEnabled());
  fireEvent.change(await screen.findByRole("combobox", { name: "Preset" }), {
    target: { value: "builtin:interview" },
  });
  await waitFor(() => expect(screen.getByLabelText("Diarize speakers (pyannote)")).toBeChecked());
  fireEvent.click(screen.getByLabelText("Align timestamps"));
  fireEvent.click(screen.getByText("Advanced settings", { exact: true }));
  fireEvent.change(screen.getByLabelText("Transcription model"), { target: { value: "tiny.en" } });
  fireEvent.click(screen.getByLabelText("SRT"));
  fireEvent.click(screen.getByLabelText("JSON", { exact: true }));
  fireEvent.click(screen.getByText("Save or manage presets", { exact: true }));
  fireEvent.change(screen.getByLabelText("Preset name", { exact: true }), {
    target: { value: "Interview subtitles" },
  });
  fireEvent.click(screen.getByRole("button", { name: "Save preset" }));
  await screen.findByText("Saved preset: Interview subtitles");
  expect(saves).toHaveLength(1);
  expect(JSON.stringify(saves[0])).not.toContain('"input"');
  view.unmount();
  view = mount();
  await screen.findByText("Saved preset: Interview subtitles");
  expect(screen.getByLabelText("Transcription model")).toHaveValue("tiny.en");
  expect(screen.getByLabelText("Align timestamps")).toBeChecked();
  expect(screen.getByLabelText("Diarize speakers (pyannote)")).toBeChecked();
  expect(screen.getByLabelText("SRT")).toBeChecked();
  expect(screen.getByLabelText("JSON", { exact: true })).not.toBeChecked();
  fireEvent.change(screen.getByLabelText("Audio or video file"), {
    target: { value: "interview.wav" },
  });
  await waitFor(() => expect(screen.getByRole("button", { name: "Start" })).toBeEnabled());
  fireEvent.click(screen.getByRole("button", { name: "Start" }));
  await screen.findByText("completed");
  expect(runs).toHaveLength(1);
  expect(runs[0]).toMatchObject({
    input: "interview.wav",
    model: "tiny.en",
    align: true,
    identifySpeakers: true,
    formats: ["srt"],
  });
});

it("shows resource failures before enabling Start and recovers older presets through an explicit backup", async () => {
  let recovered = false;
  const writes: string[] = [];
  vi.stubGlobal(
    "fetch",
    vi.fn(async (path: string, init?: RequestInit) => {
      if (init?.method === "POST") writes.push(path);
      if (path.endsWith("/run")) throw new Error("Unavailable resources must prevent processing");
      let value: unknown = { status: "idle" };
      if (path.endsWith("capabilities"))
        value = {
          native: true,
          cuda: false,
          align: true,
          identifySpeakers: false,
          translate: false,
          root: "/recordings",
          speakerDirectory: "/recordings/speakers",
        };
      if (path.endsWith("/presets") || path.endsWith("/preset-reset")) {
        if (path.endsWith("/preset-reset")) recovered = true;
        value = {
          presets: [],
          builtins: [],
          activeId: null,
          problem: recovered ? null : "Saved preset version is unsupported",
          canReset: !recovered,
          ...(recovered ? { backup: "/recordings/presets-backup.json" } : {}),
        };
      }
      if (path.endsWith("/preflight"))
        value = {
          ready: false,
          checks: [
            {
              resource: "transcription",
              available: false,
              message:
                "Model is missing from the local cache. Select a cached model; downloads are disabled.",
            },
          ],
        };
      return new Response(JSON.stringify(value));
    }),
  );
  render(
    <QueryClientProvider
      client={new QueryClient({ defaultOptions: { queries: { retry: false } } })}
    >
      <Workbench />
    </QueryClientProvider>,
  );
  await screen.findByText("Saved preset version is unsupported");
  fireEvent.change(screen.getByLabelText("Audio or video file"), {
    target: { value: "recording.wav" },
  });
  await screen.findByText(/Model is missing from the local cache/);
  expect(screen.getByRole("button", { name: "Start" })).toBeDisabled();
  fireEvent.click(screen.getByRole("button", { name: "Back up and reset saved presets" }));
  await screen.findByText("Original presets backed up to /recordings/presets-backup.json");
  expect(screen.queryByText("Saved preset version is unsupported")).not.toBeInTheDocument();
  expect(writes.filter((path) => !path.endsWith("/preflight"))).toEqual([
    "/api/workbench/preset-reset",
  ]);
});

it("keeps an explicit preset selection when its referenced index cannot be loaded", async () => {
  const id = "1".repeat(64);
  let activeId: string | null = null;
  const selections: string[] = [];
  const settings = {
    align: false,
    identifySpeakers: true,
    autoUpdateSpeakers: false,
    translation: null,
    model: "small",
    language: null,
    device: "cpu",
    modelDir: null,
    vadBundle: null,
    diarizationBundle: null,
    outputDir: null,
    formats: ["json"],
    speakerDirectory: { scope: "auto", explicitPath: "missing-index" },
  };
  const snapshot = () => ({
    presets: [{ id, preset: { name: "Referenced interview", settings } }],
    builtins: [],
    activeId,
    problem: null,
    canReset: false,
  });
  vi.stubGlobal(
    "fetch",
    vi.fn(async (path: string, init?: RequestInit) => {
      if (path.endsWith("/load"))
        return new Response("Saved speaker index is unavailable", { status: 400 });
      let value: unknown = { status: "idle" };
      if (path.endsWith("capabilities"))
        value = {
          native: true,
          cuda: false,
          align: true,
          identifySpeakers: true,
          translate: false,
          root: "/recordings",
          speakerDirectory: "/recordings/speakers",
        };
      if (path.endsWith("/presets")) value = snapshot();
      if (path.endsWith("/preflight")) value = { ready: false, checks: [] };
      if (path.endsWith("/preset-select")) {
        const body: unknown = JSON.parse(String(init?.body));
        if (body && typeof body === "object" && "id" in body && typeof body.id === "string") {
          activeId = body.id;
          selections.push(body.id);
        }
        value = snapshot();
      }
      return new Response(JSON.stringify(value));
    }),
  );
  render(
    <QueryClientProvider
      client={new QueryClient({ defaultOptions: { queries: { retry: false } } })}
    >
      <Workbench />
    </QueryClientProvider>,
  );
  await waitFor(() => expect(screen.getByRole("combobox", { name: "Preset" })).toBeEnabled());
  fireEvent.change(screen.getByRole("combobox", { name: "Preset" }), { target: { value: id } });
  await screen.findByText("Saved speaker index is unavailable");
  expect(selections).toEqual([id]);
  expect(activeId).toBe(id);
});
