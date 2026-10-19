import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { Workbench } from "./Workbench";

afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
});
it("submits selected tasks once and opens the completed native result", async () => {
  const requests: unknown[] = [];
  vi.stubGlobal(
    "fetch",
    vi.fn(async (path: string, init?: RequestInit) => {
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
  fireEvent.change(screen.getByLabelText("Audio or video file"), {
    target: { value: "interview.wav" },
  });
  fireEvent.click(screen.getByLabelText("Align timestamps"));
  fireEvent.click(screen.getByLabelText("Identify speakers"));
  fireEvent.click(screen.getByLabelText("Translate"));
  fireEvent.change(screen.getByLabelText("Source language"), { target: { value: "en" } });
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
});

it("disables unavailable stages and displays a native failure without success", async () => {
  vi.stubGlobal(
    "fetch",
    vi.fn(async (path: string) => {
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
  expect(screen.getByRole("button", { name: "Start" })).toBeDisabled();
  expect(screen.getByLabelText("Align timestamps")).toBeDisabled();
  expect(screen.getByLabelText("Identify speakers")).toBeDisabled();
  expect(screen.getByLabelText("Translate")).toBeDisabled();
  fireEvent.change(screen.getByLabelText("Audio or video file"), {
    target: { value: "interview.wav" },
  });
  fireEvent.click(screen.getByRole("button", { name: "Start" }));
  expect(await screen.findByRole("alert")).toHaveTextContent("required model is unavailable");
  expect(screen.queryByRole("button", { name: "Open result" })).not.toBeInTheDocument();
});
