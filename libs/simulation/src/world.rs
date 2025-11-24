use crate::*;

// ----------------------- Definitions ---------------------------
#[derive(Debug)]
pub struct World {
    pub(crate) agents: Vec<Agent>,
    pub(crate) inputs: Vec<Input>
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