use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, MouseEvent, WebGlRenderingContext};
// Import our UI interactions module
use crate::ui_interactions;

/// Draws the left sidebar region of the application
pub fn draw_sidebar(gl: &WebGlRenderingContext, _width: i32, height: i32) -> Result<(), JsValue> {
    // Define the sidebar region (left strip of the screen)
    let sidebar_width = 200; // Width in pixels
    let navbar_height = 50; // Height of navbar to avoid overlap

    // In WebGL, (0,0) is bottom-left
    gl.viewport(0, 0, sidebar_width, height - navbar_height);
    gl.scissor(0, 0, sidebar_width, height - navbar_height);

    // No need to clear or draw a background since the animation is already drawn
    // We'll just create the buttons overlay

    // Create or update the HTML overlay for sidebar buttons
    create_or_update_buttons_grid(sidebar_width, height - navbar_height)?;

    Ok(())
}

/// Creates or updates the grid of buttons in the sidebar
fn create_or_update_buttons_grid(width: i32, height: i32) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Check if our sidebar overlay already exists
    let overlay_id = "sidebar-buttons-grid";
    let overlay = match document.get_element_by_id(overlay_id) {
        Some(element) => element,
        None => create_sidebar_overlay(&document, overlay_id)?,
    };

    // Configure the sidebar overlay with semi-transparency to show animation through
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
    overlay.style().set_property("flex-wrap", "wrap")?;
    overlay
        .style()
        .set_property("align-content", "flex-start")?;
    overlay.style().set_property("justify-content", "center")?;
    overlay.style().set_property("padding", "10px")?;
    overlay
        .style()
        .set_property("background", "rgba(0, 10, 20, 0.2)")?; // Semi-transparent dark background
    overlay
        .style()
        .set_property("border-right", "1px solid rgba(139, 249, 255, 0.3)")?; // Subtle TRON-like border
    overlay
        .style()
        .set_property("box-shadow", "0 0 15px rgba(0, 191, 255, 0.2)")?; // Subtle glow

    // Check if we already have buttons
    if overlay.children().length() == 0 {
        // Define cryptic button data: (icon, hidden text for main area display)
        let button_data = [
            ("⬢", "NEXUS"),
            ("⟁", "CIPHER"),
            ("⧊", "QUANTUM"),
            ("⧉", "MATRIX"),
            ("⧇", "VERTEX"),
            ("⌬", "SECTOR"),
            ("◈", "NEURAL"),
            ("⬡", "PHOTON"),
        ];

        // Create and add each button (with only symbols shown)
        for (idx, (icon, text)) in button_data.iter().enumerate() {
            let button = create_symbol_only_button(&document, icon, text, idx)?;
            overlay.append_child(&button)?;
        }
    }

    Ok(())
}

/// Creates a sidebar button with only the symbol shown (no text)
fn create_symbol_only_button(
    document: &Document,
    icon: &str,
    hidden_text: &str,
    index: usize,
) -> Result<HtmlElement, JsValue> {
    let button = document.create_element("div")?.dyn_into::<HtmlElement>()?;

    // Set unique ID and base styling
    button.set_id(&format!("sidebar-button-{}", index));
    button.set_class_name("sidebar-button");

    // Apply TRON-like styling with transparency to let animation show through
    button.style().set_property("width", "80px")?;
    button.style().set_property("height", "80px")?;
    button.style().set_property("margin", "5px")?;
    button
        .style()
        .set_property("background-color", "rgba(16, 24, 32, 0.5)")?; // More transparent
    button.style().set_property("border", "1px solid #8DF9FF")?;
    button
        .style()
        .set_property("box-shadow", "0 0 10px #00BFFF, inset 0 0 5px #00BFFF")?;
    button.style().set_property("color", "#8DF9FF")?;
    button.style().set_property("display", "flex")?;
    button.style().set_property("justify-content", "center")?;
    button.style().set_property("align-items", "center")?;
    button.style().set_property("cursor", "pointer")?;
    button.style().set_property("transition", "all 0.2s ease")?;
    button
        .style()
        .set_property("font-family", "'Orbitron', 'Arial', sans-serif")?;
    button.style().set_property("font-size", "32px")?; // Larger font for the icon
    button
        .style()
        .set_property("backdrop-filter", "blur(3px)")?; // Add blur effect behind buttons

    // Set the icon as the button's text
    button.set_inner_text(icon);

    // Add click event to show the hidden text in main area
    let text_to_display = hidden_text.to_string();
    let click_handler = Closure::wrap(Box::new(move |_event: MouseEvent| {
        ui_interactions::set_active_selection(&text_to_display);

        // Refresh the main region to display the new text
        if let Some(main_area) = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("main-region-overlay")
        {
            if let Ok(main_element) = main_area.dyn_into::<HtmlElement>() {
                main_element.set_inner_text(&text_to_display);
            }
        }
    }) as Box<dyn FnMut(MouseEvent)>);

    button.set_onclick(Some(click_handler.as_ref().unchecked_ref()));
    click_handler.forget(); // Prevent closure from being dropped

    // Add hover effects
    let mouseover_handler = Closure::wrap(Box::new(move |_event: MouseEvent| {
        if let Some(target) = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(&format!("sidebar-button-{}", index))
        {
            if let Ok(button_el) = target.dyn_into::<HtmlElement>() {
                button_el
                    .style()
                    .set_property("background-color", "rgba(30, 40, 50, 0.7)")
                    .unwrap();
                button_el
                    .style()
                    .set_property("box-shadow", "0 0 15px #00FFFF, inset 0 0 8px #00FFFF")
                    .unwrap();
                button_el
                    .style()
                    .set_property("transform", "scale(1.05)")
                    .unwrap();
            }
        }
    }) as Box<dyn FnMut(MouseEvent)>);

    let mouseout_handler = Closure::wrap(Box::new(move |_event: MouseEvent| {
        if let Some(target) = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(&format!("sidebar-button-{}", index))
        {
            if let Ok(button_el) = target.dyn_into::<HtmlElement>() {
                button_el
                    .style()
                    .set_property("background-color", "rgba(16, 24, 32, 0.5)")
                    .unwrap();
                button_el
                    .style()
                    .set_property("box-shadow", "0 0 10px #00BFFF, inset 0 0 5px #00BFFF")
                    .unwrap();
                button_el
                    .style()
                    .set_property("transform", "scale(1.0)")
                    .unwrap();
            }
        }
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

    // Add custom styling for the futuristic font
    let style = document.create_element("style")?;
    style.set_text_content(Some("@import url('https://fonts.googleapis.com/css2?family=Orbitron:wght@400;700&display=swap');"));

    // Append the style to the document body if it doesn't exist yet
    if document
        .get_element_by_id("futuristic-font-style")
        .is_none()
    {
        style.set_id("futuristic-font-style");
        document.head().unwrap().append_child(&style)?;
    }

    // Append overlay to body
    document.body().unwrap().append_child(&overlay)?;

    Ok(overlay)
}
