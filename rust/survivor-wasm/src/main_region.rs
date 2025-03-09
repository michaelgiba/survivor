use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, WebGlRenderingContext};

// Import our modules
use crate::ui_interactions;

/// Draws the main content region of the application
pub fn draw_main_region(
    gl: &WebGlRenderingContext,
    width: i32,
    height: i32,
) -> Result<(), JsValue> {
    // Define the main region dimensions
    let sidebar_width = 200; // Width of sidebar
    let navbar_height = 50; // Height of navbar

    // Main region is the remaining space after sidebar and navbar
    let main_width = width - sidebar_width;
    let main_height = height - navbar_height;

    gl.viewport(sidebar_width, 0, main_width, main_height);
    gl.scissor(sidebar_width, 0, main_width, main_height);

    // No need to clear or draw a background since the animation is already drawn
    // We'll just create the overlay for showing text content

    // Create or update the text display overlay for button selections
    create_or_update_main_overlay(sidebar_width, main_width, main_height)?;

    Ok(())
}

/// Creates or updates the overlay for displaying selected text in main region
fn create_or_update_main_overlay(x_offset: i32, width: i32, height: i32) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Check if our main region overlay already exists
    let overlay_id = "main-region-overlay";
    let overlay = match document.get_element_by_id(overlay_id) {
        Some(element) => element,
        None => create_main_overlay(&document, overlay_id)?,
    };

    // Configure the main region overlay
    let overlay = overlay.dyn_into::<HtmlElement>()?;
    overlay.style().set_property("position", "absolute")?;
    overlay.style().set_property("top", "50px")?; // Position below navbar
    overlay
        .style()
        .set_property("left", &format!("{}px", x_offset))?;
    overlay
        .style()
        .set_property("width", &format!("{}px", width))?;
    overlay
        .style()
        .set_property("height", &format!("{}px", height))?;
    overlay.style().set_property("display", "flex")?;
    overlay.style().set_property("align-items", "center")?;
    overlay.style().set_property("justify-content", "center")?;

    // Apply futuristic styling for text
    overlay
        .style()
        .set_property("font-family", "'Orbitron', 'Arial', sans-serif")?;
    overlay.style().set_property("font-size", "42px")?;
    overlay.style().set_property("font-weight", "bold")?;
    overlay.style().set_property("color", "#8DF9FF")?; // Cyan color
    overlay
        .style()
        .set_property("text-shadow", "0 0 15px #00FFFF, 0 0 25px #00BFFF")?; // Glow effect
    overlay.style().set_property("letter-spacing", "4px")?;
    overlay.style().set_property("text-align", "center")?;
    overlay.style().set_property("pointer-events", "none")?; // Ensure it doesn't interfere with clicks
    overlay.style().set_property("mix-blend-mode", "lighten")?; // Make text blend with animation

    // Display the currently selected text, if any
    if let Some(text) = ui_interactions::get_active_selection() {
        overlay.set_inner_text(&text);
    } else {
        overlay.set_inner_text("[SELECT FUNCTION]");
    }

    Ok(())
}

/// Creates a new HTML element for the main region overlay
fn create_main_overlay(document: &Document, id: &str) -> Result<web_sys::Element, JsValue> {
    let overlay = document.create_element("div")?;
    overlay.set_id(id);

    // Append overlay to body
    document.body().unwrap().append_child(&overlay)?;

    // Initialize with default text
    overlay.set_text_content(Some("[SELECT FUNCTION]"));

    Ok(overlay)
}
