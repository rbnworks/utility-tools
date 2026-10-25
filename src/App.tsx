import { useState } from "react";
import { Sidebar } from "@/components/Sidebar";
import { DuplicateFinder } from "@/components/DuplicateFinder";

export function App() {
  const [activeTool, setActiveTool] = useState("duplicate-finder");

  return (
    <div className="flex h-screen bg-zinc-950">
      <Sidebar activeTool={activeTool} onToolChange={setActiveTool} />
      {activeTool === "duplicate-finder" && <DuplicateFinder />}
    </div>
  );
}
