use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{console, Request, RequestInit, RequestMode, Response};
use wasm_bindgen_futures::JsFuture;

// Define the structures to match the JSON format
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventParams {
    #[serde(rename = "player_ids_in_round", skip_serializing_if = "Option::is_none")]
    pub player_ids_in_round: Option<Vec<u32>>,
    
    #[serde(rename = "send_player_id", skip_serializing_if = "Option::is_none")]
    pub send_player_id: Option<u32>,
    
    #[serde(rename = "recv_player_id", skip_serializing_if = "Option::is_none")]
    pub recv_player_id: Option<u32>,
    
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    
    #[serde(rename = "speaking_player_id", skip_serializing_if = "Option::is_none")]
    pub speaking_player_id: Option<u32>,
    
    #[serde(rename = "statement", skip_serializing_if = "Option::is_none")]
    pub statement: Option<String>,
    
    #[serde(rename = "voting_player_id", skip_serializing_if = "Option::is_none")]
    pub voting_player_id: Option<u32>,
    
    #[serde(rename = "target_elimination_player_id", skip_serializing_if = "Option::is_none")]
    pub target_elimination_player_id: Option<u32>,
    
    #[serde(rename = "player_id_to_vote_count", skip_serializing_if = "Option::is_none")]
    pub player_id_to_vote_count: Option<std::collections::HashMap<String, u32>>,
    
    #[serde(rename = "eliminated_player_id", skip_serializing_if = "Option::is_none")]
    pub eliminated_player_id: Option<u32>,
    
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    
    #[serde(rename = "final_two_player_ids", skip_serializing_if = "Option::is_none")]
    pub final_two_player_ids: Option<Vec<u32>>,
    
    #[serde(rename = "speech_text", skip_serializing_if = "Option::is_none")]
    pub speech_text: Option<String>,
    
    #[serde(rename = "voted_to_win_player_id", skip_serializing_if = "Option::is_none")]
    pub voted_to_win_player_id: Option<u32>,
    
    #[serde(rename = "winner_player_id", skip_serializing_if = "Option::is_none")]
    pub winner_player_id: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub seq_number: u32,
    pub event_type: String,
    pub event_params_type: String,
    pub event_params: EventParams,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RolloutData {
    pub events: Vec<Event>,
}

/// Cached rollout data
static mut CURRENT_ROLLOUT: Option<RolloutData> = None;
static mut AVAILABLE_ROLLOUTS: Option<Vec<String>> = None;

#[wasm_bindgen]
pub async fn load_available_rollouts() -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init("/rollouts/", &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    if !resp.ok() {
        return Err(JsValue::from_str("Failed to load rollouts list"));
    }

    // For now, we'll just hardcode the rollout we know exists
    let rollouts = vec!["survivor-1.json".to_string()];
    unsafe {
        AVAILABLE_ROLLOUTS = Some(rollouts.clone());
    }
    
    Ok(serde_wasm_bindgen::to_value(&rollouts)?)
}

#[wasm_bindgen]
pub async fn load_rollout_data(filename: &str) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&format!("/rollouts/{}", filename), &opts)?;
    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    if !resp.ok() {
        return Err(JsValue::from_str("Failed to load rollout data"));
    }

    let json = JsFuture::from(resp.json()?).await?;
    let rollout_data: RolloutData = serde_wasm_bindgen::from_value(json)?;
    
    // Cache the rollout data
    unsafe {
        CURRENT_ROLLOUT = Some(rollout_data.clone());
    }
    
    // Return summary data for display
    let summary = extract_summary(&rollout_data);
    Ok(serde_wasm_bindgen::to_value(&summary)?)
}

#[wasm_bindgen]
pub fn get_rollout_summary() -> Result<JsValue, JsValue> {
    unsafe {
        if let Some(ref rollout) = CURRENT_ROLLOUT {
            let summary = extract_summary(rollout);
            return Ok(serde_wasm_bindgen::to_value(&summary)?);
        }
    }
    
    Err(JsValue::from_str("No rollout data loaded"))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RolloutSummary {
    pub total_events: usize,
    pub winner_id: Option<u32>,
    pub player_count: usize,
    pub message_count: usize,
}

fn extract_summary(rollout: &RolloutData) -> RolloutSummary {
    let mut player_ids = std::collections::HashSet::new();
    let mut message_count = 0;
    let mut winner_id = None;
    
    for event in &rollout.events {
        // Count messages
        if event.event_type == "PRIVATE_MESSAGE" || event.event_type == "PUBLIC_STATEMENT" {
            message_count += 1;
        }
        
        // Track unique player IDs
        if let Some(id) = event.event_params.speaking_player_id {
            player_ids.insert(id);
        }
        if let Some(id) = event.event_params.send_player_id {
            player_ids.insert(id);
        }
        if let Some(id) = event.event_params.recv_player_id {
            player_ids.insert(id);
        }
        if let Some(ids) = &event.event_params.player_ids_in_round {
            for id in ids {
                player_ids.insert(*id);
            }
        }
        
        // Check for winner
        if event.event_type == "WINNER" {
            winner_id = event.event_params.winner_player_id;
        }
    }
    
    RolloutSummary {
        total_events: rollout.events.len(),
        winner_id,
        player_count: player_ids.len(),
        message_count,
    }
}