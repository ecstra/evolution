use rand::{
    Rng,
    RngCore,
    seq::IndexedRandom
};

use std::{
    ops::Index,
};

// ----------------------- Definitions ---------------------------
// Genetic Algorithm
pub struct GeneticAlgorithm<S, C, M> {
    selection_method: S,
    crossover_method: C,
    mutation_method: M
}


// Individual
#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}

pub trait Individual {
    fn create(chromosome: Chromosome) -> Self;
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}


// Selection Method
pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}
pub struct RankSelection;


// Crossover Method
pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}
pub struct UniformCrossover;


// Mutation Method
pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}
pub struct GaussianMutation {
    chance: f32,
    coeff: f32,
}
// ---------------------------------------------------------------


// --------------- Rank Selection Impementation ------------------
impl SelectionMethod for RankSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        // Sort population by fitness to determine rank
        let mut sorted_population: Vec<&I> = population.iter().collect();
        sorted_population.sort_by(|a, b| {
            a.fitness()
                .partial_cmp(&b.fitness())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Zip the sorted individuals with their rank (weight)
        // Rank starts at 1 for the lowest fitness
        let ranked_population: Vec<(&I, usize)> = sorted_population
            .into_iter()
            .enumerate()
            .map(|(index, individual)| (individual, index + 1))
            .collect();

        // Apply roulette selection on the RANKS (weights)
        let selected = ranked_population
            .choose_weighted(rng, |(_, rank)| *rank)
            .expect("got an empty population");

        // Return the selected individual
        selected.0
    }
}
// ---------------------------------------------------------------


// ------------------ Crossover Impementation --------------------
impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());

        parent_a
            .iter()
            .zip(parent_b.iter())
            .map(|(&a, &b)| if rng.random_bool(0.5) { a } else { b })
            .collect()
    }
}
// ---------------------------------------------------------------


// ------------------- Mutation Impementation --------------------
impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance >= 0.0 && chance <= 1.0);
        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            let sign = if rng.random_bool(0.5) { -1.0 } else { 1.0 };

            if rng.random_bool(self.chance as f64) {
                *gene += sign * self.coeff * rng.random::<f32>();
            }
        }
    }
}
// ---------------------------------------------------------------


// ----------------- Chromosome Impementation --------------------
// Just make chromosomes a vector 👍
// Enable indexing for chromosomes
impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

// Enable iterating Chromosomes
impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

// Implement the iterator trait for Chromosome
impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}
// ---------------------------------------------------------------


// --------------- Genetic Algorithm Implementation --------------
impl<S, C, M> GeneticAlgorithm<S, C, M>
where
    S: SelectionMethod,
    C: CrossoverMethod,
    M: MutationMethod
{
    
    pub fn new(selection_method: S, crossover_method: C, mutation_method: M) -> Self {
        Self { 
            selection_method,
            crossover_method,
            mutation_method
        }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        // if population is empty, who you gonna mutate?
        assert!(!population.is_empty());

        (0..population.len())
        .map(|_| {
            // 1. Selection
            let parent_a = self.selection_method.select(rng, population).chromosome();
            let parent_b = self.selection_method.select(rng, population).chromosome();

            // 2. Crossover
            let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);

            // TODO mutation
            self.mutation_method.mutate(rng, &mut child);

            I::create(child)
        })
        .collect()
    }
}
// ---------------------------------------------------------------


