use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{HtmlCanvasElement, KeyEvent, MouseEvent, WebGl2RenderingContext};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct App {
    canvas: HtmlCanvasElement,
    gl: WebGl2RenderingContext,
}

impl App {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        Self { canvas }
    }

    pub fn animation_frame(&mut self) {
        log("Animation Frame")
    }

    pub fn mouse_event(&mut self, event: MouseEvent) {
        log(&format!("Mouse Event {:?}", event))
    }

    pub fn key_event(&mut self, event: KeyEvent) {
        log(&format!("Key Event {:?}", event))
    }
}

fn get_gl_context(canvas: &HtmlCanvasElemtn) -> Result<WebGl2RenderingContext, JsValue> {
    Ok(canvas.get_context("webgl2")?.unwrap().dyn_into()?)
}
