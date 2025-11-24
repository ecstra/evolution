# Evolve

A biological evolution simulation built from scratch using  **Rust** ,  **WebAssembly** , and  **React** .

This project simulates a population of autonomous agents that evolve over generations to survive. Each agent possesses a unique brain (Neural Network) and genome (Chromosome). Through the process of natural selection, agents that are better at finding food pass their genes to the next generation, resulting in emergent intelligent behavior.

[**View Live Demo**](https://gilded-beignet-e84d24.netlify.app/ "null")

## Key Features

* **Custom Neural Network Engine** : A fully connected feed-forward neural network implemented in pure Rust without external ML frameworks.
* **Genetic Algorithm** : A modular evolutionary engine featuring Rank Selection, Uniform Crossover, and Gaussian Mutation.
* **High-Performance Simulation** : Core logic runs in Rust, compiled to WebAssembly (WASM) for near-native performance in the browser.
* **Sensory System** : Agents "see" their environment using a custom ray-casting FOV system (9 photoreceptors) that detects food proximity and angle.
* **Modern UI** : A responsive, glassmorphism dashboard built with React, TypeScript, and Recharts to visualize real-time evolutionary statistics.

## Technical Architecture

The project is structured as a monorepo with distinct libraries handling specific domains.

### 1. The Brain (`libs/neural-network`)

Instead of using crates like `tch-rs` or `tensorflow`, the neural network is hand-rolled to demonstrate understanding of the underlying math.

* **Architecture** : Multi-Layer Perceptron (MLP).
* **Topology** : Input Layer (9 Eye Cells) → Hidden Layer (18 Neurons) → Output Layer (Speed & Rotation).
* **Activation Function** : ReLU (Rectified Linear Unit).

### 2. The Genetics (`libs/genetic-algorithm`)

The evolutionary engine drives the improvement of the agents.

* **Selection** : `RankSelection` (Linear Ranking) - Ensures even the worst agents have a tiny chance to reproduce, maintaining genetic diversity.
* **Crossover** : `UniformCrossover` - Genes are mixed randomly from both parents.
* **Mutation** : `GaussianMutation` - Adds random noise to weights to discover new behaviors.

### 3. The Simulation (`libs/simulation`)

Handles the physics and world state.

* **Physics** : Vector-based movement using `nalgebra`.
* **Vision** : Agents have a field of view defined by `FOV_RANGE` and `FOV_ANGLE`. The vision system calculates the angle and distance to food sources, mapping them to neural inputs.
* **Cycle** : Sense -> Think -> Act -> Survive

1. **Sense** : Eye processes world data.
2. **Think** : Brain propagates inputs to outputs.
3. **Act** : Agent applies force (Speed/Rotation) based on brain output.
4. **Survive** : Agents gain fitness by consuming food.

## Project Structure

```
├── libs/
│   ├── genetic-algorithm/  # Generic GA implementation (Selection, Crossover, Mutation)
│   ├── neural-network/     # Generic Neural Network (Layers, Neurons, Propagation)
│   ├── simulation/         # Core world logic (Physics, Agents, Food)
│   └── simulation-wasm/    # WASM bindings to expose Rust structs to JS
├── simulation-ui/          # Frontend application (React, Vite, Tailwind)
└── Cargo.toml              # Workspace configuration
```

## Getting Started

### Prerequisites

* **Rust** (latest stable)
* **Node.js** & **npm**
* **wasm-pack** (`cargo install wasm-pack`)

### Installation

1. **Clone the repository**

   ```
   git clone https://github.com/ecstra/evolution.git
   cd evolve
   ```
2. **Build the WASM package**

   ```
   cd libs/simulation-wasm
   wasm-pack build
   ```
3. **Install Frontend Dependencies**

   ```
   cd simulation-ui
   npm install
   ```
4. **Run the Application**

   ```
   npm run dev
   ```

   Open `http://localhost:5173` to watch evolution in action!

## Evolutionary Metrics

The dashboard provides real-time insights into the genetic health of the population:

* **Min/Max Fitness** : Shows the range of performance in the current generation.
* **Average Fitness** : The primary metric for tracking evolutionary progress.
* **Stagnation Detection** : The simulation automatically detects when fitness converges and evolution plateaus.

## Credits & Inspiration

This project was inspired by and adopted concepts from the excellent [Learning to Fly](https://pwy.io/posts/learning-to-fly-pt1/ "null") series by Patryk Wychowaniec. It serves as a study in applying Rust to complex systems programming tasks.

*This project is for educational and showcase purposes.*
