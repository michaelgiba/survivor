import React, { useState, useEffect, useCallback } from 'react';
import { PlaybackControls } from './PlaybackControls';
import { RolloutSelector } from './RolloutSelector';
import { ThreeCanvas } from './ThreeCanvas';

export const App: React.FC = () => {
  const [isPlaying, setIsPlaying] = useState(false);
  const [currentStep, setCurrentStep] = useState(0);
  const [totalSteps, setTotalSteps] = useState(0);
  const [rolloutData, setRolloutData] = useState<any>(null);
  const [playbackInterval, setPlaybackInterval] = useState<number | null>(null);

  const stopPlayback = useCallback(() => {
    if (playbackInterval) {
      clearInterval(playbackInterval);
      setPlaybackInterval(null);
    }
    setIsPlaying(false);
  }, [playbackInterval]);

  const startPlayback = useCallback(() => {
    if (!rolloutData) return;
    
    setIsPlaying(true);
    const interval = window.setInterval(() => {
      setCurrentStep(current => {
        if (current >= totalSteps - 1) {
          stopPlayback();
          return current;
        }
        return current + 1;
      });
    }, 1000);
    
    setPlaybackInterval(interval);
  }, [rolloutData, totalSteps, stopPlayback]);

  const handlePrev = () => {
    stopPlayback();
    setCurrentStep(current => Math.max(0, current - 1));
  };

  const handleNext = () => {
    stopPlayback();
    setCurrentStep(current => Math.min(totalSteps - 1, current + 1));
  };

  const handleRolloutSelect = async (filename: string) => {
    if (!filename) {
      setRolloutData(null);
      return;
    }

    try {
      const response = await fetch(`/rollouts/${filename}`);
      const data = await response.json();
      setRolloutData(data);
      setCurrentStep(0);
      setTotalSteps(data.length || 0);
    } catch (error) {
      console.error('Error loading rollout:', error);
    }
  };

  useEffect(() => {
    return () => {
      if (playbackInterval) {
        clearInterval(playbackInterval);
      }
    };
  }, [playbackInterval]);

  return (
    <>
      <RolloutSelector onSelect={handleRolloutSelect} />
      <div className="crt-container">
        <ThreeCanvas 
          currentStep={currentStep}
          rolloutData={rolloutData}
        />
      </div>
      <PlaybackControls
        onPlay={startPlayback}
        onPause={stopPlayback}
        onNext={handleNext}
        onPrev={handlePrev}
        isPlaying={isPlaying}
        currentStep={currentStep}
        totalSteps={totalSteps}
      />
    </>
  );
};