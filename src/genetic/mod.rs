/// Chromosome-related functionalities.
pub mod chromosome;

/// Crossover
pub mod crossover;

/// Heuristics to generate initial population
pub mod heuristics;

///Selection strategy
pub mod selection;

///Population
pub mod population;

pub use chromosome::Chromosome;
pub use crossover::{Crossover, SinglePoint};
pub use heuristics::{h1, h2, h3, h4, h5, Heuristic};
pub use population::Population;
pub use selection::{KTournament, Selection};
