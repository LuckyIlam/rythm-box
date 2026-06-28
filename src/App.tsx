import { useCallback, useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open, save } from '@tauri-apps/plugin-dialog';
import { MenuBar } from './components/MenuBar';
import { Toolbar } from './components/Toolbar';
import { PatternTabs } from './components/PatternTabs';
import { PatternGrid } from './components/PatternGrid';
import { useProject } from './hooks/useProject';
import type { Project } from './types';

function App() {
  const {
    project,
    activePattern,
    activePatternId,
    dirty,
    setActivePatternId,
    toggleStep,
    addPattern,
    removePattern,
    setSteps,
    setBpm,
    loadProject,
    resetProject,
  } = useProject();

  const [isPlaying, setIsPlaying] = useState(false);
  const [currentStep, setCurrentStep] = useState<number | null>(null);
  const [recentFiles, setRecentFiles] = useState<string[]>(() => {
    const saved = localStorage.getItem('rythm-box-recent');
    return saved ? JSON.parse(saved) : [];
  });
  const playIntervalRef = useRef<number | null>(null);
  const isPlayingRef = useRef(false);

  const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  const [dark] = useState(prefersDark);

  useEffect(() => {
    document.documentElement.classList.toggle('dark', dark);
  }, [dark]);

  useEffect(() => {
    isPlayingRef.current = isPlaying;
  }, [isPlaying]);

  const handlePlay = useCallback(() => {
    if (!activePattern) return;
    if (isPlayingRef.current) {
      setIsPlaying(false);
      setCurrentStep(null);
      if (playIntervalRef.current) {
        clearInterval(playIntervalRef.current);
        playIntervalRef.current = null;
      }
      if (isTauri) {
        invoke('stop_audio');
      }
    } else {
      setIsPlaying(true);
      setCurrentStep(0);
      if (isTauri) {
        invoke('play_pattern', { patternJson: JSON.stringify(activePattern) });
      }
      const intervalMs = 60000 / (activePattern.bpm * 4);
      let step = 0;
      playIntervalRef.current = window.setInterval(() => {
        step = (step + 1) % activePattern.steps;
        setCurrentStep(step);
      }, intervalMs);
    }
  }, [activePattern, isTauri]);

  const handleStop = useCallback(() => {
    setIsPlaying(false);
    setCurrentStep(null);
    if (playIntervalRef.current) {
      clearInterval(playIntervalRef.current);
      playIntervalRef.current = null;
    }
    if (isTauri) {
      invoke('stop_audio');
    }
  }, [isTauri]);

  const handleBpmChange = useCallback(
    (bpm: number) => {
      setBpm(bpm);
      if (isTauri) {
        invoke('set_audio_bpm', { bpm });
      }
    },
    [setBpm, isTauri]
  );

  useEffect(() => {
    const saved = localStorage.getItem('rythm-box-project');
    if (saved) {
      try {
        const parsed = JSON.parse(saved) as Project;
        const hasCrash = parsed.patterns.some(p =>
          p.instruments.some(i => i.id === 'crash')
        );
        if (hasCrash) {
          loadProject(parsed);
        }
      } catch { /* ignore corrupt save */ }
    }
  }, []);

  useEffect(() => {
    const timer = setTimeout(() => {
      localStorage.setItem('rythm-box-project', JSON.stringify(project));
    }, 2000);
    return () => clearTimeout(timer);
  }, [project]);

  useEffect(() => {
    return () => {
      if (playIntervalRef.current) clearInterval(playIntervalRef.current);
    };
  }, []);

  const handleSave = useCallback(async () => {
    const json = JSON.stringify(project, null, 2);
    if (isTauri) {
      try {
        const path = await save({
          defaultPath: `${project.meta.name}.rythm`,
          filters: [{ name: 'Rhythm Box Project', extensions: ['rythm'] }],
        });
        if (path) {
          await invoke('save_project', { path, projectJson: json });
        }
      } catch (e) {
        console.error('Save failed:', e);
      }
    } else {
      try {
        const blob = new Blob([json], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `${project.meta.name}.rythm`;
        a.style.display = 'none';
        document.body.appendChild(a);
        a.click();
        setTimeout(() => {
          document.body.removeChild(a);
          URL.revokeObjectURL(url);
        }, 1000);
      } catch (e) {
        console.error('Save failed:', e);
      }
    }
    // Add to recent files
    setRecentFiles((prev) => {
      const updated = [project.meta.name, ...prev.filter((f) => f !== project.meta.name)].slice(0, 5);
      localStorage.setItem('rythm-box-recent', JSON.stringify(updated));
      return updated;
    });
  }, [project, isTauri]);

  const handleOpenRecent = useCallback((_fileName: string) => {
    const saved = localStorage.getItem('rythm-box-project');
    if (saved) {
      try {
        const parsed = JSON.parse(saved) as Project;
        loadProject(parsed);
      } catch { /* ignore */ }
    }
  }, [loadProject]);

  const handleOpen = useCallback(async () => {
    if (isTauri) {
      try {
        const path = await open({
          filters: [{ name: 'Rhythm Box Project', extensions: ['rythm'] }],
          multiple: false,
        });
        if (path) {
          const content = await invoke<string>('load_project', { path });
          const parsed = JSON.parse(content) as Project;
          loadProject(parsed);
        }
      } catch (e) {
        console.error('Open failed:', e);
      }
    } else {
      const input = document.createElement('input');
      input.type = 'file';
      input.accept = '.rythm,application/json';
      input.onchange = async (e) => {
        const file = (e.target as HTMLInputElement).files?.[0];
        if (!file) return;
        const text = await file.text();
        try {
          const parsed = JSON.parse(text) as Project;
          loadProject(parsed);
        } catch {
          alert('Invalid project file');
        }
      };
      input.click();
    }
  }, [isTauri, loadProject]);

  const handleNew = useCallback(() => {
    if (dirty && !confirm('Discard unsaved changes?')) return;
    resetProject();
  }, [dirty, resetProject]);

  const handleExport = useCallback(() => {
    if (!activePattern) return;
    const json = JSON.stringify(activePattern, null, 2);
    const blob = new Blob([json], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${activePattern.name}.json`;
    a.click();
    URL.revokeObjectURL(url);
  }, [activePattern]);

  useEffect(() => {
    const onKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 's') { e.preventDefault(); handleSave(); }
      if ((e.ctrlKey || e.metaKey) && e.key === 'o') { e.preventDefault(); handleOpen(); }
      if ((e.ctrlKey || e.metaKey) && e.key === 'n') { e.preventDefault(); handleNew(); }
      if (e.key === ' ') { e.preventDefault(); handlePlay(); }
    };
    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
  }, [handleSave, handleOpen, handleNew, handlePlay]);

  if (!activePattern) {
    return (
      <div className="flex items-center justify-center h-screen text-neutral-500">
        No patterns. Click + to create one.
      </div>
    );
  }

  return (
    <div className="flex flex-col h-screen bg-white dark:bg-neutral-900 text-neutral-900 dark:text-neutral-100">
      <MenuBar
        onNew={handleNew}
        onOpen={handleOpen}
        onSave={handleSave}
        onSaveAs={handleSave}
        onExport={handleExport}
        onExit={() => window.close()}
        recentFiles={recentFiles}
        onOpenRecent={handleOpenRecent}
      />
      <Toolbar
        isPlaying={isPlaying}
        bpm={activePattern.bpm}
        steps={activePattern.steps}
        onPlay={handlePlay}
        onStop={handleStop}
        onSave={handleSave}
        onNew={handleNew}
        onBpmChange={handleBpmChange}
        onStepsChange={setSteps}
      />
      <PatternTabs
        patterns={project.patterns}
        activeId={activePatternId}
        isPlaying={isPlaying}
        onSelect={setActivePatternId}
        onAdd={addPattern}
        onRemove={removePattern}
      />
      <div className="flex-1 overflow-hidden">
        <PatternGrid
          key={activePattern.id}
          pattern={activePattern}
          currentStep={currentStep}
          isPlaying={isPlaying}
          onToggleStep={toggleStep}
        />
      </div>
    </div>
  );
}

export default App;
