use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, HtmlCanvasElement, KeyEvent, MouseEvent, WebGl2RenderingContext};

use super::map_sprite::MapSprite;
use super::ship_sprite::ShipSprite;
use super::transform::Transform2d;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct App {
    canvas: HtmlCanvasElement,
    gl: WebGl2RenderingContext,
    ship_sprite: ShipSprite,
    map_sprite: MapSprite,
    canvas_resolution: (u32, u32),
}

impl App {
    pub fn new(canvas: HtmlCanvasElement, options: String) -> Self {
        let gl = get_gl_context(&canvas).expect("No GL Canvas");

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        //gl.enable(WebGl2RenderingContext::DEPTH_TEST);
        gl.enable(WebGl2RenderingContext::BLEND);
        gl.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        if gl.is_null() {
            panic!("No Webg1")
        }

        let ship_sprite = match ShipSprite::new(&gl) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("error {:?}", err));
                panic!("error");
            }
        };

        let map_sprite = match MapSprite::new(&gl, options) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("error {:?}", err));
                panic!("error");
            }
        };

        Self {
            canvas,
            gl,
            ship_sprite,
            map_sprite,
            canvas_resolution: (0, 0),
        }
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
            self.canvas_resolution = (client_width, client_height);

            log(&format!("Resized to {}:{}", client_width, client_height));
        }
    }

    pub fn animation_frame(&mut self) {
        let now = window().unwrap().performance().unwrap().now();
        let time = (now / 1000.0) as f32;

        self.check_resize();
        self.gl.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        let camera_transform =
            Transform2d::new(0.0, 0.0, 0.0, 1.0 / self.canvas_resolution.1 as f32);
        let world_to_camera = camera_transform.to_mat3_array();
        let camera_to_clipspace = [
            self.canvas_resolution.0 as f32,
            0.0,
            0.0,
            0.0,
            self.canvas_resolution.1 as f32,
            0.0,
            0.0,
            0.0,
            1.0,
        ];

        self.ship_sprite.world_to_camera = world_to_camera;
        self.ship_sprite.camera_to_clipspace = camera_to_clipspace;

        let mut ship_sprite_transform = Transform2d::new(0.0, 0.0, f32::sin(time), 0.1);
        // Render the first ship
        ship_sprite_transform.rot = -std::f32::consts::PI / 6.0 - time;
        self.ship_sprite.world_to_sprite = ship_sprite_transform.to_mat3_array();
        self.ship_sprite.ship_color = (0.0, 0.5, 1.0, 1.0);
        self.ship_sprite.ship_engine = 1.0;
        self.ship_sprite.render(&self.gl);

        // Render another ship
        ship_sprite_transform.x = f32::sin(time) * 0.5;
        ship_sprite_transform.y = f32::cos(time) * 0.5;
        ship_sprite_transform.rot = -std::f32::consts::PI / 2.0 - time;
        self.ship_sprite.world_to_sprite = ship_sprite_transform.to_mat3_array();
        self.ship_sprite.ship_color = (1.0, 0.5, 0.0, 1.0);
        self.ship_sprite.ship_engine = 3.0;
        self.ship_sprite.render(&self.gl);

        let map_sprite_transform = Transform2d::new(0.0, 0.0, 0.0, 1.0);

        // Render the map
        self.map_sprite.world_to_camera = world_to_camera;
        self.map_sprite.camera_to_clipspace = camera_to_clipspace;
        self.map_sprite.world_to_sprite = map_sprite_transform.to_mat3_array();
        self.map_sprite.render(&self.gl);
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
