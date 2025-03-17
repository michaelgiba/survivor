use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;
// Import our UI interactions module

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

    Ok(())
}
