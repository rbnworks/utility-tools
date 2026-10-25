import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { FolderPicker, type FolderEntry } from "@/components/FolderPicker";
import { ScanOptions, type ScanOptionsData } from "@/components/ScanOptions";
import { ScanProgress } from "@/components/ScanProgress";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { Card } from "@/components/ui/card";
import { AlertCircle, Scan } from "lucide-react";

type ViewMode = "input" | "progress";

export interface ScanProgressEvent {
  scanned: number;
  total: number;
  file?: string;
  message?: string;
}

const DEFAULT_OPTIONS: ScanOptionsData = {
  includeSubdirectories: true,
  includeVideoFiles: true,
  minFileSizeBytes: 10240,
};

interface ScanState {
  viewMode: ViewMode;
  folders: FolderEntry[];
  options: ScanOptionsData;
  scanned: number;
  total: number;
  currentFile?: string;
  error?: string;
  completed: boolean;
}

export function DuplicateFinder() {
  const [state, setState] = useState<ScanState>({
    viewMode: "input",
    folders: [],
    options: DEFAULT_OPTIONS,
    scanned: 0,
    total: 0,
    completed: false,
  });

  const handleAddFolder = useCallback((path: string) => {
    const id = `${Date.now()}-${Math.random().toString(36).slice(2, 9)}`;
    setState((prev) => ({
      ...prev,
      folders: [...prev.folders, { id, path }],
    }));
  }, []);

  const handleRemoveFolder = useCallback((id: string) => {
    setState((prev) => ({
      ...prev,
      folders: prev.folders.filter((f) => f.id !== id),
    }));
  }, []);

  const handleClearFolders = useCallback(() => {
    setState((prev) => ({ ...prev, folders: [] }));
  }, []);

  const handleOptionsChange = useCallback((options: ScanOptionsData) => {
    setState((prev) => ({ ...prev, options }));
  }, []);

  const handleScan = useCallback(async () => {
    if (state.folders.length === 0) return;

    setState((prev) => ({
      ...prev,
      viewMode: "progress",
      scanned: 0,
      total: 0,
      currentFile: undefined,
      completed: false,
      error: undefined,
    }));

    try {
      const folderPaths = state.folders.map((f) => f.path);

      await invoke("scan_folders", {
        folderPaths,
        options: {
          includeSubdirectories: state.options.includeSubdirectories,
          includeVideoFiles: state.options.includeVideoFiles,
          minFileSizeBytes: state.options.minFileSizeBytes,
        },
      });

      setState((prev) => ({ ...prev, completed: true }));
    } catch (err) {
      const message = err instanceof Error ? err.message : "Scan failed";
      setState((prev) => ({
        ...prev,
        error: message,
        completed: true,
      }));
    }
  }, [state.folders, state.options]);

  useEffect(() => {
    if (state.viewMode !== "progress") return;

    let unlisten: (() => void) | null = null;

    const setupListener = async () => {
      try {
        unlisten = await listen<ScanProgressEvent>(
          "scan:progress",
          (event) => {
            setState((prev) => ({
              ...prev,
              scanned: event.payload.scanned,
              total: event.payload.total,
              currentFile: event.payload.file,
            }));
          }
        );
      } catch {
        // Event system not available outside Tauri context
      }
    };

    setupListener();

    return () => {
      if (unlisten) unlisten();
    };
  }, [state.viewMode]);

  const handleNewScan = useCallback(() => {
    setState({
      viewMode: "input",
      folders: [],
      options: DEFAULT_OPTIONS,
      scanned: 0,
      total: 0,
      completed: false,
      error: undefined,
      currentFile: undefined,
    });
  }, []);

  return (
    <div className="flex-1 p-6 space-y-6 overflow-y-auto bg-zinc-950">
      <h2 className="text-2xl font-semibold text-white mb-4">
        Duplicate Finder
      </h2>

      {state.viewMode === "input" && (
        <Card className="p-6 bg-zinc-900/50 border-zinc-800 space-y-6">
          <FolderPicker
            folders={state.folders}
            onAdd={handleAddFolder}
            onRemove={handleRemoveFolder}
            onClear={handleClearFolders}
          />

          <Separator />

          <ScanOptions
            options={state.options}
            onChange={handleOptionsChange}
          />

          {state.error && (
            <div className="flex items-center gap-2 text-red-400 text-sm">
              <AlertCircle className="h-4 w-4 flex-shrink-0" />
              {state.error}
            </div>
          )}

          <Button
            onClick={handleScan}
            disabled={state.folders.length === 0}
            className="w-full"
            size="lg"
          >
            <Scan className="mr-2 h-5 w-5" />
            Scan for Duplicates
          </Button>
        </Card>
      )}

      {state.viewMode === "progress" && (
        <ScanProgress
          scanned={state.scanned}
          total={state.total}
          currentFile={state.currentFile}
          completed={state.completed}
          error={state.error}
          onNewScan={handleNewScan}
        />
      )}
    </div>
  );
}

export default DuplicateFinder;
