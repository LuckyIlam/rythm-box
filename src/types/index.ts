export interface Instrument {
  id: string;
  name: string;
  sound: string;
  volume: number;
  sample: string | null;
}

export interface Pattern {
  id: string;
  name: string;
  bpm: number;
  steps: number;
  instruments: Instrument[];
  grid: Record<string, boolean[]>;
}

export interface ProjectMeta {
  name: string;
  version: string;
  created: string;
  modified: string;
}

export interface ProjectSettings {
  master_volume: number;
}

export interface Project {
  meta: ProjectMeta;
  settings: ProjectSettings;
  patterns: Pattern[];
}

export const DEFAULT_INSTRUMENTS: Instrument[] = [
  { id: 'kick', name: 'Kick', sound: 'kick', volume: 0.8, sample: 'kick' },
  { id: 'snare', name: 'Snare', sound: 'snare', volume: 0.8, sample: 'snare' },
  { id: 'hihat-closed', name: 'Hi-Hat', sound: 'hihat-closed', volume: 0.8, sample: 'hihat-closed' },
  { id: 'hihat-open', name: 'Open Hat', sound: 'hihat-open', volume: 0.8, sample: 'hihat-open' },
  { id: 'crash', name: 'Crash', sound: 'crash', volume: 0.8, sample: 'crash' },
];

export function createDefaultPattern(): Pattern {
  const grid: Record<string, boolean[]> = {};
  for (const inst of DEFAULT_INSTRUMENTS) {
    grid[inst.id] = new Array(16).fill(false);
  }
  return {
    id: 'pattern-1',
    name: 'Pattern 1',
    bpm: 120,
    steps: 16,
    instruments: [...DEFAULT_INSTRUMENTS],
    grid,
  };
}

export function createDefaultProject(): Project {
  return {
    meta: {
      name: 'Untitled',
      version: '1.0',
      created: new Date().toISOString(),
      modified: new Date().toISOString(),
    },
    settings: { master_volume: 1.0 },
    patterns: [createDefaultPattern()],
  };
}
