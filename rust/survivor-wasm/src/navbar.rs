use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, WebGlRenderingContext};
use crate::ui_interactions;

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

    // Create the CHAT button
    create_or_update_chat_button(&document, &overlay)?;

    Ok(())
}

/// Creates or updates the CHAT button
fn create_or_update_chat_button(document: &Document, parent: &HtmlElement) -> Result<(), JsValue> {
    // Check if the button already exists
    let button_id = "chat-button";
    let button = match document.get_element_by_id(button_id) {
        Some(element) => element,
        None => {
            // Create a new button element
            let btn = document.create_element("div")?;
            btn.set_id(button_id);
            parent.append_child(&btn)?;
            btn
        }
    };

    // Style the button
    let button = button.dyn_into::<HtmlElement>()?;
    button.style().set_property("cursor", "pointer")?;
    button.style().set_property("padding", "5px 20px")?;
    button.style().set_property("border", "2px solid #8DF9FF")?;
    button.style().set_property("border-radius", "4px")?;
    button.style().set_property("background", "rgba(0, 40, 60, 0.5)")?;
    button.style().set_property("color", "#8DF9FF")?;
    button.style().set_property("transition", "all 0.3s")?;
    
    // Set the text
    button.set_inner_text("CHAT");
    
    // Add hover effect using classList and mouseover/mouseout events
    let button_clone = button.clone();
    let hover_callback = Closure::wrap(Box::new(move || {
        let _ = button_clone.style().set_property("background", "rgba(0, 60, 90, 0.7)");
        let _ = button_clone.style().set_property("box-shadow", "0 0 15px rgba(0, 191, 255, 0.5)");
    }) as Box<dyn FnMut()>);
    
    let button_clone = button.clone();
    let unhover_callback = Closure::wrap(Box::new(move || {
        let _ = button_clone.style().set_property("background", "rgba(0, 40, 60, 0.5)");
        let _ = button_clone.style().set_property("box-shadow", "none");
    }) as Box<dyn FnMut()>);
    
    // Set up click handler to activate chat mode
    let click_callback = Closure::wrap(Box::new(move || {
        ui_interactions::set_active_selection("CHAT_MODE");
        
        // Here we would trigger the display of the retro chat window
        // For now, we just set the selection to indicate chat mode is active
        web_sys::console::log_1(&"Chat button clicked".into());
    }) as Box<dyn FnMut()>);
    
    // Add event listeners
    button.add_event_listener_with_callback("mouseover", hover_callback.as_ref().unchecked_ref())?;
    button.add_event_listener_with_callback("mouseout", unhover_callback.as_ref().unchecked_ref())?;
    button.add_event_listener_with_callback("click", click_callback.as_ref().unchecked_ref())?;
    
    // Prevent callbacks from being garbage collected
    hover_callback.forget();
    unhover_callback.forget();
    click_callback.forget();

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
