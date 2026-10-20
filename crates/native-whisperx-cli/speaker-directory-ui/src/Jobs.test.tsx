import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { Jobs } from "./Jobs";

afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
});
it("opens a completed row and retries only the explicitly selected failed file", async () => {
  const retries: unknown[] = [];
  const history = {
    jobs: [
      {
        id: "original",
        status: "failed",
        error: "Could not read second.wav",
        speakerDirectory: "/recordings/speakers",
        settings: {
          model: "small",
          device: "cpu",
          align: false,
          identifySpeakers: false,
          translation: null,
        },
        files: [
          {
            input: "/recordings/first.wav",
            status: "completed",
            currentTask: null,
            seconds: 1.2,
            error: null,
            tasks: { transcribe: { status: "completed" } },
            outputs: [{ path: "/recordings/run/first.json", format: "json" }],
          },
          {
            input: "/recordings/second.wav",
            status: "failed",
            currentTask: null,
            seconds: null,
            error: "Invalid media",
            tasks: { decode: { status: "failed" } },
            outputs: [],
          },
          {
            input: "/recordings/third.wav",
            status: "notRun",
            currentTask: null,
            seconds: null,
            error: null,
            tasks: {},
            outputs: [],
          },
        ],
      },
    ],
  };
  vi.stubGlobal(
    "fetch",
    vi.fn(async (path: string, init?: RequestInit) => {
      let value: unknown = history;
      if (path.endsWith("/job-result"))
        value = {
          path: "/recordings/run/first.json",
          content: '{"text":"Completed first file"}',
          text: "Completed first file",
        };
      if (path.endsWith("/retry")) retries.push(JSON.parse(String(init?.body)));
      return new Response(JSON.stringify(value), {
        headers: { "Content-Type": "application/json" },
      });
    }),
  );
  render(
    <QueryClientProvider
      client={new QueryClient({ defaultOptions: { queries: { retry: false } } })}
    >
      <Jobs />
    </QueryClientProvider>,
  );
  await screen.findByText("Invalid media");
  const first = screen.getByRole("row", { name: /first.wav/ });
  expect(within(first).queryByRole("button", { name: /Retry/ })).toBeNull();
  fireEvent.click(within(first).getByRole("button", { name: /Open/ }));
  await screen.findByText("Completed first file");
  fireEvent.click(screen.getByRole("button", { name: "Retry second.wav" }));
  await screen.findByText("Retry requested for second.wav.");
  expect(retries).toEqual([{ batchId: "original", fileIndex: 1 }]);
  expect(screen.getByRole("row", { name: /third.wav/ }).textContent).toContain("Not run");
});

it("requests cancellation while keeping the completed row available", async () => {
  let cancellations = 0;
  const history = {
    jobs: [
      {
        id: "active",
        status: "running",
        error: null,
        speakerDirectory: "/recordings/speakers",
        settings: {
          model: "small",
          device: "cpu",
          align: false,
          identifySpeakers: false,
          translation: null,
        },
        files: [
          {
            input: "/recordings/first.wav",
            status: "completed",
            currentTask: null,
            seconds: 1,
            error: null,
            tasks: {},
            outputs: [{ format: "json", path: "/recordings/run/first.json" }],
          },
          {
            input: "/recordings/second.wav",
            status: "running",
            currentTask: "transcribe",
            seconds: null,
            error: null,
            tasks: { transcribe: { status: "running" } },
            outputs: [],
          },
          {
            input: "/recordings/third.wav",
            status: "queued",
            currentTask: null,
            seconds: null,
            error: null,
            tasks: {},
            outputs: [],
          },
        ],
      },
    ],
  };
  vi.stubGlobal(
    "fetch",
    vi.fn(async (path: string) => {
      if (path.endsWith("/cancel")) {
        cancellations += 1;
        return new Response(JSON.stringify({ status: "cancelling" }));
      }
      return new Response(JSON.stringify(history));
    }),
  );
  render(
    <QueryClientProvider
      client={new QueryClient({ defaultOptions: { queries: { retry: false } } })}
    >
      <Jobs />
    </QueryClientProvider>,
  );
  await screen.findByText("Running · Transcribe…");
  fireEvent.click(screen.getByRole("button", { name: "Cancel current batch" }));
  await screen.findByText("Cancellation requested; waiting for a native safe boundary.");
  expect(cancellations).toBe(1);
  expect(screen.getByRole("button", { name: "Open first.json" })).toBeEnabled();
  expect(screen.queryByRole("button", { name: "Retry first.wav" })).toBeNull();
});
