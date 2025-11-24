use nalgebra as na;
use rand::{
    Rng, 
    RngCore
};
use std::f32::consts::FRAC_PI_2;

use lib_neural_network as nn;

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

const SPEED_MIN: f32 = 0.0001;
const SPEED_MAX: f32 = 0.0005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;
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
        self.process_brains();
        self.process_collisions(rng);
        self.process_movements();
    }

    // Movement of the agents
    // Just add their randomly generated velocity to themselves till they reach max velocity...
    // Velocity control can be done later...
    fn process_movements(&mut self) {
        for agent in &mut self.world.agents {
            // Add velocity to the position
            agent.position += agent.rotation * na::Vector2::new(0.0, agent.speed);

            // Clamp the position to be within 0 and 1
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

    fn process_brains(&mut self) {
        for agent in &mut self.world.agents {
            let vision = agent.eye.process_vision(
                agent.position,
                agent.rotation,
                &self.world.inputs,
            );

            let response = agent.brain.propagate(vision);

            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            agent.speed = (agent.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
            agent.rotation = na::Rotation2::new(agent.rotation.angle() + rotation);
        }
    }
}
// ---------------------------------------------------------------