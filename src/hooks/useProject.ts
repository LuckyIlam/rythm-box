import { useState, useCallback } from 'react';
import type { Project, Pattern, Instrument } from '../types';
import { createDefaultProject } from '../types';

export function useProject() {
  const [project, setProject] = useState<Project>(createDefaultProject);
  const [activePatternId, setActivePatternId] = useState(project.patterns[0]?.id ?? '');
  const [dirty, setDirty] = useState(false);

  const activePattern = project.patterns.find((p) => p.id === activePatternId) ?? project.patterns[0];

  const updatePattern = useCallback((patternId: string, updates: Partial<Pattern>) => {
    setProject((prev) => ({
      ...prev,
      patterns: prev.patterns.map((p) =>
        p.id === patternId ? { ...p, ...updates } : p
      ),
    }));
    setDirty(true);
  }, []);

  const toggleStep = useCallback(
    (instrumentId: string, step: number) => {
      setProject((prev) => ({
        ...prev,
        patterns: prev.patterns.map((p) => {
          if (p.id !== activePatternId) return p;
          const newGrid = { ...p.grid };
          const steps = [...(newGrid[instrumentId] ?? [])];
          if (step < steps.length) {
            steps[step] = !steps[step];
          }
          newGrid[instrumentId] = steps;
          return { ...p, grid: newGrid };
        }),
      }));
      setDirty(true);
    },
    [activePatternId]
  );

  const addPattern = useCallback(() => {
    const newPattern: Pattern = {
      id: `pattern-${Date.now()}`,
      name: `Pattern ${project.patterns.length + 1}`,
      bpm: 120,
      steps: 16,
      instruments: [
        { id: 'kick', name: 'Kick', sound: 'kick', volume: 0.8, sample: 'kick' },
        { id: 'snare', name: 'Snare', sound: 'snare', volume: 0.8, sample: 'snare' },
        { id: 'hihat-closed', name: 'Hi-Hat', sound: 'hihat-closed', volume: 0.8, sample: 'hihat-closed' },
        { id: 'hihat-open', name: 'Open Hat', sound: 'hihat-open', volume: 0.8, sample: 'hihat-open' },
        { id: 'crash', name: 'Crash', sound: 'crash', volume: 0.8, sample: 'crash' },
      ],
      grid: {
        kick: new Array(16).fill(false),
        snare: new Array(16).fill(false),
        'hihat-closed': new Array(16).fill(false),
        'hihat-open': new Array(16).fill(false),
        crash: new Array(16).fill(false),
      },
    };
    setProject((prev) => ({ ...prev, patterns: [...prev.patterns, newPattern] }));
    setActivePatternId(newPattern.id);
    setDirty(true);
  }, [project.patterns.length]);

  const removePattern = useCallback((patternId: string) => {
    setProject((prev) => {
      const patterns = prev.patterns.filter((p) => p.id !== patternId);
      if (patterns.length === 0) return prev;
      return { ...prev, patterns };
    });
    setActivePatternId((prev) => {
      if (prev === patternId) {
        const remaining = project.patterns.filter((p) => p.id !== patternId);
        return remaining[0]?.id ?? '';
      }
      return prev;
    });
    setDirty(true);
  }, [project.patterns]);

  const addInstrument = useCallback(
    (name: string) => {
      const id = name.toLowerCase().replace(/\s+/g, '-');
      const instrument: Instrument = { id, name, sound: 'sine', volume: 0.8, sample: null };
      updatePattern(activePatternId, {
        instruments: [...(activePattern?.instruments ?? []), instrument],
        grid: {
          ...(activePattern?.grid ?? {}),
          [id]: new Array(activePattern?.steps ?? 16).fill(false),
        },
      });
    },
    [activePattern, activePatternId, updatePattern]
  );

  const removeInstrument = useCallback(
    (instrumentId: string) => {
      if (!activePattern) return;
      const { [instrumentId]: _, ...rest } = activePattern.grid;
      updatePattern(activePatternId, {
        instruments: activePattern.instruments.filter((i) => i.id !== instrumentId),
        grid: rest,
      });
    },
    [activePattern, activePatternId, updatePattern]
  );

  const renameInstrument = useCallback(
    (instrumentId: string, newName: string) => {
      if (!activePattern) return;
      updatePattern(activePatternId, {
        instruments: activePattern.instruments.map((i) =>
          i.id === instrumentId ? { ...i, name: newName } : i
        ),
      });
    },
    [activePattern, activePatternId, updatePattern]
  );

  const setInstrumentSound = useCallback(
    (instrumentId: string, sound: string) => {
      if (!activePattern) return;
      updatePattern(activePatternId, {
        instruments: activePattern.instruments.map((i) =>
          i.id === instrumentId ? { ...i, sound, sample: null } : i
        ),
      });
    },
    [activePattern, activePatternId, updatePattern]
  );

  const setInstrumentSample = useCallback(
    (instrumentId: string, sample: string | null) => {
      if (!activePattern) return;
      updatePattern(activePatternId, {
        instruments: activePattern.instruments.map((i) =>
          i.id === instrumentId ? { ...i, sample } : i
        ),
      });
    },
    [activePattern, activePatternId, updatePattern]
  );

  const setSteps = useCallback(
    (count: number) => {
      if (!activePattern) return;
      const newGrid: Record<string, boolean[]> = {};
      for (const [id, steps] of Object.entries(activePattern.grid)) {
        const newSteps = steps.slice(0, count);
        while (newSteps.length < count) newSteps.push(false);
        newGrid[id] = newSteps;
      }
      updatePattern(activePatternId, { steps: count, grid: newGrid });
    },
    [activePattern, activePatternId, updatePattern]
  );

  const setBpm = useCallback(
    (bpm: number) => {
      updatePattern(activePatternId, { bpm });
    },
    [activePatternId, updatePattern]
  );

  const loadProject = useCallback((p: Project) => {
    setProject(p);
    setActivePatternId(p.patterns[0]?.id ?? '');
    setDirty(false);
  }, []);

  const resetProject = useCallback(() => {
    const fresh = createDefaultProject();
    setProject(fresh);
    setActivePatternId(fresh.patterns[0]?.id ?? '');
    setDirty(false);
  }, []);

  return {
    project,
    activePattern,
    activePatternId,
    dirty,
    setActivePatternId,
    toggleStep,
    addPattern,
    removePattern,
    addInstrument,
    removeInstrument,
    renameInstrument,
    setSteps,
    setBpm,
    loadProject,
    resetProject,
    setProject,
    setInstrumentSound,
    setInstrumentSample,
  };
}
