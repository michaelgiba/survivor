import React from 'react';

interface RolloutSelectorProps {
  onSelect: (filename: string) => void;
}

export const RolloutSelector: React.FC<RolloutSelectorProps> = ({ onSelect }) => {
  return (
    <div id="navbar">
      <div style={{ flex: 1, minWidth: '120px' }}>Survivor</div>
      <select
        id="rolloutSelector"
        onChange={(e) => onSelect(e.target.value)}
        style={{
          background: 'rgba(0, 20, 40, 0.7)',
          border: '1px solid rgba(141, 249, 255, 0.3)',
          color: 'rgba(141, 249, 255, 0.9)',
          fontFamily: "'Orbitron', sans-serif",
          fontSize: '14px',
          padding: '4px 8px',
          borderRadius: '3px',
          margin: '5px',
          outline: 'none',
          cursor: 'pointer',
        }}
      >
        <option value="">Select a rollout...</option>
        <option value="survivor-1.json">Survivor 1</option>
      </select>
    </div>
  );
};