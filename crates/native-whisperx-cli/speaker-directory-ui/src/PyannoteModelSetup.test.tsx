import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { PyannoteModelSetup } from "./PyannoteModelSetup";
const boundary = vi.hoisted(() => ({ load: vi.fn(), archive: vi.fn() }));
vi.mock("../vendor/pyannote-resources.js", () => ({
  loadCommunityDiarizationResources: boundary.load,
  archiveCommunityDiarizationResources: boundary.archive,
}));
afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
  vi.clearAllMocks();
});
it("prepares both model paths without sending or retaining the HF token in app requests", async () => {
  boundary.load.mockResolvedValue({ files: {} });
  boundary.archive.mockResolvedValue(new Uint8Array([80, 75]));
  const fetch = vi.fn(
    async () =>
      new Response(
        JSON.stringify({
          bundlePath: "/root/models/community",
          vadBundlePath: "/root/models/community",
        }),
      ),
  );
  vi.stubGlobal("fetch", fetch);
  const prepared = vi.fn();
  render(
    <QueryClientProvider client={new QueryClient()}>
      <PyannoteModelSetup onPrepared={prepared} />
    </QueryClientProvider>,
  );
  fireEvent.change(screen.getByLabelText("HF_TOKEN"), { target: { value: "hf_private_test" } });
  fireEvent.click(screen.getByRole("button", { name: "Prepare pyannote models" }));
  await waitFor(() =>
    expect(prepared).toHaveBeenCalledWith({
      bundlePath: "/root/models/community",
      vadBundlePath: "/root/models/community",
    }),
  );
  expect(boundary.load.mock.calls[0]?.[0].token).toBe("hf_private_test");
  expect(JSON.stringify(fetch.mock.calls)).not.toContain("hf_private_test");
  expect(screen.getByLabelText("HF_TOKEN")).toHaveValue("");
});
it("cancels model setup without installing or publishing paths, then permits retry", async () => {
  boundary.load.mockImplementationOnce(
    ({ signal }: { signal: AbortSignal }) =>
      new Promise((_resolve, reject) =>
        signal.addEventListener("abort", () => reject(new DOMException("Cancelled", "AbortError"))),
      ),
  );
  const fetch = vi.fn();
  vi.stubGlobal("fetch", fetch);
  const prepared = vi.fn();
  render(
    <QueryClientProvider client={new QueryClient()}>
      <PyannoteModelSetup onPrepared={prepared} />
    </QueryClientProvider>,
  );
  fireEvent.change(screen.getByLabelText("HF_TOKEN"), { target: { value: "hf_cancel" } });
  fireEvent.click(screen.getByRole("button", { name: "Prepare pyannote models" }));
  await screen.findByRole("button", { name: "Cancel model setup" });
  fireEvent.click(screen.getByRole("button", { name: "Cancel model setup" }));
  expect(await screen.findByRole("alert")).toHaveTextContent("Model setup cancelled.");
  expect(fetch).not.toHaveBeenCalled();
  expect(prepared).not.toHaveBeenCalled();
  expect(screen.getByLabelText("HF_TOKEN")).toHaveValue("");
  expect(screen.getByRole("button", { name: "Prepare pyannote models" })).toBeEnabled();
});
