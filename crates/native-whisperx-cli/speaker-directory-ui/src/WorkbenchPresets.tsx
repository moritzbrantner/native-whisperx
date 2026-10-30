import { type WorkbenchConfiguration } from "./workbenchConfiguration";

export function WorkbenchPresets({
  configuration,
  disabled,
}: {
  configuration: WorkbenchConfiguration;
  disabled: boolean;
}) {
  const c = configuration;
  const indicator = c.selected
    ? `${c.modified ? "Modified preset" : c.custom ? "Saved preset" : "Preset"}: ${c.selected.preset.name}`
    : "Unsaved configuration";
  return (
    <section aria-label="Presets" className="presetControls">
      <label>
        Preset
        <select
          aria-label="Preset"
          disabled={disabled || c.pending || !c.initialized}
          value={c.selectedId ?? ""}
          onChange={(event) => c.select(event.currentTarget.value)}
        >
          <option value="">Custom configuration</option>
          <optgroup label="Built-in · check local models">
            {c.presets.data?.builtins.map((choice) => (
              <option key={choice.id} value={choice.id} disabled={!choice.supported}>
                {choice.preset.name}
                {choice.supported ? "" : " (unavailable in this build)"}
              </option>
            ))}
          </optgroup>
          <optgroup label="Saved presets">
            {c.presets.data?.presets.map((choice) => (
              <option key={choice.id} value={choice.id}>
                {choice.preset.name}
              </option>
            ))}
          </optgroup>
        </select>
      </label>
      <p className="muted presetIndicator" role="status">
        {indicator}
      </p>
      <details>
        <summary>Save or manage presets</summary>
        <label>
          Preset name
          <input
            aria-label="Preset name"
            value={c.name}
            maxLength={128}
            disabled={disabled || c.pending}
            onChange={(event) => c.setName(event.currentTarget.value)}
          />
        </label>
        <button
          type="button"
          disabled={disabled || c.pending || !c.name.trim() || Boolean(c.presets.data?.problem)}
          onClick={() => c.save(false)}
        >
          Save preset
        </button>
        {c.custom ? (
          <>
            <button
              type="button"
              disabled={disabled || c.pending || !c.modified}
              onClick={() => c.save(true)}
            >
              Update preset
            </button>
            <button type="button" disabled={disabled || c.pending} onClick={c.remove}>
              Delete preset
            </button>
          </>
        ) : null}
        {c.settings.identifySpeakers && c.directory ? (
          <label>
            <input
              type="checkbox"
              checked={c.settings.speakerDirectory !== null}
              disabled={disabled || c.pending}
              onChange={(event) => {
                const checked = event.currentTarget.checked;
                const directory = c.directory;
                c.setSettings((settings) => ({
                  ...settings,
                  speakerDirectory:
                    checked && directory
                      ? {
                          scope: directory.scope === "explicit" ? "auto" : directory.scope,
                          explicitPath: directory.scope === "explicit" ? directory.path : null,
                        }
                      : null,
                }));
              }}
            />
            Remember active speaker index in this preset
          </label>
        ) : null}
        {c.settings.speakerDirectory ? (
          <>
            <p className="muted">
              Preset speaker index:{" "}
              {c.settings.speakerDirectory.explicitPath ?? c.settings.speakerDirectory.scope}
            </p>
            <button
              type="button"
              disabled={disabled || c.pending}
              onClick={() => c.setSettings((settings) => ({ ...settings, speakerDirectory: null }))}
            >
              Use active speaker index instead
            </button>
          </>
        ) : null}
      </details>
      {c.presets.data?.problem ? (
        <div role="alert">
          <p>{c.presets.data.problem}</p>
          {c.presets.data.canReset ? (
            <button type="button" disabled={disabled || c.pending} onClick={c.recover}>
              Back up and reset saved presets
            </button>
          ) : null}
        </div>
      ) : null}
      {c.backup ? <p role="status">Original presets backed up to {c.backup}</p> : null}
    </section>
  );
}
