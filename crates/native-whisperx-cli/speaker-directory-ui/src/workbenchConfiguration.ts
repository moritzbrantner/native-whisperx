import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect, useRef, useState } from "react";
import { speakerDirectoryApi } from "./api";
import { speakerLibraryApi } from "./speakerLibraryApi";
import {
  defaultWorkbenchSettings,
  workbenchApi,
  type SavedWorkbenchPreset,
  type WorkbenchSettings,
} from "./workbenchApi";

const presetsKey = ["workbench", "presets"];

export function useWorkbenchConfiguration(
  input: string,
  multiple: boolean,
  busy: boolean,
  enabled: boolean,
) {
  const client = useQueryClient();
  const presets = useQuery({ queryKey: presetsKey, queryFn: workbenchApi.presets, retry: false });
  const directory = useQuery({
    queryKey: ["speaker-directory-state"],
    queryFn: speakerDirectoryApi.getState,
    enabled: false,
  });
  const [settings, setSettings] = useState<WorkbenchSettings>(defaultWorkbenchSettings);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [name, setName] = useState("");
  const [initialized, setInitialized] = useState(false);
  const restoreStarted = useRef(false);
  const choices = [...(presets.data?.builtins ?? []), ...(presets.data?.presets ?? [])];
  const selected = choices.find((choice) => choice.id === selectedId);
  const custom = presets.data?.presets.some((choice) => choice.id === selectedId) ?? false;
  const modified =
    selected != null &&
    (name.trim() !== selected.preset.name ||
      JSON.stringify(settings) !== JSON.stringify(selected.preset.settings));
  const apply = useMutation({
    mutationFn: async (choice: SavedWorkbenchPreset | null) => {
      setSelectedId(choice?.id ?? null);
      setName(choice?.preset.name ?? "");
      if (choice) setSettings(choice.preset.settings);
      const value = await workbenchApi.selectPreset(choice?.id ?? null);
      client.setQueryData(presetsKey, value);
      const reference = choice?.preset.settings.speakerDirectory;
      if (reference && choice?.preset.settings.identifySpeakers) {
        const applied = await speakerLibraryApi.load(
          reference.explicitPath
            ? { scope: "explicit", path: reference.explicitPath }
            : { scope: reference.scope },
        );
        client.setQueryData(["speaker-directory-state"], applied.state);
        void client.invalidateQueries({ queryKey: ["workbench", "preflight"] });
        void client.invalidateQueries({ queryKey: ["workbench", "capabilities"] });
      }
      return value;
    },
  });
  useEffect(() => {
    if (restoreStarted.current || !presets.data || presets.isFetching || busy) return;
    restoreStarted.current = true;
    const active = choices.find((choice) => choice.id === presets.data?.activeId);
    if (active) apply.mutate(active, { onSettled: () => setInitialized(true) });
    else setInitialized(true);
  }, [presets.data, presets.isFetching, busy, choices, apply]);
  const save = useMutation({
    mutationFn: (update: boolean) =>
      workbenchApi.savePreset({
        ...(update && custom && selectedId ? { id: selectedId } : {}),
        preset: { name: name.trim(), settings },
      }),
    onSuccess: (value) => {
      client.setQueryData(presetsKey, value);
      setSelectedId(value.activeId);
      const saved = value.presets.find((choice) => choice.id === value.activeId);
      if (saved) {
        setSettings(saved.preset.settings);
        setName(saved.preset.name);
      }
    },
  });
  const remove = useMutation({
    mutationFn: workbenchApi.deletePreset,
    onSuccess: (value) => {
      client.setQueryData(presetsKey, value);
      setSelectedId(null);
      setName("");
    },
  });
  const recover = useMutation({
    mutationFn: workbenchApi.resetPresets,
    onSuccess: (value) => {
      client.setQueryData(presetsKey, value);
      setSelectedId(null);
      setName("");
    },
  });
  const pending = apply.isPending || save.isPending || remove.isPending || recover.isPending;
  const preflightInput = multiple ? "" : input;
  const preflight = useQuery({
    queryKey: ["workbench", "preflight", preflightInput, settings],
    queryFn: () => workbenchApi.preflight({ ...settings, input: preflightInput }),
    enabled: initialized && enabled && !busy && !pending,
    retry: false,
  });
  return {
    settings,
    setSettings,
    name,
    setName,
    selectedId,
    selected,
    custom,
    modified,
    initialized,
    presets,
    preflight,
    pending,
    directory: directory.data,
    select: (id: string) => apply.mutate(choices.find((choice) => choice.id === id) ?? null),
    save: (update: boolean) => save.mutate(update),
    remove: () => {
      if (custom && selectedId) remove.mutate(selectedId);
    },
    recover: () => recover.mutate(),
    error:
      presets.error ??
      apply.error ??
      save.error ??
      remove.error ??
      recover.error ??
      preflight.error,
    backup: recover.data?.backup,
  };
}
export type WorkbenchConfiguration = ReturnType<typeof useWorkbenchConfiguration>;
