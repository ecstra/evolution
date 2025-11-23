use rand::{
    RngCore,
    seq::IndexedRandom
};

use std::{
    ops::Index,
};
// ----------------------- Definitions ---------------------------
// Genetic Algorithm
pub struct GeneticAlgorithm<S> {
    selection_method: S,
}

// Selection Algorithm
pub struct RankSelection;

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
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
impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(selection_method: S) -> Self {
        Self { selection_method }
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

            // TODO crossover
            // TODO mutation
            todo!()
        })
        .collect()
    }
}
// ---------------------------------------------------------------


#[cfg(test)]
mod tests {
    use super::*;
    use rand::{
        SeedableRng,
        rngs::StdRng
    };
    use std::collections::BTreeMap;
    use std::iter::FromIterator;
    
    #[derive(Clone, Debug)]
    struct TestIndividual {
        fitness: f32,
    }

    impl TestIndividual {
        fn new(fitness: f32) -> Self {
            Self { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            self.fitness
        }

        fn chromosome(&self) -> &Chromosome {
            panic!("not supported for TestIndividual")
        }
    }

    #[test]
    fn roulette_wheel_selection() {
        let mut rng = StdRng::seed_from_u64(42);

        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        let mut actual_histogram = BTreeMap::new();

        //          vv This 50 is chosen at random.
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
}