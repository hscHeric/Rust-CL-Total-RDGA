//! # Genetic Algorithm Module
//!
//! This module provides the core components for implementing a genetic algorithm,
//! including structures and methods for representing chromosomes, managing populations,
//! applying heuristics, and performing selection and crossover operations.
//!
//! ## Key Components
//!
//! ### Chromosome
//! - Represents an individual solution in the genetic algorithm.
//! - Contains methods for:
//!   - Initialization (`Chromosome::new`).
//!   - Evaluation of fitness (`Chromosome::fitness`).
//!   - Fixing genes to ensure validity (`Chromosome::fix`).
//!
//! ### Heuristics
//! - Functions that generate initial chromosomes based on specific rules or algorithms.
//! - Examples include:
//!   - `h1`: A randomized heuristic.
//!   - `h2`: Degree-based heuristic.
//!   - `h3`: Neighbor-degree ordering heuristic.
//!   - `h4`: Heuristic focusing on isolated vertices.
//!   - `h5`: Baseline heuristic with uniform labeling.
//!
//! ### Population
//! - Represents a collection of chromosomes evolving over generations.
//! - Provides methods for managing the population, such as:
//!   - Initialization (`Population::new`).
//!   - Accessing the best and worst chromosomes (`Population::best_chromosome`, `Population::worst_chromosome`).
//!   - Adding new chromosomes (`Population::add_chromosome`).
//!
//! ### Selection
//! - Mechanisms for selecting chromosomes based on fitness.
//! - Examples include:
//!   - K-Tournament Selection.
//!   - Roulette Wheel Selection.
//!
//! ### Crossover
//! - Operations for combining two parent chromosomes to create offspring.
//! - Examples include:
//!   - Single-point crossover.
//!   - Two-point crossover.
//!
//! ## Public API
//!
//! ### Modules
//! - [`chromosome`]: Defines the `Chromosome` structure and related methods.
//! - [`heuristics`]: Provides heuristic functions for generating initial solutions.
//! - [`population`]: Implements the `Population` structure for managing chromosomes.
//! - [`selection`]: Contains selection strategies for genetic algorithms.
//! - [`crossover`]: Contains crossover strategies for genetic algorithms.
//!
//! ### Re-exports
//! - `Chromosome`: The main structure for genetic algorithm solutions.
//! - `Population`: The main structure for managing a group of chromosomes.
//! - `h1`, `h2`, `h3`, `h4`, `h5`: Heuristic functions for initializing chromosomes.
//!
//! ## Notes
//! - The module is designed to be extensible, allowing custom selection, crossover,
//!   and mutation strategies to be implemented as needed.
//! - Heuristic functions may produce non-deterministic results due to randomization or iteration
//!   over unordered collections.

/// Chromosome-related functionalities.
pub mod chromosome;

/// Heuristic functions for initializing chromosomes.
pub mod heuristics;

/// Population management for genetic algorithms.
pub mod population;

/// Crossover strategies
pub mod crossover;

/// Selection strategies for genetic algorithms.
pub mod selection;

pub use chromosome::Chromosome;
pub use heuristics::{h1, h2, h3, h4, h5};
pub use population::Population;
