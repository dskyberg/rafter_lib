//! This utility library is partly in support of the rafters project and partly in support
//! of simply learning programmatic trigonometry.  The `dead_code` methods are just for trig fun.
use pythagoras::{ra_bc, rb_ac};

pub const PITCH_RUN: f32 = 12.0;

/// Calculate the angle is for a given pitch.
/// Given pitch is just the rise, when the run is 12, calculate the angle as
/// `tan(θ) = pitch/12`
#[inline(always)]
pub fn pitch_to_angle(pitch: u32) -> f32 {
    pitch_to_radians(pitch).to_degrees()
}

#[inline(always)]
pub fn pitch_to_radians(pitch: u32) -> f32 {
    (pitch as f32 / PITCH_RUN).atan()
}

#[inline(always)]
pub fn radians_to_pitch(radians: f32) -> u32 {
    (radians.tan() * PITCH_RUN) as u32
}

/// Calculate the pitch for a given angle in degrees.
/// Given angle is the angle of the triangle.  The run is always 12.
/// Calculate the pitch as `pitch = tan(θ) * 12`
#[allow(dead_code)]
#[inline(always)]
pub fn angle_to_pitch(angle: f32) -> u32 {
    radians_to_pitch(angle.to_radians())
}

/// Given an angle and a side (opposite or adjacent), calculate the other side and hypotenuse
pub fn toa(pitch: u32, opposite: Option<f32>, adjacent: Option<f32>) -> (f32, f32) {
    let rad = (pitch as f32 / 12.0).atan();
    match (opposite, adjacent) {
        (Some(a), None) => ra_bc(&rad, &a),
        (None, Some(b)) => rb_ac(&rad, &b),
        _ => panic!("Either opposite or adjacent must be provided"),
    }
}
