use petgraph::graph::UnGraph;

use super::{heuristics::Heuristic, Chromosome};

/// Represents a population of chromosomes for evolutionary algorithms.
///
/// The population is responsible for maintaining a collection of chromosomes,
/// managing their initialization through heuristics, and calculating aggregate
/// metrics such as average fitness.
///
/// # Fields
/// - `chromosomes: Vec<Chromosome>`: A vector containing the chromosomes in the population.
/// - `size: usize`: The maximum size of the population.
#[derive(Clone)]
pub struct Population {
    chromosomes: Vec<Chromosome>,
    size: usize,
}

impl Population {
    /// Creates a new population of chromosomes using the provided heuristics and graph.
    ///
    /// This function generates chromosomes by applying the heuristics in sequence. If the
    /// population size exceeds the number of heuristics, the last heuristic is used to
    /// generate the remaining chromosomes.
    ///
    /// # Parameters
    /// - `size: usize`: The number of chromosomes to generate for the population.
    /// - `heuristics: Vec<Heuristic>`:
    ///   A vector of heuristic functions used to generate chromosomes.
    ///   Each heuristic is a function of the form `fn(&UnGraph<u32, ()>) -> Chromosome`.
    /// - `graph: &UnGraph<u32, ()>`:
    ///   An undirected graph that represents the problem structure.
    ///
    /// # Panics
    /// - If the `heuristics` vector is empty.
    ///   - Panic message: `"At least one heuristic must be provided."`
    ///
    /// # Returns
    /// - A new instance of `Population` with chromosomes generated by the heuristics.
    ///
    /// # Notes
    /// - Chromosomes are adjusted using their `fix` method to ensure they satisfy
    ///   problem-specific constraints.
    #[inline]
    #[must_use]
    pub fn new(size: usize, heuristics: &[Heuristic], graph: &UnGraph<u32, ()>) -> Self {
        assert!(
            !heuristics.is_empty(),
            "At least one heuristic must be provided."
        );
        let mut chromosomes = Vec::with_capacity(size);

        for heuristic in heuristics {
            if chromosomes.len() < size {
                let chromosome = heuristic(graph);
                chromosomes.push(chromosome);
            }
        }

        let last_heuristic = *heuristics.last().unwrap();
        while chromosomes.len() < size {
            let mut chromosome = last_heuristic(graph);
            chromosome.fix(graph); // Adjust the chromosome if needed.
            chromosomes.push(chromosome);
        }

        chromosomes.sort_unstable_by_key(super::chromosome::Chromosome::fitness);
        Self { chromosomes, size }
    }

    /// Returns a reference to the chromosome with the best fitness (lowest value).
    #[inline]
    #[must_use]
    pub fn best_chromosome(&self) -> Option<&Chromosome> {
        self.chromosomes.first()
    }

    /// Returns a reference to the chromosome with the worst fitness (highest value).
    #[inline]
    #[must_use]
    pub fn worst_chromosome(&self) -> Option<&Chromosome> {
        self.chromosomes.last()
    }

    /// Returns a reference to the chromosomes in the population.
    ///
    /// # Returns
    /// - A slice of `Chromosome` representing the chromosomes in the population.
    #[inline]
    #[must_use]
    pub fn chromosomes(&self) -> &[Chromosome] {
        &self.chromosomes
    }

    /// Returns the size of the population.
    ///
    /// # Returns
    /// - A `usize` value representing the number of chromosomes in the population.
    #[inline]
    #[must_use]
    pub fn size(&self) -> usize {
        self.size
    }

    /// Adds a new chromosome to the population, maintaining sorted order.
    /// If the population is at capacity, replaces the worst chromosome if the new one is better.
    ///
    /// # Parameters
    /// - `chromosome`: New chromosome to add
    ///
    /// # Returns
    /// true if the chromosome was added, false if it was rejected
    pub fn add_chromosome(&mut self, chromosome: Chromosome) -> bool {
        let new_fitness = chromosome.fitness();

        // If population is not at capacity, insert in sorted position
        if self.chromosomes.len() < self.size {
            let pos = self
                .chromosomes
                .binary_search_by_key(&new_fitness, super::chromosome::Chromosome::fitness)
                .unwrap_or_else(|e| e);
            self.chromosomes.insert(pos, chromosome);
            return true;
        }

        // Otherwise, only replace if better than worst
        if let Some(worst) = self.worst_chromosome() {
            if new_fitness < worst.fitness() {
                self.chromosomes.pop();
                let pos = self
                    .chromosomes
                    .binary_search_by_key(&new_fitness, super::chromosome::Chromosome::fitness)
                    .unwrap_or_else(|e| e);
                self.chromosomes.insert(pos, chromosome);
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::genetic::*;

    use super::*;

    fn create_test_graph() -> UnGraph<u32, ()> {
        let mut graph = UnGraph::new_undirected();
        let v0 = graph.add_node(0);
        let v1 = graph.add_node(1);
        let v2 = graph.add_node(2);
        let v3 = graph.add_node(3);

        graph.add_edge(v0, v1, ());
        graph.add_edge(v1, v2, ());
        graph.add_edge(v2, v3, ());
        graph.add_edge(v3, v0, ());

        graph
    }

    #[test]
    fn test_population_creation() {
        let graph = create_test_graph();
        let heuristics = vec![h1, h2, h3, h4];
        let pop = Population::new(10, &heuristics, &graph);
        assert_eq!(pop.size(), 10);
        assert_eq!(pop.chromosomes().len(), 10);
    }

    #[test]
    fn test_population_sorting() {
        let graph = create_test_graph();
        let heuristics = vec![h1, h2, h3, h4];
        let pop = Population::new(10, &heuristics, &graph);

        // Verify population is sorted by fitness
        let fitnesses: Vec<u32> = pop
            .chromosomes()
            .iter()
            .map(super::super::chromosome::Chromosome::fitness)
            .collect();

        let mut sorted_fitnesses = fitnesses.clone();
        sorted_fitnesses.sort_unstable();

        assert_eq!(fitnesses, sorted_fitnesses);
    }

    #[test]
    fn test_add_chromosome() {
        let graph = create_test_graph();
        let heuristics = vec![h1, h2, h3, h4];
        let mut pop = Population::new(3, &heuristics, &graph);

        // Add a chromosome with very low fitness
        let low_fitness_chromosome = Chromosome::new(vec![0, 0, 0, 0]);
        let added = pop.add_chromosome(low_fitness_chromosome);

        assert!(added);
        assert_eq!(pop.best_chromosome().unwrap().fitness(), 0);
    }
}
