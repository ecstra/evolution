use std::{
    fmt,
    error::Error,
};

use rand::{
    Rng,
    RngCore
};

// ------------------------- Error -------------------------------
#[derive(Debug)]
enum NNError {
    MismatchedNeuronInputSize  {
        recieved: usize,
        expected: usize
    }
}

// Displays for the Errors
impl fmt::Display for NNError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NNError::MismatchedNeuronInputSize{recieved, expected} => {
                write!(f, "Input Size Mismatch! \n\rExpected: {expected} inputs. \n\rRecieved: {recieved} inputs. \n\r{expected} != {recieved}")
            }
        }
    }
}

impl Error for NNError {}
// ---------------------------------------------------------------


// ----------------------- Definitions ---------------------------
#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

#[derive(Debug)]
pub struct Layer {
    neurons: Vec<Neuron>,
}

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}
// ---------------------------------------------------------------


// ----------------- Network Implementation ----------------------
impl Network {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        let layers: Vec<Layer> = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }

    pub fn weights(&self) -> Vec<f32> {
        let mut weights = Vec::new();

        for layer in &self.layers {
            for neuron in &layer.neurons {
                weights.push(neuron.bias);
                weights.extend(&neuron.weights);
            }
        }

        weights
    }

    pub fn from_weights(
        layers: &[LayerTopology],
        weights: impl IntoIterator<Item = f32>,
    ) -> Self {
        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|adjacent_layers| {
                let input_neurons = adjacent_layers[0].neurons;
                let output_neurons = adjacent_layers[1].neurons;

                let neurons = (0..output_neurons)
                    .map(|_| {
                        // We must pull exactly 1 bias + N weights
                        let bias = weights.next().expect("not enough weights");
                        
                        let neuron_weights = (0..input_neurons)
                            .map(|_| weights.next().expect("not enough weights"))
                            .collect();

                        Neuron {
                            bias,
                            weights: neuron_weights,
                        }
                    })
                    .collect();

                Layer { neurons }
            })
            .collect();

        if weights.next().is_some() {
            panic!("got too many weights/genes for this network topology");
        }

        Self { layers }
    }
}
// ---------------------------------------------------------------


// ------------------- Layer Implementation ----------------------
impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(rng, input_size))
            .collect();

        Self { neurons }
    }
}
// ---------------------------------------------------------------


// ------------------ Neuron Implementation ----------------------
impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        // Mismatched input size?
        // It's not recoverable, so just panic
        if inputs.len() != self.weights.len() {
            panic!("{}", NNError::MismatchedNeuronInputSize { 
                recieved: inputs.len(), 
                expected: self.weights.len()
            });
        }
        
        // Calculate output as : output = (Σ(input * weight)) + bias
        let mut output: f32 = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        
        // Add bias
        output += self.bias;

        // Return max of output, 0
        output.max(0.0)
    }

    fn random(rng: &mut dyn RngCore, input_size: usize) -> Self {
        let bias = rng.random_range(-1.0..=1.0);

        let weights = (0..input_size)
            .map(|_| rng.random_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }
}
// ---------------------------------------------------------------


// --------------------------- Tests -----------------------------
#[cfg(test)]
mod nn_tests {
    use super::*;
    use rand::{
        SeedableRng,
        rngs::StdRng
    };
    use approx::assert_relative_eq;

    #[test]
    fn random() {
        let mut rng = StdRng::seed_from_u64(42);
        let neuron = Neuron::random(&mut rng, 4);
        
        assert_relative_eq!(neuron.bias, -0.7331805);
        assert_relative_eq!(
            neuron.weights.as_slice(),
            [0.053114653, -0.5025234, 0.08545041, 0.7368531].as_ref()
        );
    }

    #[test]
    fn propagate() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };

        // Ensures our ReLU works
        // Math: [(Σ(input * weight)) + bias]
        //   (-0.3 * -10.0) + (0.8 * -10.0) + 0.5
        // = (3.0)          + (-8.0)        + 0.5 
        // = -4.5
        // max(-4.5, 0.0) = 0.0
        assert_relative_eq!(
            neuron.propagate(&[-10.0, -10.0]),
            0.0,
        );

        // Math: [(Σ(input * weight)) + bias]
        //   (-0.3 * 0.5) + (0.8 * 1.0) + 0.5
        // = (-0.15)      + (0.8)       + 0.5
        // = 1.15
        assert_relative_eq!(
            neuron.propagate(&[0.5, 1.0]),
            1.15,
        );
    }

    #[test]
    #[should_panic(expected="Input Size Mismatch! \n\rExpected: 2 inputs. \n\rRecieved: 3 inputs. \n\r2 != 3")]
    fn invalid_len() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };

        neuron.propagate(&[1.0, 2.0, 3.0]);
    }
}
// ---------------------------------------------------------------