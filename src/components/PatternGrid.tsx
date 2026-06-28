import type { Pattern } from '../types';
import { InstrumentRow } from './InstrumentRow';

interface PatternGridProps {
  pattern: Pattern;
  currentStep: number | null;
  isPlaying: boolean;
  onToggleStep: (instrumentId: string, step: number) => void;
}

export function PatternGrid({
  pattern,
  currentStep,
  isPlaying,
  onToggleStep,
}: PatternGridProps) {

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-stretch border-b border-neutral-300 dark:border-neutral-600 bg-neutral-50 dark:bg-neutral-850">
        <div className="w-32 px-2 py-1 text-xs font-medium text-neutral-500 border-r border-neutral-200 dark:border-neutral-700 shrink-0">
          Instruments
        </div>
        <div className="flex flex-1">
          {Array.from({ length: pattern.steps }, (_, i) => (
            <div
              key={i}
              className={`flex-1 text-center text-xs py-1 border-r border-neutral-200 dark:border-neutral-700 ${
                i % 4 === 0 ? 'border-l-2 border-l-neutral-400 dark:border-l-neutral-500 font-medium' : ''
              }`}
            >
              {i + 1}
            </div>
          ))}
        </div>
      </div>
      <div className="flex-1 overflow-y-auto">
        {pattern.instruments.map((inst) => (
          <InstrumentRow
            key={inst.id}
            instrument={inst}
            steps={pattern.grid[inst.id] ?? []}
            currentStep={currentStep}
            isPlaying={isPlaying}
            onToggleStep={(step) => onToggleStep(inst.id, step)}
          />
        ))}
      </div>
    </div>
  );
}
