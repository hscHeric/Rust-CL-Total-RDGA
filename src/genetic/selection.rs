use rand::prelude::*;

use super::{Chromosome, Population};

/// Trait defining selection operations for the genetic algorithm.
pub trait Selection {
    /// Selects a chromosome from the population.
    ///
    /// # Arguments
    ///
    /// * `population` - A reference to the population from which to select.
    ///
    /// # Returns
    ///
    /// A reference to the selected chromosome.
    fn select<'a>(&self, population: &'a Population) -> &'a Chromosome;
}

/// K-Tournament selection implementation.
pub struct KTournament {
    k: usize,
}

impl KTournament {
    /// Creates a new instance of `KTournamentSelection`.
    ///
    /// # Arguments
    ///
    /// * `k` - The number of participants in each tournament.
    ///
    /// # Returns
    ///
    /// A new instance of `KTournamentSelection`.
    #[inline]
    #[must_use]
    pub fn new(k: usize) -> Self {
        Self { k }
    }
}

impl Selection for KTournament {
    /// Selects a chromosome from the population using K-Tournament selection.
    ///
    /// # Arguments
    ///
    /// * `population` - A reference to the population from which to select.
    ///
    /// # Returns
    ///
    /// A reference to the selected chromosome.

    fn select<'a>(&self, population: &'a Population) -> &'a Chromosome {
        let mut rng = thread_rng();
        let pop_size = population.size();

        let mut indices = Vec::with_capacity(self.k);
        for _ in 0..self.k {
            indices.push(rng.gen_range(0..pop_size));
        }

        let best_idx = indices
            .iter()
            .min_by_key(|&&idx| population.chromosomes()[idx].fitness())
            .copied()
            .unwrap_or(0);

        &population.chromosomes()[best_idx]
    }
}
