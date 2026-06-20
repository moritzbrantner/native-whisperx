
pub(crate) const SPEAKER_DIRECTORY_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Speaker Directory</title>
  <style>
    :root {
      color-scheme: light;
      font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
      background: #f5f6f8;
      color: #171b22;
    }
    body {
      margin: 0;
    }
    header {
      background: #ffffff;
      border-bottom: 1px solid #d9dde5;
      padding: 24px 32px 18px;
    }
    main {
      max-width: 1120px;
      margin: 0 auto;
      padding: 24px 24px 40px;
    }
    h1, h2, h3 {
      margin: 0;
      letter-spacing: 0;
    }
    h1 {
      font-size: 28px;
      line-height: 1.2;
    }
    h2 {
      font-size: 18px;
      margin-bottom: 12px;
    }
    h3 {
      font-size: 15px;
      margin-bottom: 8px;
    }
    .summary {
      display: grid;
      grid-template-columns: repeat(3, minmax(0, 1fr));
      gap: 12px;
      margin-top: 18px;
    }
    .metric, .item {
      background: #ffffff;
      border: 1px solid #d9dde5;
      border-radius: 8px;
    }
    .metric {
      padding: 12px 14px;
      min-width: 0;
    }
    .label {
      color: #5b6472;
      font-size: 12px;
      text-transform: uppercase;
    }
    .value {
      font-size: 15px;
      margin-top: 5px;
      overflow-wrap: anywhere;
    }
    .grid {
      display: grid;
      grid-template-columns: minmax(280px, 0.9fr) minmax(320px, 1.1fr);
      gap: 16px;
      align-items: start;
    }
    .panel {
      margin-bottom: 22px;
    }
    .list {
      display: grid;
      gap: 10px;
    }
    .item {
      padding: 12px;
    }
    .row {
      display: flex;
      justify-content: space-between;
      gap: 12px;
      align-items: baseline;
    }
    .mono {
      font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, "Liberation Mono", monospace;
      font-size: 13px;
      overflow-wrap: anywhere;
    }
    .muted {
      color: #5b6472;
    }
    .status {
      display: inline-flex;
      align-items: center;
      border-radius: 999px;
      padding: 3px 9px;
      background: #edf2f7;
      font-size: 12px;
      text-transform: capitalize;
    }
    .valid {
      background: #e7f6ed;
      color: #17663a;
    }
    .invalid {
      background: #ffe9e6;
      color: #9a2516;
    }
    .missing {
      background: #fff4db;
      color: #77510e;
    }
    .metadata, .span {
      margin-top: 8px;
      padding-top: 8px;
      border-top: 1px solid #eceff3;
    }
    .profile-form {
      display: grid;
      gap: 8px;
      margin-top: 10px;
    }
    .trace-controls {
      display: grid;
      gap: 8px;
      margin-bottom: 12px;
    }
    .report {
      display: grid;
      gap: 5px;
      margin-top: 8px;
      font-size: 13px;
    }
    input, textarea {
      width: 100%;
      box-sizing: border-box;
      border: 1px solid #cbd2dc;
      border-radius: 6px;
      padding: 8px 10px;
      font: inherit;
      background: #ffffff;
      color: #171b22;
    }
    textarea {
      min-height: 78px;
      resize: vertical;
      font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, "Liberation Mono", monospace;
      font-size: 13px;
    }
    .actions {
      display: flex;
      gap: 8px;
      flex-wrap: wrap;
    }
    button {
      border: 1px solid #253043;
      border-radius: 6px;
      background: #253043;
      color: #ffffff;
      padding: 7px 10px;
      font: inherit;
      cursor: pointer;
    }
    button.secondary {
      background: #ffffff;
      color: #9a2516;
      border-color: #e0b4ae;
    }
    .error {
      color: #9a2516;
    }
    @media (max-width: 760px) {
      header {
        padding: 20px;
      }
      main {
        padding: 18px;
      }
      .summary, .grid {
        grid-template-columns: 1fr;
      }
      .row {
        display: block;
      }
    }
  </style>
