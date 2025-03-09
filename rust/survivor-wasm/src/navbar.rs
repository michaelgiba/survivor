use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, WebGlRenderingContext};

/// Draws the navbar region of the application
pub fn draw_navbar(gl: &WebGlRenderingContext, width: i32, height: i32) -> Result<(), JsValue> {
    // Define the navbar region (top strip of the screen)
    let navbar_height = 50; // Height in pixels

    // In WebGL, (0,0) is bottom-left, so we define the navbar at the top
    gl.viewport(0, height - navbar_height, width, navbar_height);
    gl.scissor(0, height - navbar_height, width, navbar_height);

    // No need to clear or draw a background since the animation is already drawn
    // We'll just create the text overlay

    // Create or update the HTML overlay for text
    create_or_update_text_overlay(width)?;

    Ok(())
}

/// Creates or updates an HTML element overlay for the navbar text
fn create_or_update_text_overlay(width: i32) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Check if our text overlay already exists
    let overlay_id = "navbar-text-overlay";
    let overlay = match document.get_element_by_id(overlay_id) {
        Some(element) => element,
        None => create_text_overlay(&document, overlay_id)?,
    };

    // Configure the text overlay - ensure it's positioned correctly
    let overlay = overlay.dyn_into::<HtmlElement>()?;
    overlay.style().set_property("position", "absolute")?;
    overlay.style().set_property("top", "0px")?;
    overlay.style().set_property("left", "0px")?;
    overlay
        .style()
        .set_property("width", &format!("{}px", width))?;
    overlay.style().set_property("height", "50px")?; // Fixed navbar height
    overlay.style().set_property("display", "flex")?;
    overlay.style().set_property("align-items", "center")?;
    overlay.style().set_property("justify-content", "center")?;

    // Apply futuristic styling with semi-transparency to show animation through
    overlay
        .style()
        .set_property("font-family", "'Orbitron', 'Arial', sans-serif")?;
    overlay.style().set_property("font-size", "28px")?;
    overlay.style().set_property("font-weight", "bold")?;
    overlay.style().set_property("letter-spacing", "3px")?;
    overlay.style().set_property("color", "#8DF9FF")?; // Cyan color
    overlay
        .style()
        .set_property("text-shadow", "0 0 10px #00FFFF, 0 0 20px #00BFFF")?; // Glow effect
    overlay.style().set_property("pointer-events", "auto")?; // Make it interactive
    overlay
        .style()
        .set_property("background", "rgba(0, 10, 20, 0.3)")?; // Semi-transparent dark background
    overlay
        .style()
        .set_property("border-bottom", "1px solid rgba(139, 249, 255, 0.3)")?; // Subtle TRON-like border
    overlay
        .style()
        .set_property("box-shadow", "0 0 15px rgba(0, 191, 255, 0.2)")?; // Subtle glow

    // Set the cryptic title text
    overlay.set_inner_text("NEXUS-7 // QUANTUM LINK");

    Ok(())
}

/// Creates a new HTML element for text overlay
fn create_text_overlay(document: &Document, id: &str) -> Result<web_sys::Element, JsValue> {
    let overlay = document.create_element("div")?;
    overlay.set_id(id);

    // Add custom styling for the futuristic font directly to the overlay
    // instead of trying to add a link element to the head
    let style = document.create_element("style")?;
    style.set_text_content(Some(
        "@import url('https://fonts.googleapis.com/css2?family=Orbitron:wght@700&display=swap');",
    ));

    // Append the style to the document body
    document.body().unwrap().append_child(&style)?;

    // Append overlay to body to ensure it's above the canvas
    document.body().unwrap().append_child(&overlay)?;

    Ok(overlay)
}
