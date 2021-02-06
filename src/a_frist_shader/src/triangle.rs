use wasm_bindgen::{JsCast, JsValue};
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

#[derive(Debug)]
pub enum TriangleError {
    /// Failed to upload buffer data to the GPU
    BufferCreationFailed,

    /// An unhandled/unspecified error
    JsError(JsValue)
}

impl From<JsValue> for TriangleError {
    fn from(err: JsValue) -> TriangleError {
        TriangleError::JsError(err)
    }
}

pub struct FirstTriangle {}

impl FirstTriangle {
    pub fn new(gl: &WebGl2RenderingContext) -> Result<Self, TriangleError> {
        let position_buffer = upload_array_f32(gl, vec![-1.0, 1.0, 1.0, 1.0, 0.0, -1.0])?;
        Ok(Self {})
    }

    pub fn render(&mut self, gl: &WebGl2RenderingContext) {}
}

fn upload_array_f32(
    gl: &WebGl2RenderingContext,
    vertices: Vec<f32>,
) -> Result<WebGlBuffer, TriangleError> {
    let position_buffer = gl
        .create_buffer()
        .ok_or(TriangleError::BufferCreationFailed)?;

    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));

    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<js_sys::WebAssembly::Memory>()?
        .buffer();

    let vertices_location = vertices.as_ptr() as u32 / 4;

    let vert_array = js_sys::Float32Array::new(&memory_buffer)
        .subarray(vertices_location, vertices_location + vertices.len() as u32);

    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &vert_array,
        WebGl2RenderingContext::STATIC_DRAW,
    );

    Ok(position_buffer)
}
