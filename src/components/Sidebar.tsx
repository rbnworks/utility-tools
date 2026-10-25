import { Image as ImageIcon, ChevronRight } from "lucide-react";

interface SidebarProps {
  activeTool: string;
  onToolChange: (tool: string) => void;
}

export function Sidebar({ activeTool, onToolChange }: SidebarProps) {
  const tools = [
    { id: "duplicate-finder", label: "Duplicate Finder", icon: ImageIcon },
  ];

  return (
    <aside className="w-60 h-screen bg-zinc-900 border-r border-zinc-800 flex flex-col">
      {/* Header/Logo Area */}
      <div className="p-4 border-b border-zinc-800">
        <h1 className="text-lg font-semibold text-white">Utility Tools</h1>
        <p className="text-xs text-zinc-400">v0.1.0</p>
      </div>

      {/* Tools Section */}
      <div className="flex-1 py-4">
        <div className="px-4 mb-2">
          <h2 className="text-xs font-medium text-zinc-500 uppercase tracking-wider">
            Tools
          </h2>
        </div>
        <nav className="px-2 space-y-1">
          {tools.map((tool) => {
            const Icon = tool.icon;
            const isActive = activeTool === tool.id;
            return (
              <button
                key={tool.id}
                onClick={() => onToolChange(tool.id)}
                className={`w-full flex items-center gap-3 px-3 py-2 text-sm font-medium rounded-md transition-colors ${
                  isActive
                    ? "bg-zinc-800 text-white"
                    : "text-zinc-400 hover:bg-zinc-800/50 hover:text-zinc-200"
                }`}
              >
                <Icon className="w-4 h-4" />
                <span className="truncate">{tool.label}</span>
              </button>
            );
          })}
        </nav>
      </div>

      {/* Future Tools Section (Expandable) */}
      <div className="border-t border-zinc-800 p-4">
        <div className="flex items-center justify-between text-sm text-zinc-500">
          <span>Future Tools</span>
          <ChevronRight className="w-4 h-4" />
        </div>
        <p className="text-xs text-zinc-600 mt-1">Coming soon</p>
      </div>
    </aside>
  );
}
