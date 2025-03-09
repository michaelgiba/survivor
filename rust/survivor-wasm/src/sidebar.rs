use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, MouseEvent, WebGlRenderingContext};
// Import our UI interactions module
use crate::ui_interactions;

/// Draws the left sidebar region of the application
pub fn draw_sidebar(gl: &WebGlRenderingContext, _width: i32, height: i32) -> Result<(), JsValue> {
    // Define the sidebar region (left strip of the screen)
    let sidebar_width = 100; // Width in pixels (reduced from 200)
    let navbar_height = 50; // Height of navbar to avoid overlap

    // In WebGL, (0,0) is bottom-left
    gl.viewport(0, 0, sidebar_width, height - navbar_height);
    gl.scissor(0, 0, sidebar_width, height - navbar_height);

    // No need to clear or draw a background since the animation is already drawn
    // We'll just create the buttons overlay

    // Create or update the HTML overlay for sidebar with chat button
    create_or_update_chat_button(sidebar_width, height - navbar_height)?;

    Ok(())
}

/// Creates or updates the chat button in the sidebar
fn create_or_update_chat_button(width: i32, height: i32) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Check if our sidebar overlay already exists
    let overlay_id = "sidebar-chat-container";
    let overlay = match document.get_element_by_id(overlay_id) {
        Some(element) => element,
        None => create_sidebar_overlay(&document, overlay_id)?,
    };

    // Configure the sidebar overlay with retro terminal styling
    let overlay = overlay.dyn_into::<HtmlElement>()?;
    overlay.style().set_property("position", "absolute")?;
    overlay.style().set_property("top", "50px")?; // Position below navbar
    overlay.style().set_property("left", "0px")?;
    overlay
        .style()
        .set_property("width", &format!("{}px", width))?;
    overlay
        .style()
        .set_property("height", &format!("{}px", height))?;
    overlay.style().set_property("display", "flex")?;
    overlay.style().set_property("flex-direction", "column")?;
    overlay.style().set_property("align-items", "center")?;
    overlay.style().set_property("padding-top", "20px")?;
    overlay
        .style()
        .set_property("background", "rgba(0, 20, 40, 0.4)")?; // More opaque dark terminal background
    overlay
        .style()
        .set_property("border-right", "2px solid rgba(139, 249, 255, 0.5)")?; // More visible terminal-like border
    overlay
        .style()
        .set_property("box-shadow", "inset -5px 0 15px rgba(0, 0, 0, 0.5)")?; // Shadow to enhance CRT look

    // Add scanlines effect to sidebar to enhance retro feel
    overlay.style().set_property("background-image", "linear-gradient(
        to bottom,
        rgba(0, 0, 0, 0.1) 0%,
        rgba(0, 0, 0, 0) 50%,
        rgba(0, 0, 0, 0.1) 51%,
        rgba(0, 0, 0, 0) 100%
    )")?;
    overlay.style().set_property("background-size", "100% 4px")?;
    overlay.style().set_property("background-repeat", "repeat")?;

    // Check if we already have the chat button
    if overlay.children().length() == 0 {
        // Create and add terminal-like heading
        let terminal_header = document.create_element("div")?.dyn_into::<HtmlElement>()?;
        terminal_header.set_id("terminal-header");
        terminal_header.style().set_property("color", "#8DF9FF")?;
        terminal_header.style().set_property("font-family", "'VT323', monospace")?;
        terminal_header.style().set_property("font-size", "18px")?;
        terminal_header.style().set_property("width", "100%")?;
        terminal_header.style().set_property("text-align", "center")?;
        terminal_header.style().set_property("margin-bottom", "15px")?;
        terminal_header.set_inner_text("TERMINAL");
        overlay.append_child(&terminal_header)?;

        // Create and add the chat button that looks like a terminal command
        let button = create_chat_button(&document)?;
        overlay.append_child(&button)?;
    }

    Ok(())
}

