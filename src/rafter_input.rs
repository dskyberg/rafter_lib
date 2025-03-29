#[derive(Debug)]
pub struct RafterInput {
    /// The rise in the rise over run calculation. Such as the 4 in 4:12
    pub pitch: u32,

    /// Distance in inches between the outside edges of the opposing walls that will hold the rafters.
    /// Remember to include the thickness of sheathing.
    pub span: f32,

    /// Width of the wall (top plate width + sheathing thickness) in inches, such as 5.5 for a 2x6 plate.
    /// This is needed to properly calculate the bird's mouth
    pub wall_width: f32,

    /// Thickness of the ridge board or beam in inches. Such as 1.5 for a typical 2x8 ridge board.
    pub beam_thickness: f32,
    pub beam_width: f32,

    /// Distance from the tip of the rafter to the outside edge of the wall in inches.
    pub overhang: f32,

    /// Width of the rafter in inches, such as 9.25 for a 2x10 rafter
    pub rafter_width: f32,
}
