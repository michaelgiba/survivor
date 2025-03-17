use serde_json::Value;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

// Import our modules
mod main_region;
mod navbar;
mod playback;
mod sidebar;

// Track playback state using thread-safe Mutex
lazy_static::lazy_static! {
    static ref PLAYBACK_STATE: Mutex<(Option<i32>, Option<i32>)> = Mutex::new((None, None));
}

#[wasm_bindgen]
pub fn draw_main(canvas_id: &str, rollout_data: Option<String>) -> Result<(), JsValue> {
    // Get canvas and GL context
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id(canvas_id)
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    // Ensure the canvas has an ID for our overlay elements to reference
    canvas.set_id(canvas_id);

    let gl = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    // Get canvas dimensions
    let width = canvas.width() as i32;
    let height = canvas.height() as i32;

    // Parse rollout data if provided and update total steps
    let parsed_data = rollout_data.and_then(|data| serde_json::from_str::<Value>(&data).ok());
    if let Some(data) = &parsed_data {
        if let Some(events) = data.as_array() {
            let mut state = PLAYBACK_STATE.lock().unwrap();
            state.1 = Some(events.len() as i32);
            // Reset current step when loading new rollout
            if state.0.is_none() {
                state.0 = Some(0);
            }
        }
    }

    // Enable scissor test for drawing UI panels
    gl.enable(WebGlRenderingContext::SCISSOR_TEST);

    // Draw UI panels (which will be semi-transparent to show animation)
    sidebar::draw_sidebar(&gl, width, height)?;
    main_region::draw_main_region(&gl, width, height, parsed_data.as_ref())?;
    navbar::draw_navbar(&gl, width, height)?;

    let state = PLAYBACK_STATE.lock().unwrap();
    playback::draw_playback(&gl, width, height, state.0, state.1)?;

    // Disable scissor test when done
    gl.disable(WebGlRenderingContext::SCISSOR_TEST);

    Ok(())
}

#[wasm_bindgen]
pub fn set_current_step(step: i32) -> Result<(), JsValue> {
    let mut state = PLAYBACK_STATE.lock().unwrap();
    if let Some(total) = state.1 {
        if step >= 0 && step < total {
            state.0 = Some(step);
        }
    }
    Ok(())
}

#[wasm_bindgen]
pub fn get_current_step() -> Option<i32> {
    PLAYBACK_STATE.lock().unwrap().0
}

#[wasm_bindgen]
pub fn get_total_steps() -> Option<i32> {
    PLAYBACK_STATE.lock().unwrap().1
}
