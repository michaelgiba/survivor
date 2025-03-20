import { RolloutData, Event } from '../types/rollout';

export class MainDisplay {
    private static instance: MainDisplay;
    private canvas: HTMLCanvasElement;
    private ctx: WebGLRenderingContext;
    private textOverlay: HTMLElement;

    constructor(canvas: HTMLCanvasElement) {
        if (MainDisplay.instance) {
            return MainDisplay.instance;
        }
        MainDisplay.instance = this;

        this.canvas = canvas;
        const gl = canvas.getContext('webgl');
        if (!gl) {
            throw new Error('WebGL not supported');
        }
        this.ctx = gl;

        this.textOverlay = document.createElement('div');
        this.textOverlay.style.cssText = `
            position: absolute;
            top: 50px;
            left: 100px;
            width: calc(100% - 100px);
            height: calc(100vh - 110px);
            display: flex;
            align-items: center;
            justify-content: center;
            font-family: 'Arial', sans-serif;
            font-size: 42px;
            font-weight: bold;
            color: #8DF9FF;
            text-shadow: 0 0 15px #00FFFF, 0 0 25px #00BFFF;
            letter-spacing: 4px;
            text-align: center;
            pointer-events: none;
            mix-blend-mode: lighten;
        `;
        document.body.appendChild(this.textOverlay);

        // Set clear color to dark blue for background
        this.ctx.clearColor(0.0, 0.05, 0.1, 1.0);
    }

    draw(rolloutData: RolloutData, currentStep: number) {
        const event = rolloutData.events[currentStep];
        this.updateTextDisplay(event);
        
        // Clear the canvas
        this.ctx.clear(this.ctx.COLOR_BUFFER_BIT);
        
        // Update playback controls
        const playbackControls = document.querySelector('.playback-controls');
        if (playbackControls) {
            const progress = (currentStep + 1) / rolloutData.events.length;
            const progressBar = playbackControls.querySelector('.progress-bar') as HTMLElement;
            if (progressBar) {
                progressBar.style.width = `${progress * 100}%`;
            }
            
            const counter = playbackControls.querySelector('.step-counter') as HTMLElement;
            if (counter) {
                counter.textContent = `${currentStep + 1} / ${rolloutData.events.length}`;
            }
        }
    }

    private updateTextDisplay(event: Event) {
        let displayText = '';
        
        switch (event.event_type) {
            case 'MessageEvent':
                displayText = `${event.event_params.send_player_id} → ${event.event_params.recv_player_id}: "${event.event_params.message}"`;
                break;
            case 'SpeechEvent':
                displayText = `${event.event_params.speaking_player_id}: "${event.event_params.statement}"`;
                break;
            case 'VotingEvent':
                displayText = `${event.event_params.voting_player_id} votes for ${event.event_params.target_elimination_player_id}`;
                break;
            case 'EliminationEvent':
                displayText = `Player ${event.event_params.eliminated_player_id} eliminated`;
                break;
            case 'WinnerEvent':
                displayText = `Player ${event.event_params.winner_player_id} wins!`;
                break;
            default:
                displayText = `Event: ${event.event_type}`;
        }

        this.textOverlay.textContent = displayText;
    }

    static getInstance(): MainDisplay {
        return MainDisplay.instance;
    }
}