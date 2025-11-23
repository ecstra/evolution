use std::error::Error;
use std::fmt;

// ------------------------- Error -------------------------------
#[derive(Debug)]
enum NNError {
    MismatchedNeuronInputSize  {
        got: usize,
        expected: usize
    }
}

// Displays for the Errors
impl fmt::Display for NNError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NNError::MismatchedNeuronInputSize{got, expected} => {
                write!(f, "Got {got} inputs, but {expected} inputs were expected.")
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
struct Layer {
    neurons: Vec<Neuron>,
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}
// ---------------------------------------------------------------


// ----------------- Network Implementation ----------------------
impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}
// ---------------------------------------------------------------


// ------------------- Layer Implementation ----------------------
impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs).unwrap())
            .collect()
    }
}
// ---------------------------------------------------------------


// ------------------ Neuron Implementation ----------------------
impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> Result<f32, NNError> {
        // Mismatched input size?
        if inputs.len() != self.weights.len() {
            return Err(
                NNError::MismatchedNeuronInputSize { 
                    got: inputs.len(), 
                    expected: self.weights.len()
            });
        }
        
        // Calculate output as : output = Σ(input * weight)
        let mut output = 0.0;
        for (&input, &weight) in inputs.iter().zip(&self.weights) {
            output += input * weight;
        }

        // Add the bias
        output += self.bias;

        // Return max of output, 0
        Ok(output.max(0.0))
    }
}
// ---------------------------------------------------------------