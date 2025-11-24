use crate::*;

// ----------------------- Definitions ---------------------------
#[derive(Debug)]
pub struct Agent {
    pub(crate) position: na::Point2<f32>,
    pub(crate) speed: f32,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) eye: Eye,
    pub(crate) brain: nn::Network,
}
// ---------------------------------------------------------------


// ------------------- Agent Implementation  ---------------------
impl Agent {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        // Generate random coordinates between 0.0 and 1.0
        let position = rng.random();

        let speed = 0.0002;
        let rotation = rng.random();

        // Generate random velocity components between -0.5 and 0.5
        // so they can move up/down/left/right.

        let eye = Eye::default();

        let brain = nn::Network::random(
            rng,
            &[
                // Input Layer (Eyes are inputs)
                nn::LayerTopology {
                    neurons: eye.cells(),
                },

                // Hidden layer
                nn::LayerTopology {
                    neurons: 2 * eye.cells(),
                },
                
                // Output layer: [0] = Thrust force, [1] = Steering force
                nn::LayerTopology { neurons: 2 }
            ],
        );

        Self { position, speed, rotation, eye, brain }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}
// ---------------------------------------------------------------