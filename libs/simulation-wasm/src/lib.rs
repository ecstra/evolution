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
    fn from(Agent: &sim::Agent) -> Self {
        Self {
            x: Agent.position().x,
            y: Agent.position().y,
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
}
// ---------------------------------------------------------------
