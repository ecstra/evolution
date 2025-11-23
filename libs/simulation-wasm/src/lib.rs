use wasm_bindgen::prelude::*;
use rand::prelude::*;
use lib_simulation as sim;

// ----------------------- Definitions ---------------------------
#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct World {
    #[wasm_bindgen(getter_with_clone)]
    pub agents: Vec<Agent>
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Agent {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}
// ---------------------------------------------------------------


// ------------------- World Implementation  ---------------------
impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let agents = world.agents().iter().map(Agent::from).collect();

        Self { agents }
    }
}
// ---------------------------------------------------------------


// ------------------ Agent Implementation  --------------------
impl From<&sim::Agent> for Agent {
    fn from(agent: &sim::Agent) -> Self {

        let pos = agent.position();
        let vel = agent.velocity();

        // THE MATH: Calculate angle from velocity vector.
        // atan2(y, x) returns the angle in radians
        let rotation = vel.y.atan2(vel.x);

        Self {
            x: pos.x,
            y: pos.y,
            rotation: rotation,
        }
    }
}
// ---------------------------------------------------------------


// ---------------- Simulation Implementation  -------------------
#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let sim = sim::Simulation::random(&mut rng);

        Self { rng, sim }
    }

    pub fn world(&self) -> World {
        World::from(self.sim.world())
    }

    pub fn step(&mut self) {
        self.sim.step();
    }
}
// ---------------------------------------------------------------
