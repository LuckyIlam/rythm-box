interface MenuBarProps {
  onNew: () => void;
  onOpen: () => void;
  onSave: () => void;
  onSaveAs: () => void;
  onExport: () => void;
  onExit: () => void;
  recentFiles?: string[];
  onOpenRecent?: (name: string) => void;
}

export function MenuBar({ onNew, onOpen, onSave, onSaveAs, onExport, onExit, recentFiles, onOpenRecent }: MenuBarProps) {
  return (
    <div className="flex items-center gap-1 px-2 h-9 bg-neutral-100 dark:bg-neutral-800 border-b border-neutral-300 dark:border-neutral-700 text-sm select-none">
      <div className="relative group">
        <button className="px-2 py-1 rounded hover:bg-neutral-200 dark:hover:bg-neutral-700">File</button>
        <div className="absolute left-0 top-full hidden group-hover:block bg-white dark:bg-neutral-800 border border-neutral-300 dark:border-neutral-700 rounded shadow-lg z-50 min-w-40 py-1">
          <button onClick={onNew} className="w-full text-left px-3 py-1.5 hover:bg-neutral-100 dark:hover:bg-neutral-700">New</button>
          <button onClick={onOpen} className="w-full text-left px-3 py-1.5 hover:bg-neutral-100 dark:hover:bg-neutral-700">Open</button>
          <button onClick={onSave} className="w-full text-left px-3 py-1.5 hover:bg-neutral-100 dark:hover:bg-neutral-700">Save</button>
          <button onClick={onSaveAs} className="w-full text-left px-3 py-1.5 hover:bg-neutral-100 dark:hover:bg-neutral-700">Save As...</button>
          <div className="border-t border-neutral-200 dark:border-neutral-700 my-1" />
          <button onClick={onExport} className="w-full text-left px-3 py-1.5 hover:bg-neutral-100 dark:hover:bg-neutral-700">Export</button>
          {recentFiles && recentFiles.length > 0 && (
            <>
              <div className="border-t border-neutral-200 dark:border-neutral-700 my-1" />
              {recentFiles.map((f) => (
                <button key={f} onClick={() => onOpenRecent?.(f)} className="w-full text-left px-3 py-1.5 hover:bg-neutral-100 dark:hover:bg-neutral-700 text-xs">
                  {f}
                </button>
              ))}
            </>
          )}
          <div className="border-t border-neutral-200 dark:border-neutral-700 my-1" />
          <button onClick={onExit} className="w-full text-left px-3 py-1.5 hover:bg-neutral-100 dark:hover:bg-neutral-700">Exit</button>
        </div>
      </div>
      <div className="relative group">
        <button className="px-2 py-1 rounded hover:bg-neutral-200 dark:hover:bg-neutral-700">Help</button>
        <div className="absolute left-0 top-full hidden group-hover:block bg-white dark:bg-neutral-800 border border-neutral-300 dark:border-neutral-700 rounded shadow-lg z-50 min-w-40 py-1">
          <button className="w-full text-left px-3 py-1.5 hover:bg-neutral-100 dark:hover:bg-neutral-700">About Rhythm Box</button>
        </div>
      </div>
    </div>
  );
}
