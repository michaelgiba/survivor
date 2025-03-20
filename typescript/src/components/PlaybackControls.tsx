import React from 'react';
import '../styles/PlaybackControls.css';

interface PlaybackControlsProps {
  onPlay: () => void;
  onPause: () => void;
  onNext: () => void;
  onPrev: () => void;
  isPlaying?: boolean;
  currentStep?: number;
  totalSteps?: number;
}

export const PlaybackControls: React.FC<PlaybackControlsProps> = ({
  onPlay,
  onPause,
  onNext,
  onPrev,
  isPlaying = false,
  currentStep = 0,
  totalSteps = 0,
}) => {
  const progressPercent = totalSteps > 0 ? ((currentStep + 1) / totalSteps) * 100 : 0;

  return (
    <div className="playback-controls">
      <div className="controls-row">
        <button className="playback-button" onClick={onPrev}>◀</button>
        <button className="playback-button" onClick={isPlaying ? onPause : onPlay}>
          {isPlaying ? '⏸' : '▶'}
        </button>
        <button className="playback-button" onClick={onNext}>▶</button>
        <div className="step-counter">
          {currentStep + 1} / {totalSteps}
        </div>
      </div>
      <div className="progress-bar">
        <div 
          className="progress-fill"
          style={{ width: `${progressPercent}%` }}
        />
      </div>
    </div>
  );
};