use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, HtmlCanvasElement, KeyEvent, MouseEvent, WebGl2RenderingContext};

use super::quad::Quad;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct App {
    canvas: HtmlCanvasElement,
    gl: WebGl2RenderingContext,
    quad: Quad,
}

impl App {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let gl = get_gl_context(&canvas).expect("No GL Canvas");

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.enable(WebGl2RenderingContext::DEPTH_TEST);
        gl.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        if gl.is_null() {
            panic!("No Webg1")
        }

        let quad = match Quad::new(&gl) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("Quad error {:?}", err));
                panic!("Quad error");
            }
        };

        Self { canvas, gl, quad }
    }

    fn check_resize(&mut self) {
        let client_width = self.canvas.client_width();
        let client_height = self.canvas.client_height();
        let canvas_width = self.canvas.width() as i32;
        let canvas_height = self.canvas.height() as i32;

        if client_width != canvas_width || client_height != canvas_height {
                        self.gl.viewport(0, 0, client_width, client_height);
           let client_width = client_width as u32;
           let client_height = client_height as u32;

           self.canvas.set_width(client_width);
           self.canvas.set_height(client_height);
           self.quad.resolution = (client_width, client_height);

            log(&format!("Resized to {}:{}", client_width, client_height));
        }
    }

    pub fn animation_frame(&mut self) {
        self.check_resize();
        self.gl.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        let now = window().unwrap().performance().unwrap().now();
        let time = (now / 1000.0) as f32;
        self.quad.time = time;

        self.quad.render(&self.gl)
    }

    pub fn mouse_event(&mut self, event: MouseEvent) {
        log(&format!("Mouse Event {:?}", event))
    }

    pub fn key_event(&mut self, event: KeyEvent) {
        log(&format!("Key Event {:?}", event))
    }
}

fn get_gl_context(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext, JsValue> {
    Ok(canvas.get_context("webgl2")?.unwrap().dyn_into()?)
}
