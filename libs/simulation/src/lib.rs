use nalgebra as na;
use rand::{
    Rng, 
    RngCore
};

// ----------------------- Definitions ---------------------------
#[derive(Debug)]
pub struct Agent {
    position: na::Point2<f32>,
    velocity: na::Vector2<f32>
}

#[derive(Debug)]
pub struct Input {
    position: na::Point2<f32>
}

#[derive(Debug)]
pub struct World {
    agents: Vec<Agent>,
    inputs: Vec<Input>
}

pub struct Simulation {
    world: World
}
// ---------------------------------------------------------------


// ------------------ Agent Implementation  --------------------
impl Agent {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.random(),
            velocity: na::Vector2::new(0.002, rng.random())
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn velocity(&self) -> na::Vector2<f32> {
        self.velocity
    }
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


// ------------------- World Implementation  ---------------------
impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let agents = (0..40)
            .map(|_| Agent::random(rng))
            .collect();

        let inputs = (0..60)
            .map(|_| Input::random(rng))
            .collect();

        Self { agents, inputs }
    }

    pub fn agents(&self) -> &[Agent] {
        &self.agents
    }
    
    pub fn inputs(&self) -> &[Input] {
        &self.inputs
    }
}
// ---------------------------------------------------------------


// ---------------- Simulation Implementation  -------------------
impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng)
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }
}
// ---------------------------------------------------------------


