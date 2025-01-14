use std::collections::HashSet;

use petgraph::graph::{NodeIndex, UnGraph};
use rand::seq::IteratorRandom;

use super::Chromosome;

/// Aliases to representation of a Heuristic
pub type Heuristic = fn(&UnGraph<u32, ()>) -> Chromosome;

/// Validates the provided `Chromosome` against the given graph.
///
/// # Arguments
/// - `chromosome`: A reference to the `Chromosome` whose genes are to be validated.
/// - `graph`: A reference to the undirected graph (`UnGraph`) being validated.
///
/// # Returns
/// - `true` if the chromosome adheres to the graph's constraints.
/// - `false` otherwise.
///
/// # Constraints
/// 1. A vertex with label `0` must have at least one neighbor with label `2`.
/// 2. A vertex with label `1` or `2` must have at least one neighbor with a label greater than `0`.
/// 3. Labels outside the range `[0, 2]` are considered invalid.
#[must_use]
pub fn is_valid(chromosome: &Chromosome, graph: &UnGraph<u32, ()>) -> bool {
    for vertex in graph.node_indices() {
        match chromosome.genes()[vertex.index()] {
            0 => {
                // Vértice com rótulo 0 deve ter pelo menos um vizinho com rótulo 2
                if !graph
                    .neighbors(vertex)
                    .any(|v| chromosome.genes()[v.index()] == 2)
                {
                    return false;
                }
            }
            1 | 2 => {
                // Vértice com rótulo 1 ou 2 deve ter pelo menos um vizinho com rótulo > 0
                if !graph
                    .neighbors(vertex)
                    .any(|v| chromosome.genes()[v.index()] > 0)
                {
                    return false;
                }
            }
            _ => return false, // Rótulo inválido
        }
    }
    true
}

/// A heuristic function to generate a `Chromosome` using a randomized approach.
///
/// # Arguments
/// - `graph`: A reference to the undirected graph (`UnGraph`) for which the chromosome is generated.
///
/// # Returns
/// - A `Chromosome` where genes are assigned based on the following procedure:
///   - Randomly select a vertex and assign it label `2`.
///   - Assign one of its neighbors label `1`.
///   - Remaining neighbors are labeled `0`.
///   - Isolated vertices are handled separately and assigned labels to satisfy constraints.
#[must_use]
pub fn h1(graph: &UnGraph<u32, ()>) -> Chromosome {
    let mut genes = vec![0u8; graph.node_count()];
    let h = graph.clone();
    let mut rng = rand::thread_rng();

    let mut removed: HashSet<usize> = HashSet::new();

    while let Some(v) = h
        .node_indices()
        .filter(|n| !removed.contains(&n.index()))
        .choose(&mut rng)
    {
        genes[v.index()] = 2;

        let neighbors: Vec<NodeIndex> = h.neighbors(v).collect();
        if let Some(&first_neighbor) = neighbors.iter().find(|&&n| genes[n.index()] == 0) {
            genes[first_neighbor.index()] = 1;

            // Continuamos marcando os demais vizinhos como 0
            for &w in neighbors.iter().filter(|&&n| n != first_neighbor) {
                genes[w.index()] = 0;
            }
        }

        removed.insert(v.index());
        for n in neighbors {
            removed.insert(n.index());
        }

        let isolated: Vec<NodeIndex> = h
            .node_indices()
            .filter(|&n| !removed.contains(&n.index()))
            .filter(|&n| {
                h.neighbors(n).count() == 0
                    || h.neighbors(n).all(|nb| removed.contains(&nb.index()))
            })
            .collect();

        for z in isolated {
            genes[z.index()] = 1;

            let has_neighbor_with_1 = graph.neighbors(z).any(|n| genes[n.index()] == 1);

            if !has_neighbor_with_1 {
                if let Some(n) = graph.neighbors(z).find(|&n| genes[n.index()] == 0) {
                    genes[n.index()] = 1;
                }
            }

            removed.insert(z.index());
        }
    }

    Chromosome::new(genes)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_h1() {
        let mut graph = UnGraph::<u32, ()>::new_undirected();
        let v0 = graph.add_node(0); // Nó 0
        let v1 = graph.add_node(1); // Nó 1
        let v2 = graph.add_node(2); // Nó 2
        let v3 = graph.add_node(3); // Nó 3

        graph.add_edge(v0, v1, ());
        graph.add_edge(v1, v2, ());
        graph.add_edge(v2, v3, ());
        graph.add_edge(v3, v0, ());

        let chromosome: Chromosome = h1(&graph);

        assert!(is_valid(&chromosome, &graph));
    }
}
