import React, { useRef, useMemo } from 'react';
import { Canvas } from '@react-three/fiber';
import * as THREE from 'three';
import { OrbitControls } from '@react-three/drei';

interface PlayerEntity {
  id: number;
  position: THREE.Vector3;
  isEliminated: boolean;
}

function Circle({ position, color }: { position: THREE.Vector3; color: string }) {
  return (
    <mesh position={position}>
      <circleGeometry args={[0.5, 32]} />
      <meshStandardMaterial color={color} transparent opacity={0.8} />
    </mesh>
  );
}

interface Scene {
  currentStep: number;
  rolloutData: any;
}

function Entities({ currentStep, rolloutData }: Scene) {
  const entitiesRef = useRef<THREE.Group>(null);
  
  // Calculate player positions in a circle
  const players = useMemo(() => {
    if (!rolloutData?.events) return [];
    
    // Get initial player list from first ENTER_NORMAL_ROUND event
    const initialEvent = rolloutData.events.find(
      (event: any) => event.event_type === 'ENTER_NORMAL_ROUND'
    );
    
    if (!initialEvent) return [];
    
    const playerIds = initialEvent.event_params.player_ids_in_round;
    const radius = playerIds.length * 0.8; // Scale circle based on player count
    
    const players: PlayerEntity[] = playerIds.map((id: number, i: number) => {
      const angle = (i / playerIds.length) * Math.PI * 2;
      return {
        id,
        position: new THREE.Vector3(
          Math.cos(angle) * radius,
          0,
          Math.sin(angle) * radius
        ),
        isEliminated: false
      };
    });

    // Process elimination events up to current step
    for (let i = 0; i <= currentStep && i < rolloutData.events.length; i++) {
      const event = rolloutData.events[i];
      if (event.event_type === 'ELIMINATION') {
        const eliminatedId = event.event_params.eliminated_player_id;
        const player = players.find(p => p.id === eliminatedId);
        if (player) player.isEliminated = true;
      }
    }
    
    return players;
  }, [rolloutData, currentStep]);

  return (
    <group ref={entitiesRef}>
      {players.map((player) => (
        <Circle
          key={player.id}
          position={player.position}
          color={player.isEliminated ? 'red' : 'cyan'}
        />
      ))}
    </group>
  );
}

interface ThreeCanvasProps {
  currentStep: number;
  rolloutData: any;
}

export const ThreeCanvas: React.FC<ThreeCanvasProps> = ({ currentStep, rolloutData }) => {
  return (
    <div className="canvas-container">
      <Canvas
        camera={{ position: [0, 10, 0], fov: 75 }}
        style={{
          background: 'black',
          width: '100%',
          height: '100%',
        }}
      >
        <OrbitControls />
        <ambientLight intensity={0.5} />
        <pointLight position={[10, 10, 10]} />
        <gridHelper args={[20, 20, 'cyan', 'cyan']} />
        <Entities currentStep={currentStep} rolloutData={rolloutData} />
      </Canvas>
      <div className="crt-overlay" />
      <div className="crt-glow" />
      <div className="power-button" />
    </div>
  );
};