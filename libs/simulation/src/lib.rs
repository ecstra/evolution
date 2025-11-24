use nalgebra as na;
use rand::{
    Rng, 
    RngCore
};

mod agent;
mod eye;
mod input;
mod world;

pub use crate::{
    agent::*, 
    eye::*, 
    input::*,
    world::*
};

// ----------------------- Definitions ---------------------------
#[derive(Debug)]
pub struct Simulation {
    world: World
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

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_movements();
    }

    // Movement of the agents
    // Just add their randomly generated velocity to themselves till they reach max velocity...
    // Velocity control can be done later...
    fn process_movements(&mut self) {
        for agent in &mut self.world.agents {
            agent.position += agent.velocity;

            agent.position.x = na::wrap(agent.position.x, 0.0, 1.0);
            agent.position.y = na::wrap(agent.position.y, 0.0, 1.0);
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for agent in &mut self.world.agents {
            for input in &mut self.world.inputs {
                let distance = na::distance(&agent.position, &input.position);
                
                // If they collide, don't remove but rather re-locate
                // This prevents consumption and regneration logic
                if distance <= 0.01 {
                    input.position = rng.random();
                }
            }
        }
    }
}
// ---------------------------------------------------------------