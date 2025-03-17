use serde_json::Value;
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, WebGlRenderingContext};

/// Draws the main content region of the application
pub fn draw_main_region(
    gl: &WebGlRenderingContext,
    width: i32,
    height: i32,
    rollout_data: Option<&Value>,
) -> Result<(), JsValue> {
    // Define the main region dimensions
    let sidebar_width = 100; // Width of sidebar (reduced from 200)
    let navbar_height = 50; // Height of navbar

    // Main region is the remaining space after sidebar and navbar
    let main_width = width - sidebar_width;
    let main_height = height - navbar_height;

    gl.viewport(sidebar_width, 0, main_width, main_height);
    gl.scissor(sidebar_width, 0, main_width, main_height);

    // Create or update the text display overlay for rollout data
    create_or_update_main_overlay(sidebar_width, main_width, main_height, rollout_data)?;

    Ok(())
}

/// Creates or updates the overlay for displaying selected text in main region
fn create_or_update_main_overlay(
    x_offset: i32,
    width: i32,
    height: i32,
    rollout_data: Option<&Value>,
) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Check if our main region overlay already exists
    let overlay_id = "main-region-overlay";
    let overlay = match document.get_element_by_id(overlay_id) {
        Some(element) => element,
        None => create_main_overlay(&document, overlay_id)?,
    };

    let style = format!(
        "position: absolute; \
        top: 50px; \
        left: {}px; \
        width: {}px; \
        height: {}px; \
        display: flex; \
        align-items: center; \
        justify-content: center; \
        font-family: 'Orbitron', 'Arial', sans-serif; \
        font-size: 42px; \
        font-weight: bold; \
        color: #8DF9FF; \
        text-shadow: 0 0 15px #00FFFF, 0 0 25px #00BFFF; \
        letter-spacing: 4px; \
        text-align: center; \
        pointer-events: none; \
        mix-blend-mode: lighten;",
        x_offset, width, height
    );

    overlay.set_attribute("style", &style)?;

    // Update content based on rollout data
    let display_text = match rollout_data {
        Some(data) => format!("Rollout Data: {}", data),
        None => String::from("[SELECT ROLLOUT]"),
    };
    overlay.set_text_content(Some(&display_text));

    Ok(())
}

/// Creates a new HTML element for the main region overlay
fn create_main_overlay(document: &Document, id: &str) -> Result<web_sys::Element, JsValue> {
    let overlay = document.create_element("div")?;
    overlay.set_id(id);

    // Append overlay to body
    document.body().unwrap().append_child(&overlay)?;

    // Initialize with default text
    overlay.set_text_content(Some("[SELECT ROLLOUT]"));

    Ok(overlay)
}
