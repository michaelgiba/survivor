export interface EventParams {
    player_ids_in_round?: number[];
    send_player_id?: number;
    recv_player_id?: number;
    message?: string;
    speaking_player_id?: number;
    statement?: string;
    voting_player_id?: number;
    target_elimination_player_id?: number;
    player_id_to_vote_count?: Record<string, number>;
    eliminated_player_id?: number;
    reason?: string;
    final_two_player_ids?: number[];
    speech_text?: string;
    voted_to_win_player_id?: number;
    winner_player_id?: number;
}

export interface Event {
    seq_number: number;
    event_type: string;
    event_params_type: string;
    event_params: EventParams;
}

export interface RolloutData {
    events: Event[];
}

export interface RolloutSummary {
    total_events: number;
    winner_id?: number;
    player_count: number;
    message_count: number;
}