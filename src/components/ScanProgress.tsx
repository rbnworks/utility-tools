import { Progress } from "@/components/ui/progress";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Card } from "@/components/ui/card";
import { Loader2, CheckCircle2, AlertCircle, RefreshCw } from "lucide-react";

export interface ScanProgressProps {
  scanned: number;
  total: number;
  currentFile?: string;
  completed: boolean;
  error?: string;
  onNewScan: () => void;
}

export function ScanProgress({
  scanned,
  total,
  currentFile,
  completed,
  error,
  onNewScan,
}: ScanProgressProps) {
  const percentage = total > 0 ? Math.round((scanned / total) * 100) : 0;

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h3 className="text-lg font-semibold text-white">
          {completed ? "Scan Complete" : error ? "Scan Error" : "Scanning..."}
        </h3>
        <Badge variant="secondary">
          {completed ? (
            <CheckCircle2 className="mr-1 h-3 w-3 text-green-400" />
          ) : error ? (
            <AlertCircle className="mr-1 h-3 w-3 text-red-400" />
          ) : (
            <Loader2 className="mr-1 h-3 w-3 animate-spin" />
          )}
          {completed ? "Complete" : error ? "Error" : "Scanning"}
        </Badge>
      </div>

      <Progress value={percentage} className="h-2" />
      <p className="text-sm text-zinc-400">
        {scanned} / {total} files
      </p>

      {!completed && currentFile && (
        <p className="text-xs text-zinc-500 truncate">
          Scanning: {currentFile}
        </p>
      )}

      {(completed || error) && (
        <Card className="p-4 bg-zinc-900/50 border-zinc-800">
          <div className="flex items-start gap-3">
            {completed ? (
              <CheckCircle2 className="h-5 w-5 text-green-400 flex-shrink-0 mt-0.5" />
            ) : (
              <AlertCircle className="h-5 w-5 text-red-400 flex-shrink-0 mt-0.5" />
            )}
            <div>
              <p className="text-sm font-medium text-zinc-200">
                {completed ? "Scan complete" : "Scan failed"}
              </p>
              {completed && (
                <p className="text-xs text-zinc-400 mt-1">
                  {total} files scanned across{" "}
                  {scanned > 0 ? scanned : 0} files processed.
                </p>
              )}
              {error && (
                <p className="text-xs text-zinc-400 mt-1">{error}</p>
              )}
            </div>
          </div>
        </Card>
      )}

      {(completed || error) && (
        <Button onClick={onNewScan} className="w-full">
          <RefreshCw className="mr-2 h-4 w-4" />
          New Scan
        </Button>
      )}
    </div>
  );
}

export default ScanProgress;
