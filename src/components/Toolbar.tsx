interface ToolbarProps {
  isPlaying: boolean;
  bpm: number;
  steps: number;
  onPlay: () => void;
  onStop: () => void;
  onSave: () => void;
  onNew: () => void;
  onBpmChange: (bpm: number) => void;
  onStepsChange: (steps: number) => void;
}

import { useEffect, useState } from 'react';

export function Toolbar({
  isPlaying,
  bpm,
  steps,
  onPlay,
  onStop,
  onSave,
  onNew,
  onBpmChange,
  onStepsChange,
}: ToolbarProps) {
  const [bpmInput, setBpmInput] = useState(String(bpm));
  useEffect(() => { setBpmInput(String(bpm)); }, [bpm]);
  return (
    <div className="flex items-center gap-2 px-3 h-12 bg-white dark:bg-neutral-900 border-b border-neutral-300 dark:border-neutral-700">
      <button
        onClick={onNew}
        disabled={isPlaying}
        className={`px-3 py-1.5 text-sm bg-neutral-100 dark:bg-neutral-800 rounded border border-neutral-300 dark:border-neutral-600 ${
          isPlaying ? 'opacity-40 cursor-not-allowed' : 'hover:bg-neutral-200 dark:hover:bg-neutral-700'
        }`}
        title={isPlaying ? 'Stop playback first' : 'New Project'}
      >
        New
      </button>
      <button
        onClick={onSave}
        disabled={isPlaying}
        className={`px-3 py-1.5 text-sm bg-neutral-100 dark:bg-neutral-800 rounded border border-neutral-300 dark:border-neutral-600 ${
          isPlaying ? 'opacity-40 cursor-not-allowed' : 'hover:bg-neutral-200 dark:hover:bg-neutral-700'
        }`}
        title={isPlaying ? 'Stop playback first' : 'Save'}
      >
        Save
      </button>
      <div className="w-px h-6 bg-neutral-300 dark:bg-neutral-600" />
      <button
        onClick={onPlay}
        disabled={isPlaying}
        className={`px-4 py-1.5 text-sm rounded font-medium ${
          isPlaying
            ? 'bg-green-600 text-white opacity-60 cursor-not-allowed'
            : 'bg-neutral-100 dark:bg-neutral-800 hover:bg-neutral-200 dark:hover:bg-neutral-700 border border-neutral-300 dark:border-neutral-600'
        }`}
        title={isPlaying ? 'Playing...' : 'Play'}
      >
        {isPlaying ? 'Playing' : 'Play'}
      </button>
      <button
        onClick={onStop}
        className="px-3 py-1.5 text-sm bg-neutral-100 dark:bg-neutral-800 rounded hover:bg-neutral-200 dark:hover:bg-neutral-700 border border-neutral-300 dark:border-neutral-600"
        title="Stop"
      >
        Stop
      </button>
      <div className="w-px h-6 bg-neutral-300 dark:bg-neutral-600" />
      <label className="text-sm text-neutral-600 dark:text-neutral-400">BPM:</label>
      <input
        type="number"
        value={bpmInput}
        disabled={isPlaying}
        onChange={(e) => setBpmInput(e.target.value)}
        onBlur={() => {
          const val = Number(bpmInput);
          if (bpmInput === '' || isNaN(val)) {
            setBpmInput(String(bpm));
          } else {
            const clamped = Math.max(20, Math.min(300, val));
            setBpmInput(String(clamped));
            onBpmChange(clamped);
          }
        }}
        onKeyDown={(e) => {
          if (e.key === 'Enter') {
            (e.target as HTMLInputElement).blur();
          }
        }}
        className={`w-16 px-2 py-1 text-sm border border-neutral-300 dark:border-neutral-600 rounded bg-white dark:bg-neutral-800 ${
          isPlaying ? 'opacity-40 cursor-not-allowed' : ''
        }`}
        min={20}
        max={300}
      />
      <label className="text-sm text-neutral-600 dark:text-neutral-400 ml-2">Steps:</label>
      <select
        value={steps}
        disabled={isPlaying}
        onChange={(e) => onStepsChange(Number(e.target.value))}
        className={`px-2 py-1 text-sm border border-neutral-300 dark:border-neutral-600 rounded bg-white dark:bg-neutral-800 ${
          isPlaying ? 'opacity-40 cursor-not-allowed' : ''
        }`}
      >
        <option value={8}>8</option>
        <option value={16}>16</option>
        <option value={32}>32</option>
      </select>
    </div>
  );
}
