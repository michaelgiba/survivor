use wasm_bindgen::prelude::*;
use web_sys::{Document, WebGlRenderingContext};

/// Draws the playback control region of the application
pub fn draw_playback(
    gl: &WebGlRenderingContext,
    width: i32,
    _height: i32,
    current_step: Option<i32>,
    total_steps: Option<i32>,
) -> Result<(), JsValue> {
    // Define the playback region (bottom strip of the screen)
    let playback_height = 60; // Height in pixels

    // Position at bottom of screen
    gl.viewport(0, 0, width, playback_height);
    gl.scissor(0, 0, width, playback_height);

    // Create or update the HTML overlay for playback controls
    create_or_update_playback_overlay(width, playback_height, current_step, total_steps)?;

    Ok(())
}

/// Creates or updates the playback controls overlay
fn create_or_update_playback_overlay(
    width: i32,
    height: i32,
    current_step: Option<i32>,
    total_steps: Option<i32>,
) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let overlay_id = "playback-controls-overlay";
    let overlay = match document.get_element_by_id(overlay_id) {
        Some(element) => element,
        None => create_playback_overlay(&document, overlay_id)?,
    };

    let progress = current_step
        .map(|curr| total_steps.map(|total| (curr as f32 / total as f32) * 100.0))
        .flatten()
        .unwrap_or(0.0);

    let style = format!(
        "position: absolute; \
        bottom: 0px; \
        left: 0px; \
        width: {}px; \
        height: {}px; \
        display: flex; \
        flex-direction: column; \
        align-items: center; \
        justify-content: center; \
        background: rgba(0, 10, 20, 0.3); \
        border-top: 1px solid rgba(139, 249, 255, 0.3); \
        box-shadow: 0 0 15px rgba(0, 191, 255, 0.2); \
        z-index: 1000;",
        width, height
    );

    overlay.set_attribute("style", &style)?;

    // Format the step counter text
    let step_text = match (current_step, total_steps) {
        (Some(curr), Some(total)) => format!("Step {} / {}", curr, total),
        _ => String::from("--/--"),
    };

    overlay.set_inner_html(&format!(
        r#"
        <div style="
            width: 100%;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 20px;
            padding: 0 20px;
        ">
            <button id="playback-prev" class="playback-button" style="
                background: rgba(0, 20, 40, 0.8);
                border: 1px solid rgba(139, 249, 255, 0.5);
                color: #8DF9FF;
                font-family: 'Orbitron', sans-serif;
                font-size: 14px;
                padding: 5px 15px;
                border-radius: 4px;
                cursor: pointer;
                text-shadow: 0 0 5px #00FFFF;
                transition: all 0.3s ease;
            ">◀</button>

            <button id="playback-play" class="playback-button" style="
                background: rgba(0, 20, 40, 0.8);
                border: 1px solid rgba(139, 249, 255, 0.5);
                color: #8DF9FF;
                font-family: 'Orbitron', sans-serif;
                font-size: 14px;
                padding: 5px 20px;
                border-radius: 4px;
                cursor: pointer;
                text-shadow: 0 0 5px #00FFFF;
                transition: all 0.3s ease;
            ">▶</button>

            <button id="playback-next" class="playback-button" style="
                background: rgba(0, 20, 40, 0.8);
                border: 1px solid rgba(139, 249, 255, 0.5);
                color: #8DF9FF;
                font-family: 'Orbitron', sans-serif;
                font-size: 14px;
                padding: 5px 15px;
                border-radius: 4px;
                cursor: pointer;
                text-shadow: 0 0 5px #00FFFF;
                transition: all 0.3s ease;
            ">▶</button>

            <div style="
                font-family: 'Orbitron', sans-serif;
                font-size: 14px;
                color: #8DF9FF;
                text-shadow: 0 0 5px #00FFFF;
                min-width: 100px;
                text-align: center;
            ">{}</div>
        </div>

        <div style="
            width: calc(100% - 40px);
            height: 4px;
            background: rgba(0, 20, 40, 0.8);
            border-radius: 2px;
            margin: 8px 20px;
            overflow: hidden;
        ">
            <div style="
                width: {}%;
                height: 100%;
                background: #8DF9FF;
                box-shadow: 0 0 10px #00FFFF;
                transition: width 0.3s ease;
            "></div>
        </div>
        "#,
        step_text, progress
    ));

    Ok(())
}

/// Creates a new HTML element for the playback overlay
fn create_playback_overlay(document: &Document, id: &str) -> Result<web_sys::Element, JsValue> {
    let overlay = document.create_element("div")?;
    overlay.set_id(id);

    // Add hover effects for playback buttons
    let style = document.create_element("style")?;
    style.set_text_content(Some(
        r#"
        .playback-button:hover {
            background: rgba(0, 30, 60, 0.8) !important;
            border-color: rgba(139, 249, 255, 0.8) !important;
            box-shadow: 0 0 15px rgba(0, 191, 255, 0.3) !important;
        }
    "#,
    ));

    document.body().unwrap().append_child(&style)?;
    document.body().unwrap().append_child(&overlay)?;

    Ok(overlay)
}
