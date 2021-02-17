use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, HtmlCanvasElement, KeyboardEvent, MouseEvent, WebGl2RenderingContext};

use super::ai::calc_ai_control;
use super::camera::Camera;
use super::engine_trail::EngineTrail;
use super::engine_trail_sprite::EngineTrailSprite;
use super::keymap::{KeyMap, KeyState};
use super::map::Map;
use super::map_sprite::MapSprite;
use super::physics::calc_ship_physics;
use super::ship::Ship;
use super::ship_sprite::ShipSprite;
use super::transform::Transform2d;

const CYAN_SHIP: (f32, f32, f32, f32) = (0.0, 0.5, 1.0, 1.0);
const YELLOW_SHIP: (f32, f32, f32, f32) = (1.0, 0.5, 0.0, 1.0);
const PINK_SHIP: (f32, f32, f32, f32) = (1.0, 0.0, 0.5, 1.0);
const PURPLE_SHIP: (f32, f32, f32, f32) = (0.5, 0.0, 1.0, 1.0);
const WHITE_SHIP: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 1.0);

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
      #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f32;
}

pub struct App {
    canvas: HtmlCanvasElement,
    gl: WebGl2RenderingContext,
    ship_sprite: ShipSprite,
    map_sprite: MapSprite,
    engine_trail_sprite: EngineTrailSprite,
    key_map: KeyMap,
    map: Map,

    prev_time: f64,

    ship_entities: Vec<Ship>,
    engine_trails: Vec<(EngineTrail,EngineTrail,EngineTrail)>,
    camera: Camera,

    canvas_resolution: (u32, u32),
}

