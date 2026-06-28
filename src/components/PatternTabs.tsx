import type { Pattern } from '../types';

interface PatternTabsProps {
  patterns: Pattern[];
  activeId: string;
  isPlaying: boolean;
  onSelect: (id: string) => void;
  onAdd: () => void;
  onRemove: (id: string) => void;
}

export function PatternTabs({ patterns, activeId, isPlaying, onSelect, onAdd, onRemove }: PatternTabsProps) {
  return (
    <div className="flex items-center gap-0.5 px-2 pt-1 bg-neutral-50 dark:bg-neutral-900 border-b border-neutral-300 dark:border-neutral-700">
      {patterns.map((pattern) => (
        <div
          key={pattern.id}
          onClick={() => !isPlaying && onSelect(pattern.id)}
          className={`flex items-center gap-1 px-3 py-1.5 text-sm rounded-t border border-b-0 ${
            isPlaying ? 'cursor-default' : 'cursor-pointer'
          } ${
            pattern.id === activeId
              ? 'bg-white dark:bg-neutral-800 border-neutral-300 dark:border-neutral-600 -mb-px'
              : 'bg-neutral-100 dark:bg-neutral-850 border-transparent hover:bg-neutral-200 dark:hover:bg-neutral-750'
          }`}
        >
          <span>{pattern.name}</span>
          {patterns.length > 1 && !isPlaying && (
            <button
              onClick={(e) => {
                e.stopPropagation();
                onRemove(pattern.id);
              }}
              className="ml-1 w-4 h-4 flex items-center justify-center text-xs rounded hover:bg-neutral-300 dark:hover:bg-neutral-600"
            >
              x
            </button>
          )}
        </div>
      ))}
      {!isPlaying && (
        <button
          onClick={onAdd}
          className="px-2 py-1.5 text-sm text-neutral-500 hover:text-neutral-700 dark:hover:text-neutral-300"
          title="Add Pattern"
        >
          +
        </button>
      )}
    </div>
  );
}
