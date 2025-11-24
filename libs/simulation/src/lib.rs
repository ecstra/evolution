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
        let num_agents = 20;
        let num_inputs = 40;

        let agents = (0..num_agents)
            .map(|_| Agent::random(rng))
            .collect();

        let inputs = (0..num_inputs)
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