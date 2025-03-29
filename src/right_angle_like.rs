#![allow(dead_code)]

pub trait RightAngleLike<T: Into<f32> + From<f32>> {
    fn rise(&self) -> T;
    fn run(&self) -> T;
    fn length(&self) -> T;
    fn angle(&self) -> T;

    #[inline(always)]
    fn asin(&self) -> T {
        (self.rise().into() / self.length().into()).asin().into()
    }

    #[inline(always)]
    fn acos(&self) -> T {
        (self.run().into() / self.length().into()).acos().into()
    }

    #[inline(always)]
    fn atan(&self) -> T {
        (self.rise().into() / self.run().into()).atan().into()
    }
}
