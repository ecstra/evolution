use nalgebra as na;
use rand::{
    Rng, 
    RngCore
};

// ----------------------- Definitions ---------------------------
#[derive(Debug)]
pub struct Agent {
    pub(crate) position: na::Point2<f32>,
    pub(crate) velocity: na::Vector2<f32>
}
// ---------------------------------------------------------------


// ------------------- Agent Implementation  ---------------------
impl Agent {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        // Generate random coordinates between 0.0 and 1.0
        let position = rng.random();

        // Generate random velocity components between -0.5 and 0.5
        // so they can move up/down/left/right.

        let speed_factor = 1e-3;
        let velocity = na::Vector2::new(
            rng.random_range(-0.5..0.5), 
            rng.random_range(-0.5..0.5)
        ) * speed_factor; // Scale down speed so they don't nyoom

        Self { position, velocity }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn velocity(&self) -> na::Vector2<f32> {
        self.velocity
    }
}
// ---------------------------------------------------------------