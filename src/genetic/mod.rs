/// Module that contains the implementation of the `Chromosome` structure.
///
/// This module defines the `Chromosome` structure, which is used to represent
/// a chromosome in the context of genetic algorithms. The structure includes
/// methods for creation, evaluation (fitness), and correction (`fix`) based on an
/// undirected graph.
///
/// # Module Structure
/// - **`Chromosome`**: The main structure that stores the genes and provides associated methods.
/// - **Main Methods**:
///     - `new`: Creates a new chromosome.
///     - `fitness`: Calculates the fitness of the chromosome.
///     - `genes`: Returns the genes of the chromosome.
///     - `fix`: Fixes the genes of the chromosome based on a graph.
pub mod chromosome;

/// # Heuristic Functions for Graph Labeling
///
/// This module provides heuristic implementations for solving graph labeling problems.
/// Each heuristic function assigns a set of labels (genes) to the vertices of a graph based
/// on specific rules. The primary goal is to generate a valid chromosome that adheres to
/// the problem's labeling constraints.
///
/// ## Definitions
/// - **Heuristic:** A function that takes an `UndirectedGraph` as input and returns an `Chromosome`.
/// - **Chromosome:** Represents a potential solution in the form of a gene vector. Each gene corresponds to a label assigned to a vertex.
///
/// ## Public API
/// - [`Heuristic`]: Defines the type signature for heuristic functions.
/// - [`h1`]: Implements a randomized heuristic for graph labeling.
/// - [`h2`]: Implements a heuristic using the vertex with the highest degree.
/// - [`h3`]: An extension of `h2` that orders neighbors by degree.
/// - [`h4`]: A heuristic focusing on isolated vertices and their neighbors.
/// - [`h5`]: A baseline heuristic where all vertices are assigned the same label.
///
/// ## Note on Determinism
/// Heuristics involving randomization or iteration over collections (e.g., `HashSet` or
/// `HashMap`) may produce non-deterministic results due to the lack of guaranteed iteration order.
///
/// ## Usage
/// Heuristic functions are designed to operate on an [`UnGraph`] and return a labeled
/// [`Chromosome`]. The implementation of each heuristic ensures that the generated chromosome
/// is valid according to the labeling rules defined for the problem domain.
pub mod heuristics;

/// # Population Module
///
/// This module contains the implementation of the `Population` structure,
/// which represents a collection of chromosomes used in genetic algorithms.
/// It provides methods for creating, managing, and evaluating populations of
/// chromosomes.
///
/// ## Definitions
/// - **Population**: A group of chromosomes representing potential solutions
///   to a problem. The population evolves over generations in genetic algorithms.
/// - **Chromosome**: Represents an individual solution within the population.
///   Each chromosome has a fitness value that determines its quality.
///
/// ## Structure
/// - **`Population`**:
///   The main structure that stores chromosomes and provides methods for:
///   - Initialization (`new`)
///   - Accessing chromosomes (`chromosomes`)
///   - Calculating the average fitness of the population (`average_fitness`)
///   - Getting the population size (`size`)
///
/// ## Public API
/// - [`Population::new`]: Initializes a new population of chromosomes using heuristics.
/// - [`Population::chromosomes`]: Returns a slice of chromosomes in the population.
/// - [`Population::size`]: Returns the size of the population.
/// - [`Population::average_fitness`]: Calculates the average fitness of the population.
pub mod population;

pub use chromosome::Chromosome;
pub use heuristics::h1;
pub use heuristics::h2;
pub use heuristics::h3;
pub use heuristics::h4;
pub use heuristics::h5;
pub use population::Population;
