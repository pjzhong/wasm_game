use super::transform::Vec2;
use std::collections::VecDeque;

const NUM_SEGEMENTS: usize = 20;
const TIME_PER_SEGMENT: f32 = 0.25;

pub struct PathPoint {
    pub position: Vec2,
    pub tangent: Vec2,
    pub intensity: f32,
    pub width: f32,
    pub brightness: f32,
}

pub struct EngineTrail {
    path: VecDeque<PathPoint>,
    pub color: (f32, f32, f32, f32),
    pub width: f32,
    pub brightness: f32,
    max_length: usize,
    time_since_emit: f32,
    prev_position: Vec2,
}

impl EngineTrail {
    pub fn new(color: (f32, f32, f32, f32), width: f32, brightness: f32) -> Self {
        Self {
            path: VecDeque::new(),
            color,
            max_length: NUM_SEGEMENTS,
            prev_position: (0.0, 0.0),
            time_since_emit: 0.0,
            width,
            brightness,
        }
    }

    pub fn update(&mut self, dt: f32, position: Vec2, intensity: f32) {
        self.time_since_emit += dt;

        if self.path.len() != self.max_length {
            self.path.clear();
            for _ in 0..self.max_length {
                self.path.push_back(PathPoint {
                    position: position,
                    tangent: (0.0, 0.0),
                    intensity: intensity,
                    brightness: 0.0,
                    width: 0.0,
                });
            }
            assert!(self.path.len() == self.max_length)
        }

        let current_tangent = (
            (self.prev_position.0 - position.0) / dt,
            (self.prev_position.1 - position.1) / dt,
        );
        self.prev_position.0 = position.0;
        self.prev_position.1 = position.1;

        if self.time_since_emit > TIME_PER_SEGMENT {
            self.path.rotate_right(1);
            self.time_since_emit = dt;
        }

        {
            let first = self.path.get_mut(0).expect("path invalid");
            first.position.0 = position.0;
            first.position.1 = position.1;
            first.tangent.0 = current_tangent.0 * self.time_since_emit;
            first.tangent.1 = current_tangent.1 * self.time_since_emit;
            first.intensity = intensity;
            first.brightness = self.brightness;
            first.width = self.width;
        }
    }

    pub fn length(&self) -> i32 {
        self.path.len() as i32
    }

    pub fn get_percent_offset(&self) -> f32 {
        (1.0 - self.time_since_emit / TIME_PER_SEGMENT) / ((self.max_length - 2) as f32)
    }

    pub fn path_data_buffers(&self) -> Vec<f32> {
        let mut point_buffer = vec![];

        for point in self.path.iter() {
            point_buffer.push(point.position.0);
            point_buffer.push(point.position.1);
            point_buffer.push(point.tangent.0);
            point_buffer.push(point.tangent.1);

            point_buffer.push(point.intensity);
            point_buffer.push(point.brightness);
            point_buffer.push(point.width);
            point_buffer.push(0.0);
        }

        point_buffer
    }
}