impl App {
    pub fn new(canvas: HtmlCanvasElement, _options: String) -> Self {
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
                log(&format!("ship error {:?}", err));
                panic!("error");
            }
        };

        let map_sprite = match MapSprite::new(&gl) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("map error {:?}", err));
                panic!("error");
            }
        };

        let engine_trail_sprite = match EngineTrailSprite::new(&gl) {
            Ok(g) => g,
            Err(err) => {
                log(&format!("engine trail error {:?}", err));
                panic!("engine trail error");
            }
        };

        let ship_entities = vec![
            Ship::new(CYAN_SHIP, Transform2d::new(0.0, 0.0, 0.0, 0.1)),
            Ship::new(YELLOW_SHIP, Transform2d::new(0.0, 0.1, 0.0, 0.1)),
            Ship::new(PINK_SHIP, Transform2d::new(0.0, 0.2, 0.0, 0.1)),
            Ship::new(PURPLE_SHIP, Transform2d::new(0.0, 0.3, 0.0, 0.1)),
            Ship::new(WHITE_SHIP, Transform2d::new(0.0, 0.4, 0.0, 0.1)),
        ];

        let mut engine_trails = vec![];
        for ship in ship_entities.iter() {
              const MAIN_TRAIL_WIDTH: f32 = 0.10;
            const WINGTIP_TRAIL_WIDTH: f32 = 0.02;
            const MAIN_TRAIL_BRIGHTNESS: f32 = 0.3;
            const WINGTIP_TRAIL_BRIGHTNESS: f32 = 1.0;

            engine_trails.push(
                (
                    EngineTrail::new(ship.color.clone(), MAIN_TRAIL_WIDTH, MAIN_TRAIL_BRIGHTNESS),
                    EngineTrail::new(ship.color.clone(), WINGTIP_TRAIL_WIDTH, WINGTIP_TRAIL_BRIGHTNESS),
                    EngineTrail::new(ship.color.clone(),  WINGTIP_TRAIL_WIDTH, WINGTIP_TRAIL_BRIGHTNESS),
                )
            );
        }

        let map = Map {
            sin_consts: [2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            cos_consts: [0.0, -2.0, 0.0, 1.0, 0.0, 0.0, 0.5, 0.0],
            track_base_radius: 8.0,
            track_width: 0.7,
        };

        let camera = Camera::new();

        let now = window().unwrap().performance().unwrap().now();
        let prev_time = now / 1000.0;

        let mut game = Self {
            canvas,
            gl,
            ship_sprite,
            map_sprite,
            engine_trail_sprite,
            map,
            key_map: KeyMap::new(),
            canvas_resolution: (0, 0),
            ship_entities,
            engine_trails,
            camera,
            prev_time,
        };
        game.start_game();
        game
    }

    fn start_game(&mut self) {
        self.camera.reset();
        self.map.randomize();
        self.map_sprite.set_to_map(&self.gl, &self.map);

        {
            const SHIP_SPACING: f32 = 0.12;
            let start_position = self.map.get_start_position();
            let startline_angle = self.map.get_track_direction(start_position.angle);

            let startline_tangent = (f32::cos(startline_angle), f32::sin(startline_angle));
            let startline_normal = (-f32::sin(startline_angle), f32::cos(startline_angle));

            let num_ships = self.ship_entities.len();

            for (id, ship) in self.ship_entities.iter_mut().enumerate() {
                let offset = (id as f32) - ((num_ships - 1) as f32) * 0.5;

                let offset_vec = (
                    (startline_tangent.0 * offset - startline_normal.0) * SHIP_SPACING,
                    (startline_tangent.1 * offset - startline_normal.1) * SHIP_SPACING,
                );

                let ship_start_position = start_position.to_cartesian();
                ship.position.x = ship_start_position.0 + offset_vec.0;
                ship.position.y = ship_start_position.1 + offset_vec.1;
                ship.position.rot = startline_angle;

                ship.velocity.x = 0.0;
                ship.velocity.y = 0.0;
                ship.velocity.rot = 0.0;
            }
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
        let time = now / 1000.0;

        let dt = time - self.prev_time;
        self.prev_time = time;

        {
            // Logic uncomment it to play with computer
            /*   let player_ship = &mut self.ship_entities[0];
            player_ship.linear_thrust = 0.0;
            player_ship.angular_thrust = 0.0;
            if self.key_map.forwards.active() {
                player_ship.linear_thrust = 1.0;
            }
            if self.key_map.backwards.active() {
                player_ship.linear_thrust = -1.0;
            }

            if self.key_map.turn_left.active() {
                player_ship.angular_thrust += 1.0;
            }
            if self.key_map.turn_right.active() {
                player_ship.angular_thrust -= 1.0;
            }

            if self.key_map.turn_right.active() || self.key_map.turn_left.active() {
                if player_ship.linear_thrust < 0.0 {
                    player_ship.linear_thrust = -0.5;
                } else if 0.0 < player_ship.linear_thrust {
                    player_ship.linear_thrust = 0.5;
                }
            }*/

            self.key_map.update();

            //let num_ships = self.ship_entities.len() - 2;
            for (_, ship) in self.ship_entities[0..].iter_mut().enumerate() {
                let skill = f32::max(random(), 0.33);
                calc_ai_control(ship, skill, &self.map);
            }
        }

        {
            // physics
            calc_ship_physics(&mut self.ship_entities, &self.map, dt as f32);
        }

        {
            // camera
            self.camera.target_posiion.0 = self.ship_entities[0].position.x;
            self.camera.target_posiion.1 = self.ship_entities[0].position.y;
            self.camera.target_velocity.0 = self.ship_entities[0].velocity.x;
            self.camera.target_velocity.1 = self.ship_entities[0].velocity.y;
            self.camera.update(dt as f32);
        }

        {
            // Trails
            for (ship, trail) in self.ship_entities.iter().zip(self.engine_trails.iter_mut()) {
                trail.0.update(
                    dt as f32,
                    ship.get_engine_position(),
                    f32::abs(ship.linear_thrust),
                );

                let wingtip_positions = ship.get_wingtip_positions();

                let raw_slip = ship.calc_slip() / 2.5;
                let base_slip = f32::abs(raw_slip);
                let left_slip = base_slip + raw_slip / 8.0;
                let right_slip = base_slip - raw_slip / 8.0;

                trail.1.update(
                    dt as f32,
                    wingtip_positions.0,
                    f32::max(f32::min(left_slip, 1.0), 0.0),
                );
                trail.2.update(
                    dt as f32,
                    wingtip_positions.1,
                    f32::max(f32::min(right_slip, 1.0), 0.0),
                );
            }
        }

        {
            // Rendering
            self.check_resize();
            self.gl.clear(
                WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
            );

            let world_to_camera = self
                .camera
                .get_camera_matrix();
            let camera_to_clipspace = [
                1.0,
                0.0,
                0.0,
                0.0,
                (self.canvas_resolution.1 as f32 / self.canvas_resolution.0 as f32),
                0.0,
                0.0,
                0.0,
                1.0,
            ];

            self.ship_sprite.world_to_camera = world_to_camera;
            self.ship_sprite.camera_to_clipspace = camera_to_clipspace;
            self.ship_sprite.setup(&self.gl);
            for ship in &self.ship_entities {
                self.ship_sprite.render(&self.gl, &ship);
            }

            let map_sprite_transform = Transform2d::new(0.0, 0.0, 0.0, 1.0);

            // Render the map
            self.map_sprite.world_to_camera = world_to_camera;
            self.map_sprite.camera_to_clipspace = camera_to_clipspace;
            self.map_sprite.world_to_sprite = map_sprite_transform.to_mat3_array();
            self.map_sprite.render(&self.gl);

            // Render the trails
            self.engine_trail_sprite.world_to_camera = world_to_camera;
            self.engine_trail_sprite.camera_to_clipspace = camera_to_clipspace;
            self.engine_trail_sprite.world_to_sprite = map_sprite_transform.to_mat3_array();
            self.engine_trail_sprite.setup(&self.gl);
            for engine_trail in &self.engine_trails {
                self.engine_trail_sprite.render(&self.gl, &engine_trail.0);
                self.engine_trail_sprite.render(&self.gl, &engine_trail.1);
                self.engine_trail_sprite.render(&self.gl, &engine_trail.2);
            }
        }
    }

    pub fn mouse_event(&mut self, event: MouseEvent) {
        log(&format!("Mouse Event {:?}", event))
    }

    pub fn keydown_event(&mut self, event: KeyboardEvent) {
        if !event.repeat() {
            self.key_map
                .set_state_from_str(&event.code(), KeyState::JustPressed);
        }
    }

    pub fn keyup_event(&mut self, event: KeyboardEvent) {
        self.key_map
            .set_state_from_str(&event.code(), KeyState::JustReleased);
    }
}

fn get_gl_context(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext, JsValue> {
    Ok(canvas.get_context("webgl2")?.unwrap().dyn_into()?)
}
