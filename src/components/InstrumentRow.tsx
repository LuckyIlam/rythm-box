import type { Instrument } from '../types';

interface InstrumentRowProps {
  instrument: Instrument;
  steps: boolean[];
  currentStep: number | null;
  isPlaying: boolean;
  onToggleStep: (step: number) => void;
}

export function InstrumentRow({
  instrument,
  steps,
  currentStep,
  isPlaying,
  onToggleStep,
}: InstrumentRowProps) {
  return (
    <div className="flex items-stretch border-b border-neutral-200 dark:border-neutral-700">
      <div className="flex items-center gap-1 w-40 px-2 py-1 bg-neutral-50 dark:bg-neutral-850 border-r border-neutral-200 dark:border-neutral-700 shrink-0">
        <span className="flex-1 text-sm truncate cursor-default" title={instrument.name}>
          {instrument.name}
        </span>
      </div>
      <div className="flex flex-1">
        {steps.map((active, i) => {
          const isCurrent = isPlaying && currentStep === i;
          const isBeatStart = i % 4 === 0;
          return (
            <button
              key={i}
              onClick={() => !isPlaying && onToggleStep(i)}
              className={`flex-1 aspect-square border-r border-b border-neutral-200 dark:border-neutral-700 transition-colors ${
                isPlaying ? 'cursor-default' : ''
              } ${
                isCurrent
                  ? active
                    ? 'bg-blue-500'
                    : 'bg-blue-200 dark:bg-blue-900'
                  : active
                    ? 'bg-neutral-800 dark:bg-neutral-200'
                    : 'bg-white dark:bg-neutral-900 hover:bg-neutral-100 dark:hover:bg-neutral-800'
              } ${isBeatStart ? 'border-l-2 border-l-neutral-400 dark:border-l-neutral-500' : ''}`}
            />
          );
        })}
      </div>
    </div>
  );
}
