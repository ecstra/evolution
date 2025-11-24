use crate::*;

#[derive(Debug)]
pub struct Brain {
    pub(crate) nn: nn::Network,
}

impl Brain {
    pub fn random(rng: &mut dyn RngCore, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::random(rng, &Self::topology(eye)),
        }
    }

    pub(crate) fn from_chromosome(
        chromosome: ga::Chromosome,
        eye: &Eye,
    ) -> Self {
        Self {
            nn: nn::Network::from_weights(
                &Self::topology(eye),
                chromosome,
            ),
        }
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.nn.weights().into_iter().collect()
    }

    fn topology(eye: &Eye) -> [nn::LayerTopology; 3] {
        [
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
        ]
    }
}