// --------------------------- Tests -----------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use rand::{
        SeedableRng,
        rngs::StdRng
    };
    use std::collections::BTreeMap;
    use std::iter::FromIterator;
    
    #[derive(Clone, Debug, PartialEq)]
    enum TestIndividual {
        WithChromosome { chromosome: Chromosome },
        WithFitness { fitness: f32 },
    }

    impl TestIndividual {
        fn new(fitness: f32) -> Self {
            Self::WithFitness { fitness }
        }
    }

    impl PartialEq for Chromosome {
        fn eq(&self, other: &Self) -> bool {
            approx::relative_eq!(self.genes.as_slice(), other.genes.as_slice())
        }
    }

    impl Individual for TestIndividual {
        fn create(chromosome: Chromosome) -> Self {
            Self::WithChromosome { chromosome }
        }

        fn chromosome(&self) -> &Chromosome {
            match self {
                Self::WithChromosome { chromosome } => chromosome,

                Self::WithFitness { .. } => {
                    panic!("not supported for TestIndividual::WithFitness")
                }
            }
        }

        fn fitness(&self) -> f32 {
            match self {
                Self::WithChromosome { chromosome } => {
                    chromosome.iter().sum()
                }

                Self::WithFitness { fitness } => *fitness,
            }
        }
    }

    #[test]
    fn genetic_algorithm() {
        fn individual(genes: &[f32]) -> TestIndividual {
            TestIndividual::create(genes.iter().cloned().collect())
        }

        let mut rng = StdRng::seed_from_u64(42);

        let ga = GeneticAlgorithm::new(
            RankSelection,
            UniformCrossover,
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]),
            individual(&[1.0, 1.0, 1.0]),
            individual(&[1.0, 2.0, 1.0]),
            individual(&[1.0, 2.0, 4.0]),
        ];


        for _ in 0..10 {
            population = ga.evolve(&mut rng, &population);
        }

        let expected_population = vec![
            individual(&[1.9093989, 2.5592773, 4.7587504]),
            individual(&[1.7548623, 2.5357468, 4.528109]),
            individual(&[1.498456, 1.7593423, 4.110021]),
            individual(&[2.0690544, 2.5357468, 4.528109]),
        ];

        assert_eq!(population, expected_population);
    }

    #[test]
    fn ranked_selection() {
        let mut rng = StdRng::seed_from_u64(42);

        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        let mut actual_histogram = BTreeMap::new();

        // Run the fitness selection 50 times
        for _ in 0..50 {
            let fitness = RankSelection
                .select(&mut rng, &population)
                .fitness() as i32;

            *actual_histogram
                .entry(fitness)
                .or_insert(0) += 1;
        }

        let expected_histogram = BTreeMap::from_iter([
            (1, 6),
            (2, 8),
            (3, 17),
            (4, 19),
        ]);
        
        // Even though its rng, it chooses the highest fitness in population every time
        // So its not truly rng and hence the histograms can match...
        assert_eq!(actual_histogram, expected_histogram);
    }

    #[test]
    fn uniform_crossover() {
        let mut rng = StdRng::seed_from_u64(42);

        let parent_a: Chromosome = (1..=100).map(|n| n as f32).collect();
        let parent_b: Chromosome = (1..=100).map(|n| -n as f32).collect();

        let child = UniformCrossover.crossover(&mut rng, &parent_a, &parent_b);

        // Number of genes different between `child` and `parent_a`
        let diff_a = child.iter().zip(parent_a).filter(|(c, p)| *c != p).count();

        // Number of genes different between `child` and `parent_b`
        let diff_b = child.iter().zip(parent_b).filter(|(c, p)| *c != p).count();

        assert_eq!(diff_a, 48);
        assert_eq!(diff_b, 52);
    }

    mod gaussian_mutation {
        use super::*;

        fn actual(chance: f32, coeff: f32) -> Vec<f32> {
            let mut rng = StdRng::seed_from_u64(42);
            let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();

            GaussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);

            child.into_iter().collect()
        }
        
        mod given_zero_chance {
            use approx::assert_relative_eq;

            fn actual(coeff: f32) -> Vec<f32> {
                super::actual(0.0, coeff)
            }

            mod and_zero_coefficient {
                use super::*;

                #[test]
                fn does_not_change_the_original_chromosome() {
                    let actual = actual(0.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }

            mod and_nonzero_coefficient {
                use super::*;

                #[test]
                fn does_not_change_the_original_chromosome() {
                    let actual = actual(0.5);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
        }

        mod given_fifty_fifty_chance {
            use approx::assert_relative_eq;

            fn actual(coeff: f32) -> Vec<f32> {
                super::actual(0.5, coeff)
            }

            mod and_zero_coefficient {
                use super::*;

                #[test]
                fn does_not_change_the_original_chromosome() {
                    let actual = actual(0.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }

            mod and_nonzero_coefficient {
                use super::*;

                #[test]
                fn slightly_changes_the_original_chromosome() {
                    let actual = actual(0.5);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    assert_ne!(actual, expected);
                }
            }
        }

        mod given_max_chance {
            use approx::assert_relative_eq;

            fn actual(coeff: f32) -> Vec<f32> {
                super::actual(1.0, coeff)
            }

            mod and_zero_coefficient {
                use super::*;

                #[test]
                fn does_not_change_the_original_chromosome() {
                    let actual = actual(0.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }

            mod and_nonzero_coefficient {
                use super::*;

                #[test]
                fn entirely_changes_the_original_chromosome() {
                    let actual = actual(0.5);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    for (a, e) in actual.iter().zip(expected.iter()) {
                        assert_ne!(a, e);
                    }
                }
            }
        }
    }

}
// ---------------------------------------------------------------