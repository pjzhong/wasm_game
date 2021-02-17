use super::transform::{length, Transform2d, Vec2};

const PREDICT_FACTOR: f32 = 0.6;
const ZOOM_FACTOR: f32 = 0.825;
const SMOOTHING: f32 = 0.4;
/// What the zoom is set to after calling "reset"
const RESET_ZOOM: f32 = 10.0;

pub struct Camera {
    position: Vec2,
    zoom: f32,
    pub target_posiion: Vec2,
    pub target_velocity: Vec2,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: (0.0, 0.0),
            zoom: RESET_ZOOM,
            target_posiion: (0.0, 0.0),
            target_velocity: (0.0, 0.0),
        }
    }

    pub fn reset(&mut self) {
        self.position = (0.0, 0.0);
        self.zoom = 10.0;
        self.target_posiion = (0.0, 0.0);
        self.target_velocity = (0.0, 0.0);
    }

    pub fn update(&mut self, dt: f32) {
        let ideal_position = (
            self.target_posiion.0 + self.target_velocity.0 * PREDICT_FACTOR,
            self.target_posiion.1 + self.target_velocity.1 * PREDICT_FACTOR,
        );

        let velocity = length(&self.target_velocity);
        let ideal_zoom = 1.0 + velocity * ZOOM_FACTOR;

        let zoom_err = self.zoom - ideal_zoom;
        let pos_err = (
            self.position.0 - ideal_position.0,
            self.position.1 - ideal_position.1,
        );

        self.zoom -= zoom_err * dt / SMOOTHING;

        self.position.0 -= pos_err.0 * dt / SMOOTHING;
        self.position.1 -= pos_err.1 * dt / SMOOTHING;
    }

    pub fn get_camera_matrix(&self) -> [f32; 9] {
        Transform2d::new(
            self.position.0,
            self.position.1,
            0.0,
           self.zoom,
        )
        .to_mat3_array()
    }
}
