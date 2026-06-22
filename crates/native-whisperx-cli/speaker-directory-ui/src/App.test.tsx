import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { App } from "./App";

describe("Speaker Directory UI", () => {
  it("renders a mocked Speaker Directory summary state", async () => {
    render(<App />);

    expect(await screen.findByRole("heading", { name: "Speaker Directory" })).toBeInTheDocument();
    expect(screen.getByText("Speaker Library")).toBeInTheDocument();
    expect(screen.getAllByText("Speaker Trace").length).toBeGreaterThan(0);
    expect(screen.getAllByText("Anonymous Speaker Label").length).toBeGreaterThan(0);
    expect(screen.getAllByText("Speaker A").length).toBeGreaterThan(0);
    expect(screen.getAllByText("speaker-a").length).toBeGreaterThan(0);
  });
});
