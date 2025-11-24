use nalgebra as na;
use rand::{
    Rng, 
    RngCore
};

// ----------------------- Definitions ---------------------------
#[derive(Debug)]
pub struct Input {
    pub(crate) position: na::Point2<f32>
}
// ---------------------------------------------------------------


// ------------------- Input Implementation  ---------------------
impl Input {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.random(),
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}
// ---------------------------------------------------------------
