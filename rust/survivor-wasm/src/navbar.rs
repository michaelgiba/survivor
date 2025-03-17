use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, WebGlRenderingContext};

/// Draws the navbar region of the application
pub fn draw_navbar(gl: &WebGlRenderingContext, width: i32, height: i32) -> Result<(), JsValue> {
    // Define the navbar region (top strip of the screen)
    let navbar_height = 50; // Height in pixels

    // In WebGL, (0,0) is bottom-left, so we define the navbar at the top
    gl.viewport(0, height - navbar_height, width, navbar_height);
    gl.scissor(0, height - navbar_height, width, navbar_height);

    // Create or update the HTML overlay for text and controls
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

    let style = format!(
        "position: absolute; \
        top: 0px; \
        left: 0px; \
        width: {}px; \
        height: 50px; \
        display: flex; \
        align-items: center; \
        justify-content: space-between; \
        padding: 0 20px; \
        font-family: 'Orbitron', 'Arial', sans-serif; \
        font-size: 28px; \
        font-weight: bold; \
        letter-spacing: 3px; \
        color: #8DF9FF; \
        text-shadow: 0 0 10px #00FFFF, 0 0 20px #00BFFF; \
        pointer-events: auto; \
        background: rgba(0, 10, 20, 0.3); \
        border-bottom: 1px solid rgba(139, 249, 255, 0.3); \
        box-shadow: 0 0 15px rgba(0, 191, 255, 0.2); \
        z-index: 1000;",
        width
    );

    overlay.set_attribute("style", &style)?;

    // Create the content with title and selector
    overlay.set_inner_html(&format!(
        r#"
        <div style="flex: 1;">NEXUS-7 // QUANTUM LINK</div>
        <select id="rolloutSelector" style="
            background: rgba(0, 20, 40, 0.8);
            border: 1px solid rgba(139, 249, 255, 0.5);
            color: #8DF9FF;
            font-family: 'Orbitron', sans-serif;
            font-size: 16px;
            padding: 5px 10px;
            border-radius: 4px;
            outline: none;
            margin-left: 20px;
            cursor: pointer;
            text-shadow: 0 0 5px #00FFFF;
            transition: all 0.3s ease;
        ">
            <option value="">Loading rollouts...</option>
        </select>
        "#
    ));

    Ok(())
}

/// Creates a new HTML element for text overlay
fn create_text_overlay(document: &Document, id: &str) -> Result<web_sys::Element, JsValue> {
    let overlay = document.create_element("div")?;
    overlay.set_id(id);

    // Add custom styling for the futuristic font
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
