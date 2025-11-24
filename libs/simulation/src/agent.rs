use crate::*;

// ----------------------- Definitions ---------------------------
#[derive(Debug)]
pub struct Agent {
    pub(crate) position: na::Point2<f32>,
    pub(crate) speed: f32,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) eye: Eye,
    pub(crate) brain: Brain,
    pub(crate) satiation: usize
}

#[derive(Debug)]
pub struct AgentIndividual {
    fitness: f32,
    chromosome: ga::Chromosome,
}
// ---------------------------------------------------------------


// ------------------- Agent Implementation  ---------------------
impl Agent {
    fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.random(),
            rotation: rng.random(),
            speed: 0.0002,
            eye,
            brain,
            satiation: 0,
        }
    }

    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::random(rng, &eye);
        Self::new(eye, brain, rng)
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.brain.as_chromosome()
    }

    pub(crate) fn from_chromosome(
        chromosome: ga::Chromosome,
        rng: &mut dyn RngCore,
    ) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}
// ---------------------------------------------------------------


// -------------- AgentIndividual Implementation -----------------
impl ga::Individual for AgentIndividual {
    fn create(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}

impl AgentIndividual {
    pub fn from_agent(agent: &Agent) -> Self {
        Self {
            fitness: agent.satiation as f32,
            chromosome: agent.as_chromosome(),
        }
    }

    pub fn into_agent(self, rng: &mut dyn RngCore) -> Agent {
        Agent::from_chromosome(self.chromosome, rng)
    }
}
// ---------------------------------------------------------------
