use kambo_graph::graphs::simple::UndirectedGraph;
use rand::prelude::*;

use super::chromosome::Chromosome;

/// Trait defining crossover operations
pub trait Crossover {
    /// Performs crossover between two parent chromosomes
    fn crossover(
        &self,
        parent1: &Chromosome,
        parent2: &Chromosome,
        graph: &UndirectedGraph<u32>,
    ) -> (Chromosome, Chromosome);
}

/// Single-point crossover implementation optimized for performance#[derive(Clone)]
pub struct SinglePoint {
    crossover_rate: f64,
}

impl SinglePoint {
    /// Creates a new instance with a specified crossover rate.
    ///
    /// The crossover rate determines the probability of applying the crossover
    /// operation to a pair of parents during the evolutionary process. The value
    /// must be within the range `[0.0, 1.0]`, where:
    /// - `0.0` means crossover is never applied.
    /// - `1.0` means crossover is always applied.
    ///
    /// # Parameters
    /// - `crossover_rate: f64`  
    ///   A floating-point value representing the probability of crossover.  
    ///   The value must satisfy `0.0 <= crossover_rate <= 1.0`.
    ///
    /// # Returns
    /// A new instance of `Self` with the specified crossover rate.
    ///
    /// # Panics
    /// This method will panic if:
    /// - `crossover_rate` is outside the range `[0.0, 1.0]`.  
    ///   The panic message will be:
    ///   ```text
    ///   Crossover probability must be between 0 and 1
    ///   ```
    ///
    #[inline]
    #[must_use]
    pub fn new(crossover_rate: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&crossover_rate),
            "Crossover probability must be between 0 and 1"
        );
        Self { crossover_rate }
    }
}

impl Crossover for SinglePoint {
    #[inline]
    fn crossover(
        &self,
        parent1: &Chromosome,
        parent2: &Chromosome,
        graph: &UndirectedGraph<u32>,
    ) -> (Chromosome, Chromosome) {
        let mut rng = thread_rng();

        // Se não ocorrer crossover, retorna cópias dos pais
        if !rng.gen_bool(self.crossover_rate) {
            return (
                Chromosome::new(parent1.genes().to_vec()),
                Chromosome::new(parent2.genes().to_vec()),
            );
        }

        let genes1 = parent1.genes();
        let genes2 = parent2.genes();
        let len = genes1.len();
        let point = rng.gen_range(1..len);

        let mut child1_genes = Vec::with_capacity(len);
        let mut child2_genes = Vec::with_capacity(len);

        unsafe {
            child1_genes.set_len(len);
            child2_genes.set_len(len);

            // First child
            std::ptr::copy_nonoverlapping(genes1.as_ptr(), child1_genes.as_mut_ptr(), point);
            std::ptr::copy_nonoverlapping(
                genes2.as_ptr().add(point),
                child1_genes.as_mut_ptr().add(point),
                len - point,
            );

            // Second child
            std::ptr::copy_nonoverlapping(genes2.as_ptr(), child2_genes.as_mut_ptr(), point);
            std::ptr::copy_nonoverlapping(
                genes1.as_ptr().add(point),
                child2_genes.as_mut_ptr().add(point),
                len - point,
            );
        }

        let mut child1 = Chromosome::new(child1_genes);
        let mut child2 = Chromosome::new(child2_genes);

        child1.fix(graph);
        child2.fix(graph);

        (child1, child2)
    }
}
