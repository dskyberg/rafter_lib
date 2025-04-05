use serde::{Deserialize, Serialize};

use crate::RafterInput;
use crate::beam::Beam;
use crate::birds_mouth::BirdsMouth;
use crate::right_angle_like::RightAngleLike;
use crate::tail::Tail;
use crate::utils::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rafter {
    pub width: f32,
    pub rise: f32,
    pub run: f32,
    pub length: f32,
    pub pitch: u32,
    pub angle: f32,
    pub beam: Beam,
    pub birds_mouth: BirdsMouth,
    pub tail: Tail,
    pub angled_width: f32,
    pub ridge_beam_height: f32,
    pub total_length: f32,
    pub total_height: f32,
}

impl RightAngleLike<f32> for Rafter {
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
        pitch_to_angle(self.pitch)
    }
}

/// The run of the rafter is calculated from the outside of the wall (including sheathing) to
/// the center of the structure, minuse 1/2 the thickness of the ridge board/beam.
///
/// The ridge beam height is then the rafter rise - the bird's mouth rise.
///
/// And finally, the total length is the tail length + the rafter length
impl Rafter {
    pub fn from_input(input: &RafterInput) -> Self {
        // First calculate the big triangle
        let run = (input.span / 2.0) - (input.beam_thickness / 2.0);
        let (rise, length) = toa(input.pitch, None, Some(run));
        let angle = pitch_to_angle(input.pitch);
        let beam = Beam::new(input.beam_thickness, input.beam_width);
        // The bird's mouth is pretty standard.  We just need to adjust for the
        // width of the rafter, and wall to ensure it meets code.
        let birds_mouth = BirdsMouth::from_input(input);
        let tail = Tail::from_pitch_and_run(input.pitch, input.overhang);

        // Calculate the angled rafter width, so that we can calculate the ridge beam height
        let (_, angled_width) = toa(input.pitch, None, Some(input.rafter_width));
        let ridge_beam_height = rise + angled_width - birds_mouth.rise();
        let total_length = tail.length() + length;
        let total_height = rise + angled_width - birds_mouth.rise();
        Rafter {
            width: input.rafter_width,
            rise,
            run,
            length,
            pitch: input.pitch,
            angle,
            beam,
            birds_mouth,
            tail,
            angled_width,
            ridge_beam_height,
            total_length,
            total_height,
        }
    }
}
