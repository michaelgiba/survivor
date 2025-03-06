use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::f64::consts::PI;

#[wasm_bindgen]
pub fn draw_circles(canvas_id: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id)
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;
    
    let canvas_width = canvas.width() as f64;
    let canvas_height = canvas.height() as f64;
    
    let center_x = canvas_width / 2.0;
    let center_y = canvas_height / 2.0;
    
    // Calculate sizes based on canvas dimensions
    let radius_reference = canvas_width.min(canvas_height) * 0.15;
    let orbit_radius = radius_reference * 2.5;
    let small_radius = radius_reference * 0.4;
    
    // Draw 11 green circles in an orbit
    for i in 0..11 {
        let angle = (i as f64) * (2.0 * PI / 11.0);
        let x = center_x + orbit_radius * angle.cos();
        let y = center_y + orbit_radius * angle.sin();
        
        context.begin_path();
        context.set_fill_style(&JsValue::from_str("green"));
        context.arc(x, y, small_radius, 0.0, 2.0 * PI)?;
        context.fill();
    }
    
    Ok(())
}