</head>
<body>
  <header>
    <h1>Speaker Directory</h1>
    <div class="summary">
      <div class="metric"><div class="label">Scope</div><div id="scope" class="value">...</div></div>
      <div class="metric"><div class="label">Library</div><div id="library-status" class="value">...</div></div>
      <div class="metric"><div class="label">Trace</div><div id="trace-status" class="value">...</div></div>
    </div>
  </header>
  <main>
    <section class="panel">
      <h2>Backing Path</h2>
      <div id="path" class="mono"></div>
    </section>
    <div class="grid">
      <section class="panel">
        <h2>Profiles</h2>
        <div id="profiles" class="list"></div>
      </section>
      <section class="panel">
        <h2>Speaker Trace</h2>
        <form id="trace-rescan-form" class="trace-controls">
          <input id="trace-scan-root" type="text" placeholder="Scan root" aria-label="Speaker Trace scan root">
          <div class="actions">
            <button type="submit">Rescan</button>
          </div>
          <div id="trace-rescan-error" class="error"></div>
          <div id="trace-rescan-report" class="report muted"></div>
        </form>
        <div id="trace" class="list"></div>
      </section>
    </div>
  </main>
  <script>
    const sessionToken = "__SESSION_TOKEN__";
    const text = (value) => value === undefined || value === null || value === "" ? "none" : String(value);
    const el = (tag, className, value) => {
      const node = document.createElement(tag);
      if (className) node.className = className;
      if (value !== undefined) node.textContent = value;
      return node;
    };
    const status = (value) => {
      const node = el("span", `status ${value}`, value);
      return node;
    };
    const metadataText = (metadata) => {
      const entries = Object.entries(metadata || {});
      if (!entries.length) return "metadata: none";
      return entries.map(([key, value]) => `${key}: ${value}`).join(" | ");
    };
    const metadataLines = (metadata) => Object.entries(metadata || {})
      .map(([key, value]) => `${key}=${value}`)
      .join("\n");
    const parseMetadata = (value) => {
      const metadata = {};
      for (const rawLine of value.split(/\r?\n/)) {
        const line = rawLine.trim();
        if (!line) continue;
        const index = line.indexOf("=");
        if (index <= 0) throw new Error("metadata lines must use key=value");
        metadata[line.slice(0, index).trim()] = line.slice(index + 1).trim();
      }
      return metadata;
    };
    const renderRescanReport = (report) => {
      const node = document.getElementById("trace-rescan-report");
      node.replaceChildren();
      if (!report) return;
      const stats = report.stats || {};
      node.append(el("div", "", `Scanned files: ${stats.scannedFiles ?? 0}`));
      node.append(el("div", "", `Accepted entries: ${stats.acceptedEntries ?? 0}`));
      node.append(el("div", "", `Ignored non-JSON files: ${stats.ignoredNonJsonFiles ?? 0}`));
      node.append(el("div", "", `Malformed JSON errors: ${stats.malformedJsonErrors ?? 0}`));
    };
    const api = async (path, options = {}) => {
      const response = await fetch(path, {
        ...options,
        headers: {
          "Content-Type": "application/json",
          "X-Native-Whisperx-Session-Token": sessionToken,
          ...(options.headers || {})
        }
      });
      if (!response.ok) {
        throw new Error((await response.text()).trim() || response.statusText);
      }
      return response.json();
    };
    const renderProfile = (profiles, profile) => {
      const item = el("article", "item");
      const row = el("div", "row");
      row.append(el("strong", "", text(profile.label)));
      row.append(el("span", "mono muted", text(profile.id)));
      item.append(row);
      item.append(el("div", "metadata muted", metadataText(profile.metadata)));

      const form = el("form", "profile-form");
      const label = document.createElement("input");
      label.value = profile.label || "";
      label.setAttribute("aria-label", "Speaker label");
      const metadata = document.createElement("textarea");
      metadata.value = metadataLines(profile.metadata);
      metadata.setAttribute("aria-label", "Speaker metadata");
      const actions = el("div", "actions");
      const save = document.createElement("button");
      save.type = "submit";
      save.textContent = "Save";
      const remove = document.createElement("button");
      remove.type = "button";
      remove.className = "secondary";
      remove.textContent = "Delete";
      const error = el("div", "error");
      actions.append(save, remove);
      form.append(label, metadata, actions, error);
      form.addEventListener("submit", async (event) => {
        event.preventDefault();
        error.textContent = "";
        try {
          await api(`/api/profiles/${encodeURIComponent(profile.id)}`, {
            method: "PUT",
            body: JSON.stringify({
              id: profile.id,
              label: label.value,
              metadata: parseMetadata(metadata.value)
            })
          });
          await refresh();
        } catch (err) {
          error.textContent = err.message;
        }
      });
      remove.addEventListener("click", async () => {
        error.textContent = "";
        try {
          await api(`/api/profiles/${encodeURIComponent(profile.id)}`, { method: "DELETE" });
          await refresh();
        } catch (err) {
          error.textContent = err.message;
        }
      });
      item.append(form);
      profiles.append(item);
    };
    const render = (state) => {
      document.getElementById("scope").textContent = state.scope;
      document.getElementById("path").textContent = state.path;
      document.getElementById("library-status").replaceChildren(status(state.library.status));
      document.getElementById("trace-status").replaceChildren(status(state.trace.status));

      const profiles = document.getElementById("profiles");
      profiles.replaceChildren();
      if (!state.profiles.length) {
        profiles.append(el("div", "muted", "No enrolled profiles"));
      }
      for (const profile of state.profiles) {
        renderProfile(profiles, profile);
      }

      const trace = document.getElementById("trace");
      trace.replaceChildren();
      if (!state.trace.speakers.length) {
        trace.append(el("div", "muted", state.trace.message || "No trace groups"));
      }
      for (const speaker of state.trace.speakers) {
        const item = el("article", "item");
        item.append(el("h3", "", speaker.label || speaker.anonymousLabel || speaker.profileId));
        item.append(el("div", "mono muted", speaker.profileId || speaker.anonymousLabel));
        for (const file of speaker.files) {
          const fileNode = el("div", "metadata");
          fileNode.append(el("div", "mono", file.sourceFile));
          fileNode.append(el("div", "muted", `${file.segmentCount} segments | ${file.totalDuration.toFixed(3)}s`));
          for (const span of file.spans) {
            const spanNode = el("div", "span muted");
            spanNode.textContent = `${text(span.startSeconds)}-${text(span.endSeconds)}: ${span.snippet}`;
            fileNode.append(spanNode);
          }
          item.append(fileNode);
        }
        trace.append(item);
      }
      for (const traceError of state.trace.errors || []) {
        const item = el("article", "item error");
        item.append(el("strong", "", "Malformed JSON"));
        item.append(el("div", "mono", traceError.sourceFile));
        item.append(el("div", "", traceError.message));
        trace.append(item);
      }
    };
    const refresh = async () => {
      const response = await fetch("/api/state");
      render(await response.json());
    };
    document.getElementById("trace-rescan-form").addEventListener("submit", async (event) => {
      event.preventDefault();
      const error = document.getElementById("trace-rescan-error");
      const scanRoot = document.getElementById("trace-scan-root").value.trim();
      error.textContent = "";
      try {
        const response = await api("/api/trace/rebuild", {
          method: "POST",
          body: JSON.stringify(scanRoot ? { scanRoot } : {})
        });
        render(response.state);
        renderRescanReport(response.report);
      } catch (err) {
        error.textContent = err.message;
      }
    });
    refresh().catch((error) => {
      document.getElementById("trace").textContent = error.message;
    });
  </script>
</body>
</html>
"#;
