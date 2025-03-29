use serde::Serialize;

use crate::right_angle_like::RightAngleLike;
use crate::utils::*;

#[derive(Debug, Clone, Serialize)]
pub struct Tail {
    rise: f32,
    run: f32,
    length: f32,
    angle: f32,
}

impl Tail {
    pub fn from_pitch_and_run(pitch: u32, run: f32) -> Self {
        let angle = pitch_to_angle(pitch);
        let (rise, length) = toa(pitch, None, Some(run));
        Self {
            rise,
            run,
            length,
            angle,
        }
    }
}

impl RightAngleLike<f32> for Tail {
    fn rise(&self) -> f32 {
        self.rise
    }
    fn run(&self) -> f32 {
        self.run
    }
    fn length(&self) -> f32 {
        self.length
    }
    fn angle(&self) -> f32 {
        self.angle
    }
}
