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
///
/// # Example Usage
/// ```rust
/// use crate::chromosome::Chromosome;
/// use petgraph::graph::UnGraph;
///
/// // Create a chromosome
/// let mut chromosome = Chromosome::new(vec![0, 0, 0]);
///
/// // Create a simple graph
/// let mut graph = UnGraph::<usize, ()>::new_undirected();
/// let v0 = graph.add_node(0);
/// let v1 = graph.add_node(1);
/// let v2 = graph.add_node(2);
/// graph.add_edge(v0, v1, ());
/// graph.add_edge(v1, v2, ());
///
/// // Fix the chromosome
/// chromosome.fix(&graph);
///
/// // Check the corrected genes
/// println!("{:?}", chromosome.genes());
/// ```
pub mod chromosome;

pub use chromosome::Chromosome;
