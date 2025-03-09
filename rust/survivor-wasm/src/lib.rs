use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

// Import our modules
mod animation;
mod main_region;
mod navbar;
mod sidebar;
mod ui_interactions;

#[wasm_bindgen]
pub fn draw_main(canvas_id: &str) -> Result<(), JsValue> {
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

    // First, draw the animation as a background across the entire canvas
    animation::draw_background(&gl, width, height)?;

    // Enable scissor test for drawing UI panels
    gl.enable(WebGlRenderingContext::SCISSOR_TEST);

    // Draw UI panels (which will be semi-transparent to show animation)
    sidebar::draw_sidebar(&gl, width, height)?;
    main_region::draw_main_region(&gl, width, height)?;
    navbar::draw_navbar(&gl, width, height)?;

    // Disable scissor test when done
    gl.disable(WebGlRenderingContext::SCISSOR_TEST);

    Ok(())
}
