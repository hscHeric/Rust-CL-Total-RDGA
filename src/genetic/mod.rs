//! # Módulo Genético
//!
//! Este módulo contém as implementações principais para algoritmos genéticos aplicados à
//! otimização de problemas em grafos.
//!
//! ## Componentes
//!
//! - **Chromosome**: Representa um cromossomo, incluindo sua estrutura de genes e funções auxiliares.
//! - **Crossover**: Implementa estratégias de cruzamento genético, como cruzamento de um ponto e dois pontos.
//! - **Population**: Gerencia populações de cromossomos, incluindo criação, validação e seleção dos melhores indivíduos.
//! - **Selection**: Contém estratégias de seleção, como torneio, para a escolha de indivíduos para a próxima geração.
//! - **Heuristics**: Módulo para heurísticas adicionais (pode ser estendido).

#[allow(missing_docs)]
pub mod chromosome;

#[allow(missing_docs)]
pub mod crossover;

#[allow(missing_docs)]
pub mod heuristics;

#[allow(missing_docs)]
pub mod population;

#[allow(missing_docs)]
pub mod selection;

pub use chromosome::Chromosome;
pub use crossover::{CrossoverStrategy, TwoPointCrossover};
pub use heuristics::h1;
pub use heuristics::h2;
pub use heuristics::h3;
pub use heuristics::h4;
pub use heuristics::h5;
pub use heuristics::Heuristic;
pub use population::Population;
pub use selection::{KTournamentSelection, SelectionStrategy};
