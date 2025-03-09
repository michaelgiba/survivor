use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader};

// Store animation state using thread_local storage
thread_local! {
    static ANIMATION: RefCell<Option<Animation>> = RefCell::new(None);
    static ANIMATION_ACTIVE: RefCell<bool> = RefCell::new(false);
}

// Structure to hold animation state
pub struct Animation {
    program: WebGlProgram,
    vertex_buffer: WebGlBuffer,
    start_time: f64,
    current_time: f64,
    resolution_location: Option<web_sys::WebGlUniformLocation>,
    time_location: Option<web_sys::WebGlUniformLocation>,
}

impl Animation {
    pub fn new(gl: &WebGlRenderingContext) -> Result<Self, JsValue> {
        // Create shaders and program
        let vert_shader = compile_shader(
            gl,
            WebGlRenderingContext::VERTEX_SHADER,
            r#"
            attribute vec4 position;
            void main() {
                gl_Position = position;
            }
            "#,
        )?;

        // Simplified, more optimized fragment shader
        let frag_shader = compile_shader(
            gl,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r#"
            precision mediump float;
            
            uniform vec2 resolution;
            uniform float time;
            
            // Simplified plasma function for better performance
            vec3 plasma(vec2 uv, float time) {
                float v1 = sin(uv.x * 6.0 + (time * 2.0) * 0.2);
                float v2 = sin(uv.y * 6.0 - (time * 2.0) * 0.3);
                float v3 = sin((uv.x + uv.y) * 4.0 + (time * 2.0) * 0.2);
                
                // Simpler calculation
                float v = v1 + v2 + v3;
                
                // Create color
                vec3 col;
                col.r = 0.1 * sin(v * 1.5 + (time * 2.0) * 0.5) * 0.5 + 0.5;
                col.g = 0.1 * sin(v * 1.0 + (time * 2.0) * 0.4) * 0.5 + 0.5;
                col.b = 0.1 * sin(v * 0.5 + (time * 2.0) * 0.6) * 0.5 + 0.5;
                
                return col;
            }
        
            
            void main() {
                // Normalized coordinates
                vec2 uv = gl_FragCoord.xy / resolution.xy;
                uv = uv * 2.0 - 1.0;  // Center at (0,0)
                uv.x *= resolution.x / resolution.y;  // Correct aspect ratio
                
                // Simpler plasma background
                vec3 color = plasma(uv * 0.5, (time * 2.0));

                // Vignette effect
                float dist = length(uv) * 0.7;
                color *= 1.0 - dist * dist;
                
                // TRON-like blue color emphasis
                color = mix(color, vec3(0.1, 0.4, 0.9), 0.2);
                
                gl_FragColor = vec4(color, 1.0);
            }
            "#,
        )?;

        // Create and link program
        let program = link_program(gl, &vert_shader, &frag_shader)?;
        gl.use_program(Some(&program));

        // Create vertex buffer (full screen quad)
        let vertices: [f32; 12] = [
            -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        ];

        let vertex_buffer = gl.create_buffer().ok_or("Failed to create buffer")?;
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

        // Note: `memory_buffer` is used to convert Rust array to JavaScript TypedArray
        unsafe {
            let view = js_sys::Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &view,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        // Get attribute and uniform locations
        let position_location = gl.get_attrib_location(&program, "position");
        gl.vertex_attrib_pointer_with_i32(
            position_location as u32,
            2,                            // 2 components per vertex (x,y)
            WebGlRenderingContext::FLOAT, // Data type
            false,                        // Don't normalize
            0,                            // Stride
            0,                            // Offset
        );
        gl.enable_vertex_attrib_array(position_location as u32);

        let resolution_location = gl.get_uniform_location(&program, "resolution");
        let time_location = gl.get_uniform_location(&program, "time");

        // Get current time for animation
        let now = js_sys::Date::now();

        Ok(Animation {
            program,
            vertex_buffer,
            start_time: now,
            current_time: now,
            resolution_location,
            time_location,
        })
    }

    pub fn render(
        &mut self,
        gl: &WebGlRenderingContext,
        width: i32,
        height: i32,
    ) -> Result<(), JsValue> {
        // Update time
        self.current_time = js_sys::Date::now();
        let elapsed = (self.current_time - self.start_time) / 1000.0; // Convert to seconds

        // Use our shader program
        gl.use_program(Some(&self.program));

        // Update uniforms
        gl.uniform2f(
            self.resolution_location.as_ref(),
            width as f32,
            height as f32,
        );
        gl.uniform1f(self.time_location.as_ref(), elapsed as f32);

        // Bind the vertex buffer
        gl.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.vertex_buffer),
        );

        // Draw the full-screen quad
        gl.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0, // Starting index
            6, // Number of vertices (2 triangles)
        );

        Ok(())
    }
}

// Get or create the animation singleton
fn get_or_create_animation(gl: &WebGlRenderingContext) -> Result<(), JsValue> {
    ANIMATION.with(|animation| {
        if animation.borrow().is_none() {
            let new_animation = Animation::new(gl)?;
            animation.replace(Some(new_animation));

            // Start the animation loop if it's not already running
            ANIMATION_ACTIVE.with(|active| {
                if !*active.borrow() {
                    *active.borrow_mut() = true;
                    if let Err(err) = start_animation_loop() {
                        web_sys::console::error_1(&err);
                    }
                }
            });
        }
        Ok(()) as Result<(), JsValue>
    })
}

// Draw the animation as a full-screen background
pub fn draw_background(gl: &WebGlRenderingContext, width: i32, height: i32) -> Result<(), JsValue> {
    // Set the viewport to cover the entire canvas
    gl.viewport(0, 0, width, height);

    // Initialize or get the animation
    get_or_create_animation(gl)?;

    // Render the animation frame
    ANIMATION.with(|animation| -> Result<(), JsValue> {
        if let Some(anim) = &mut *animation.borrow_mut() {
            anim.render(gl, width, height)?;
        }
        Ok(())
    })
}

// Start the animation loop properly, with only a single requestAnimationFrame chain
fn start_animation_loop() -> Result<(), JsValue> {
    // Static closure that will be called repeatedly for animation
    thread_local! {
        static ANIMATION_FRAME_CLOSURE: RefCell<Option<Closure<dyn FnMut()>>> = RefCell::new(None);
    }

    // Initialize the animation frame closure if it doesn't exist
    ANIMATION_FRAME_CLOSURE.with(|cell| {
        if cell.borrow().is_none() {
            let closure = Closure::wrap(Box::new(|| {
                // Only redraw the canvas, without requesting another frame inside draw_main
                let _ = crate::draw_main("canvas");

                // Schedule the next frame
                if let Some(window) = web_sys::window() {
                    ANIMATION_FRAME_CLOSURE.with(|cell| {
                        if let Some(ref closure) = *cell.borrow() {
                            let _ =
                                window.request_animation_frame(closure.as_ref().unchecked_ref());
                        }
                    });
                }
            }) as Box<dyn FnMut()>);

            // Store the closure
            *cell.borrow_mut() = Some(closure);

            // Start the animation loop
            if let Some(window) = web_sys::window() {
                if let Some(ref closure) = *cell.borrow() {
                    window.request_animation_frame(closure.as_ref().unchecked_ref())?;
                }
            }
        }
        Ok(()) as Result<(), JsValue>
    })
}

fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| "Failed to create shader".to_string())?;

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        let error = gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error".to_string());
        Err(error)
    }
}

fn link_program(
    gl: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| "Failed to create program".to_string())?;

    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        let error = gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error".to_string());
        Err(error)
    }
}
