use nalgebra as na;
use rand::{
    Rng, 
    RngCore
};
use std::f32::consts::FRAC_PI_2;

use lib_neural_network as nn;
use lib_genetic_algorithm as ga;

mod agent;
mod eye;
mod input;
mod world;
mod brain;

pub use crate::{
    agent::*, 
    eye::*, 
    input::*,
    world::*,
    brain::*
};

// ------------------------- Constants ---------------------------
const SPEED_MIN: f32 = 0.0001;
const SPEED_MAX: f32 = 0.0005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;
const GENERATION_LENGTH: usize = 2500;
// ---------------------------------------------------------------


// ----------------------- Definitions ---------------------------
#[derive(Debug)]
pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<ga::RankSelection, ga::UniformCrossover, ga::GaussianMutation>,
    age: usize,
}
// ---------------------------------------------------------------


// ---------------- Simulation Implementation  -------------------
impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {

        let world = World::random(rng);

        let ga = ga::GeneticAlgorithm::new(
            ga::RankSelection,
            ga::UniformCrossover,
            ga::GaussianMutation::new(0.01, 0.3),
            // ---------------------- ^--^ -^-^
            // | Chosen with a bit of experimentation.
            // |
            // | Higher values can make the simulation more chaotic,
            // | which - a bit counterintuitively - might allow for
            // | it to discover *better* solutions; but the trade-off
            // | is that higher values might also cause current, good
            // | enough solutions to be discarded.
            // | Source: https://pwy.io/posts/learning-to-fly-pt4/#huggin-n-evolvin
            // ---
        );

        Self { world, ga, age: 0 }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_brains();
        self.process_collisions(rng);
        self.process_movements();

        self.age += 1;
        if self.age > GENERATION_LENGTH {
            self.evolve(rng);
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) {
        self.age = 0;

        // Step 1: Prepare agents to be sent into the genetic algorithm
        let current_population: Vec<_> = self
            .world
            .agents
            .iter()
            .map(AgentIndividual::from_agent)
            .collect();

        // Step 2: Evolve agents
        let evolved_population = self.ga.evolve(
            rng,
            &current_population,
        );

        // Step 3: Bring agents back from the genetic algorithm
        self.world.agents = evolved_population
            .into_iter()
            .map(|individual| individual.into_agent(rng))
            .collect();

        // // Step 4: Restart foods
        // //
        // // (this is not strictly necessary, but it allows to easily spot
        // // when the evolution happens - so it's more of a UI thing.)
        // for food in &mut self.world.inputs {
        //     food.position = rng.random();
        // }
    }

    // Movement of the agents
    // Just add their randomly generated velocity to themselves till they reach max velocity...
    // Velocity control can be done later...
    fn process_movements(&mut self) {
        for agent in &mut self.world.agents {
            // Move along X-axis (agent.speed, 0.0) instead of Y-axis (0.0, agent.speed)
            // This aligns "Movement" with "0 radians" defined by the Eye.
            agent.position += agent.rotation * na::Vector2::new(agent.speed, 0.0);

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
                    agent.satiation += 1; // Update agent
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

            let response = agent.brain.nn.propagate(vision);

            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            agent.speed = (agent.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
            agent.rotation = na::Rotation2::new(agent.rotation.angle() + rotation);
        }
    }
}
// ---------------------------------------------------------------