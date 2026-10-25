import { useState, useEffect } from "react";
import { Label } from "@/components/ui/label";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";

export interface ScanOptionsData {
  includeSubdirectories: boolean;
  includeVideoFiles: boolean;
  minFileSizeBytes: number;
}

export interface ScanOptionsProps {
  options: ScanOptionsData;
  onChange: (options: ScanOptionsData) => void;
}

const DEFAULT_OPTIONS: ScanOptionsData = {
  includeSubdirectories: true,
  includeVideoFiles: true,
  minFileSizeBytes: 10 * 1024,
};

function ScanOptions({
  options = DEFAULT_OPTIONS,
  onChange,
}: ScanOptionsProps) {
  const [includeSubdirectories, setIncludeSubdirectories] =
    useState(options.includeSubdirectories);
  const [includeVideoFiles, setIncludeVideoFiles] = useState(
    options.includeVideoFiles
  );
  const [minFileSizeKB, setMinFileSizeKB] = useState(
    Math.round(options.minFileSizeBytes / 1024)
  );

  useEffect(() => {
    setIncludeSubdirectories(options.includeSubdirectories);
    setIncludeVideoFiles(options.includeVideoFiles);
    setMinFileSizeKB(Math.round(options.minFileSizeBytes / 1024));
  }, [options]);

  const handleSubdirsChange = (checked: boolean) => {
    setIncludeSubdirectories(checked);
    onChange({
      ...options,
      includeSubdirectories: checked,
    });
  };

  const handleVideoChange = (checked: boolean) => {
    setIncludeVideoFiles(checked);
    onChange({
      ...options,
      includeVideoFiles: checked,
    });
  };

  const handleFileSizeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    if (value === "") {
      setMinFileSizeKB(0);
      onChange({
        ...options,
        minFileSizeBytes: 0,
      });
      return;
    }
    const parsed = Number(value);
    if (isNaN(parsed) || parsed < 0) return;
    const kb = Math.round(parsed);
    setMinFileSizeKB(kb);
    onChange({
      ...options,
      minFileSizeBytes: kb * 1024,
    });
  };

  return (
    <div className="space-y-4">
      <h3 className="text-sm font-medium text-zinc-300">
        Scan options
      </h3>

      <div className="flex items-center gap-2">
        <Checkbox
          id="subdirs"
          checked={includeSubdirectories}
          onCheckedChange={handleSubdirsChange}
        />
        <Label htmlFor="subdirs">Scan subdirectories</Label>
      </div>

      <div className="flex items-center gap-2">
        <Checkbox
          id="videos"
          checked={includeVideoFiles}
          onCheckedChange={handleVideoChange}
        />
        <Label htmlFor="videos">Include video files</Label>
      </div>

      <div className="flex items-center gap-2">
        <Label htmlFor="min-size" className="whitespace-nowrap">
          Min file size:
        </Label>
        <Input
          id="min-size"
          type="number"
          min="0"
          step="1"
          value={minFileSizeKB}
          onChange={handleFileSizeChange}
          className="w-24"
        />
        <span className="text-xs text-zinc-500">KB</span>
      </div>
    </div>
  );
}

export default ScanOptions;
export { ScanOptions };