/// Creates a chat button with an icon that looks like a terminal command
fn create_chat_button(document: &Document) -> Result<HtmlElement, JsValue> {
    let button = document.create_element("div")?.dyn_into::<HtmlElement>()?;

    // Set unique ID and base styling
    button.set_id("chat-button");
    button.set_class_name("terminal-command");

    // Apply retro terminal styling
    button.style().set_property("width", "90%")?;
    button.style().set_property("margin", "5px")?;
    button.style().set_property("padding", "8px")?;
    button
        .style()
        .set_property("background-color", "rgba(0, 30, 50, 0.7)")?; // Terminal command background
    button.style().set_property("border", "1px solid #8DF9FF")?;
    button.style().set_property("color", "#8DF9FF")?;
    button.style().set_property("font-family", "'VT323', monospace")?;
    button.style().set_property("font-size", "16px")?;
    button.style().set_property("text-align", "left")?;
    button.style().set_property("cursor", "pointer")?;
    button.style().set_property("transition", "all 0.2s ease")?;
    button.style().set_property("position", "relative")?; // For cursor positioning
    button.style().set_property("overflow", "hidden")?;

    // Set the content to look like a terminal command with prompt
    button.set_inner_html("> <span style='color: #50FF50;'>CHAT</span>");

    // Add blinking cursor after the command
    let cursor = document.create_element("span")?.dyn_into::<HtmlElement>()?;
    cursor.set_inner_text("_");
    cursor.style().set_property("animation", "blink 1s step-end infinite")?;
    cursor.style().set_property("margin-left", "4px")?;
    button.append_child(&cursor)?;

    // Add click event to show the chat interface in main area
    let click_handler = Closure::wrap(Box::new(move |_event: MouseEvent| {
        ui_interactions::set_active_selection("CHAT_MODE");
        
        // Refresh the main region to display the chat interface
        if let Some(main_area) = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("main-region-overlay")
        {
            if let Ok(main_element) = main_area.dyn_into::<HtmlElement>() {
                // Set up the retro chat interface (will be enhanced in main_region.rs)
                main_element.set_inner_text("");
                
                // Simulate command executed message
                let cmd_result = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .create_element("div")
                    .unwrap()
                    .dyn_into::<HtmlElement>()
                    .unwrap();
                    
                cmd_result.style().set_property("color", "#50FF50").unwrap();
                cmd_result.set_inner_text("INITIALIZING COMMUNICATION MODULE...");
                main_element.append_child(&cmd_result).unwrap();
                
                web_sys::console::log_1(&"Chat interface activated".into());
            }
        }
    }) as Box<dyn FnMut(MouseEvent)>);

    button.set_onclick(Some(click_handler.as_ref().unchecked_ref()));
    click_handler.forget(); // Prevent closure from being dropped

    // Add hover effects for terminal-like feedback
    let button_clone = button.clone();
    let mouseover_handler = Closure::wrap(Box::new(move |_event: MouseEvent| {
        // Create a terminal selection effect
        button_clone
            .style()
            .set_property("background-color", "rgba(0, 60, 90, 0.9)")
            .unwrap();
        button_clone
            .style()
            .set_property("box-shadow", "0 0 10px #00FFFF")
            .unwrap();
    }) as Box<dyn FnMut(MouseEvent)>);

    let button_clone = button.clone();
    let mouseout_handler = Closure::wrap(Box::new(move |_event: MouseEvent| {
        button_clone
            .style()
            .set_property("background-color", "rgba(0, 30, 50, 0.7)")
            .unwrap();
        button_clone
            .style()
            .set_property("box-shadow", "none")
            .unwrap();
    }) as Box<dyn FnMut(MouseEvent)>);

    button.set_onmouseover(Some(mouseover_handler.as_ref().unchecked_ref()));
    button.set_onmouseout(Some(mouseout_handler.as_ref().unchecked_ref()));

    // Prevent closures from being dropped
    mouseover_handler.forget();
    mouseout_handler.forget();

    Ok(button)
}

/// Creates a new HTML element for the sidebar overlay
fn create_sidebar_overlay(document: &Document, id: &str) -> Result<web_sys::Element, JsValue> {
    let overlay = document.create_element("div")?;
    overlay.set_id(id);

    // Add custom styling for the retro font
    let style = document.create_element("style")?;
    style.set_text_content(Some("@import url('https://fonts.googleapis.com/css2?family=VT323&display=swap');"));

    // Append the style to the document body if it doesn't exist yet
    if document
        .get_element_by_id("retro-font-style")
        .is_none()
    {
        style.set_id("retro-font-style");
        document.head().unwrap().append_child(&style)?;
    }

    // Append overlay to body
    document.body().unwrap().append_child(&overlay)?;

    Ok(overlay)
}
