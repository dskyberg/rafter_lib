use serde::Serialize;

use crate::right_angle_like::RightAngleLike;
use crate::{RafterInput, pitch_to_angle, toa};

#[derive(Debug, Clone, Serialize)]
pub struct BirdsMouth {
    /// The rise of the triangle
    heel: f32,
    /// The run of the triangle
    seat: f32,
    /// The hypotenuse of the triangle
    length: f32,
    /// The angle is based on the rafter pitch
    angle: f32,
    /// Where, on the length of the rafter, does the bird's mouth start
    seat_start: f32,
}

impl BirdsMouth {
    pub fn from_input(input: &RafterInput) -> Self {
        Self::check_code(input)
    }

    /// Ensure that the birds mouth heel is no more than 25% of rafter height,
    /// and the seat is not greater than the top plate width.  Adjust the
    /// bird's mouth accordingly
    fn check_code(input: &RafterInput) -> Self {
        let max_heel = input.rafter_width / 3.0;
        let max_seat = input.wall_width;
        let rise: f32;
        let run: f32;
        let length: f32;

        let (tmp_rise, tmp_length) = toa(input.pitch, None, Some(max_seat));
        // Ideally, the heel is just the rise.  But the heel can't be greater
        // than 1/3 of the rafter width.  If it is, we need to recalc the seat.
        if tmp_rise <= max_heel {
            rise = tmp_rise;
            run = max_seat;
            length = tmp_length;
        } else {
            // calc the triangle with the overage as the rise (opposite).
            let (tmp_run, tmp_length) = toa(input.pitch, Some(max_heel), None);
            rise = max_heel;
            run = tmp_run;
            length = tmp_length;
        }

        // Calculate the distance to the start of the bird's mout heal
        // The run of this triangle is the overhang of the rafter.
        let (seat_start, _) = toa(input.pitch, None, Some(input.overhang));

        Self {
            heel: rise,
            seat: run,
            length,
            angle: pitch_to_angle(input.pitch),
            seat_start,
        }
    }
}
impl RightAngleLike<f32> for BirdsMouth {
    fn rise(&self) -> f32 {
        self.heel
    }
    fn run(&self) -> f32 {
        self.seat
    }
    fn length(&self) -> f32 {
        self.length
    }
    fn angle(&self) -> f32 {
        self.angle
    }
}
