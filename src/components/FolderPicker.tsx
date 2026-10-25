import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { FolderIcon, X, FolderOpen } from "lucide-react";
import { Separator } from "@/components/ui/separator";
import { cn } from "cn";

export interface FolderEntry {
  id: string;
  path: string;
}

export interface FolderPickerProps {
  folders: FolderEntry[];
  onAdd: (path: string) => void;
  onRemove: (id: string) => void;
  onClear: () => void;
}

function FolderPicker({
  folders,
  onAdd,
  onRemove,
  onClear,
}: FolderPickerProps) {
  const [isPicking, setIsPicking] = useState(false);
  const [path, setPath] = useState("");

  const handlePickFolders = async () => {
    setIsPicking(true);
    try {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const result = await open({ multiple: true, directory: true });
      if (Array.isArray(result)) {
        result.forEach((p: string) => {
          if (p && p.trim()) {
            onAdd(p.trim());
          }
        });
      }
    } catch {
      // Tauri not available in dev mode — falls back to manual entry
    } finally {
      setIsPicking(false);
    }
  };

  const addFolder = () => {
    const trimmed = path.trim();
    if (trimmed) {
      onAdd(trimmed);
      setPath("");
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      addFolder();
    }
  };

  return (
    <div className="space-y-4">
      <h3 className="text-sm font-medium text-zinc-300">
        Folders to scan
      </h3>

      <Button
        onClick={handlePickFolders}
        disabled={isPicking}
        className="w-full"
      >
        {isPicking ? (
          "Opening..."
        ) : (
          <>
            <FolderOpen className="mr-2 h-4 w-4" />
            Pick Folders
          </>
        )}
      </Button>

      <Separator />

      <div className="flex gap-2">
        <Input
          type="text"
          placeholder="Enter folder path..."
          value={path}
          onChange={(e) => setPath(e.target.value)}
          onKeyDown={handleKeyDown}
        />
        <Button onClick={addFolder} variant="secondary">
          Add
        </Button>
      </div>

      {folders.length === 0 ? (
        <p className="text-sm text-zinc-500 italic">
          No folders selected. Use the picker above or type a path manually.
        </p>
      ) : (
        <>
          <div className="space-y-2">
            {folders.map((f) => (
              <div
                key={f.id}
                className={cn(
                  "flex items-center gap-2 rounded-md px-3 py-2",
                  "bg-zinc-900/50"
                )}
              >
                <FolderIcon className="h-4 w-4 flex-shrink-0 text-zinc-400" />
                <span className="flex-1 truncate text-sm text-zinc-300">
                  {f.path}
                </span>
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-6 w-6 flex-shrink-0"
                  onClick={() => onRemove(f.id)}
                >
                  <X className="h-3 w-3" />
                </Button>
              </div>
            ))}
          </div>
          <Button
            variant="ghost"
            size="sm"
            onClick={onClear}
            className="text-zinc-400 hover:text-zinc-300"
          >
            Clear All
          </Button>
        </>
      )}
    </div>
  );
}

export default FolderPicker;
export { FolderPicker };
