use wasm_bindgen::prelude::wasm_bindgen;

use super::transform::PolarCoordinate;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

use super::transform::{length, normalize, Vec2};

pub struct Map {
    pub sin_consts: [f32; 8],
    pub cos_consts: [f32; 8],
    pub track_base_radius: f32,
    pub track_width: f32,
}

impl Map {
    pub fn track_radius(&self, angle: f32) -> f32 {
        let mut track_radius = self.track_base_radius;
        for i in 0..8 {
            let omega = (i + 1) as f32;
            track_radius += f32::sin(angle * omega) * self.sin_consts[i];
            track_radius += f32::cos(angle * omega) * self.cos_consts[i];
        }

        track_radius
    }

    pub fn distance_field(&self, position: Vec2) -> f32 {
        let course = length(position);
        let angle = position.0.atan2(position.1);

        let mut track_radius = self.track_base_radius;
        for i in 0..8 {
            let omega = (i + 1) as f32;
            track_radius += f32::sin(angle * omega) * self.sin_consts[i];
            track_radius += f32::cos(angle * omega) * self.cos_consts[i];
        }

        let mut track_sdf = course - track_radius;
        track_sdf = f32::abs(track_sdf) - self.track_width;

        track_sdf
    }

    pub fn calc_normal(&self, position: Vec2) -> Vec2 {
        const DELTA: f32 = 0.01;
        let here = self.distance_field(position);
        let above = self.distance_field((position.0, position.1 + DELTA));
        let right = self.distance_field((position.0 + DELTA, position.1));

        let dx = right - here;
        let dy = above - here;

        normalize((dx, dy))
    }

    pub fn get_start_position(&self) -> PolarCoordinate {
        const ANGLE: f32 = 0.0;
        PolarCoordinate {
            angle: ANGLE,
            radius: self.track_radius(ANGLE),
        }
    }

    pub fn get_track_direction(&self, angle: f32) -> f32 {
        const DELTA_ANGLE: f32 = 0.01;
        let radius_here = self.track_radius(angle);
        let radius_a_bit_further = self.track_radius(angle + DELTA_ANGLE);
        let delta_radius = radius_here - radius_a_bit_further;

        // Use cosine rule to find the length of the line joining the
        // two radius' (chord)
        let joining_side_length = cosine_rule(radius_here, radius_a_bit_further, DELTA_ANGLE);

        let ratio = radius_here / joining_side_length * f32::sin(DELTA_ANGLE);
        let ratio = f32::max(f32::min(ratio, 1.0), -1.0);
        let extra_angle = f32::asin(ratio);

        if delta_radius.is_sign_negative() {
            -angle - extra_angle
        } else {
            -angle + extra_angle + std::f32::consts::PI
        }
    }
}

pub fn cosine_rule(a: f32, b: f32, angle: f32) -> f32 {
    f32::sqrt(a * a + b * b - 2.0 * a * b * f32::cos(angle))
}
