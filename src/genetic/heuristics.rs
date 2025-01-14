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

/// A heuristic function to generate a `Chromosome` using a vertex degree-based approach.
///
/// # Overview
/// This heuristic assigns labels to vertices in an undirected graph (`UnGraph`) by prioritizing
/// vertices with the highest degree (number of neighbors). This strategy aims to maximize the
/// impact of the labels on highly connected vertices, which are likely to influence the overall
/// graph structure.
///
/// # Arguments
/// - `graph`: A reference to the undirected graph (`UnGraph`) for which the chromosome is generated.
///
/// # Returns
/// - A `Chromosome` where genes are assigned based on the following procedure:
///   1. Identify the vertex with the highest degree that has not yet been processed.
///   2. Assign label `2` to the selected vertex.
///   3. Assign label `1` to one of its neighbors, prioritizing those with label `0`.
///   4. Assign label `0` to the remaining neighbors.
///   5. Repeat the process until all vertices are labeled.
///   6. Handle isolated vertices separately, ensuring they satisfy the constraints.
///
/// # Constraints
/// The following constraints are applied during the labeling process:
/// - A vertex with label `0` must have at least one neighbor with label `2`.
/// - A vertex with label `1` or `2` must have at least one neighbor with a label greater than `0`.
/// - Isolated vertices are assigned label `1` by default, with adjustments to satisfy the above rules.
///
/// # Notes
/// This heuristic is similar to `h1`, but it prioritizes vertices with the highest degree
/// during the selection process, aiming to optimize the influence of the assigned labels.
#[must_use]
pub fn h2(graph: &UnGraph<u32, ()>) -> Chromosome {
    let mut genes = vec![0u8; graph.node_count()];
    let h = graph.clone();
    let mut removed: HashSet<usize> = HashSet::new();

    while let Some(v) = h
        .node_indices()
        .filter(|n| !removed.contains(&n.index()))
        .max_by_key(|&n| h.neighbors(n).count())
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

        // Processa vértices isolados (mesmo código do h1)
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

/// A heuristic function to generate a `Chromosome` using a degree-based and neighbor-priority approach.
///
/// # Overview
/// This heuristic assigns labels to vertices in an undirected graph (`UnGraph`) by prioritizing vertices with
/// the highest degree and further refining the selection of neighbors based on their degrees. The goal is to
/// maximize the influence of labels while ensuring constraints are satisfied.
///
/// # Arguments
/// - `graph`: A reference to the undirected graph (`UnGraph`) for which the chromosome is generated.
///
/// # Returns
/// - A `Chromosome` where genes are assigned based on the following procedure:
///   1. Select the vertex with the highest degree among the unprocessed vertices and assign it label `2`.
///   2. Sort its neighbors by their degrees in descending order and assign label `1` to the neighbor with the
///      highest degree that is currently labeled `0`.
///   3. Assign label `0` to the remaining neighbors.
///   4. Repeat the process until all vertices are labeled.
///   5. Handle isolated vertices separately to ensure all constraints are satisfied.
///
/// # Constraints
/// The following constraints are enforced during the labeling process:
/// - A vertex with label `0` must have at least one neighbor with label `2`.
/// - A vertex with label `1` or `2` must have at least one neighbor with a label greater than `0`.
/// - Isolated vertices are assigned label `1` by default, with adjustments as necessary.
///
/// # Notes
/// - This heuristic refines the approach of `h2` by introducing a sorting step to prioritize neighbors with higher degrees.
/// - It is particularly useful in graphs where the connectivity of neighbors significantly influences the solution.
#[must_use]
pub fn h3(graph: &UnGraph<u32, ()>) -> Chromosome {
    let mut genes = vec![0u8; graph.node_count()];
    let h = graph.clone();
    let mut removed: HashSet<usize> = HashSet::new();

    while let Some(v) = h
        .node_indices()
        .filter(|n| !removed.contains(&n.index()))
        .max_by_key(|&n| h.neighbors(n).count())
    {
        genes[v.index()] = 2;

        let mut neighbors: Vec<NodeIndex> = h.neighbors(v).collect();
        neighbors.sort_by_key(|&n| std::cmp::Reverse(h.neighbors(n).count()));
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

        // Processa vértices isolados (mesmo código do h1)
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

/// A heuristic function to generate a `Chromosome` using a degree-based and isolated vertex clustering approach.
///
/// # Overview
/// This heuristic assigns labels to vertices in an undirected graph (`UnGraph`) by prioritizing high-degree vertices
/// and clustering isolated vertices with common neighbors. It ensures all constraints are met while minimizing label violations.
///
/// # Arguments
/// - `graph`: A reference to the undirected graph (`UnGraph`) for which the chromosome is generated.
///
/// # Returns
/// - A `Chromosome` where genes are assigned based on the following procedure:
///   1. Select the vertex with the highest degree among unprocessed vertices and assign it label `2`.
///   2. Sort its neighbors by degree in descending order. Assign label `1` to the highest-degree neighbor, and label `0` to the rest.
///   3. Repeat this process until all vertices are processed.
///   4. For isolated vertices:
///      - Cluster them with their common neighbors, assigning labels to maintain the constraints.
///      - If a vertex has at least two connections to isolated vertices, it is prioritized for label `2`.
///
/// # Constraints
/// - A vertex with label `0` must have at least one neighbor with label `2`.
/// - A vertex with label `1` or `2` must have at least one neighbor with a label greater than `0`.
/// - Isolated vertices are clustered based on shared neighbors and labeled accordingly.
///
/// # Notes
/// - This heuristic extends `h3` by introducing specific handling for isolated vertices, grouping them
///   into clusters based on their connections to common neighbors.
/// - It is particularly useful for graphs with sparse regions or large numbers of isolated vertices.
#[must_use]
pub fn h4(graph: &UnGraph<u32, ()>) -> Chromosome {
    let mut genes = vec![0u8; graph.node_count()];
    let h = graph.clone();
    let mut removed = HashSet::new();

    while let Some(v) = h
        .node_indices()
        .filter(|n| !removed.contains(&n.index()))
        .max_by_key(|&n| h.neighbors(n).count())
    {
        genes[v.index()] = 2;

        let mut neighbors: Vec<NodeIndex> = h.neighbors(v).collect();
        neighbors.sort_by_key(|&n| std::cmp::Reverse(h.neighbors(n).count()));

        if let Some(&first_neighbor) = neighbors.first() {
            genes[first_neighbor.index()] = 1;

            for &w in neighbors.iter().skip(1) {
                genes[w.index()] = 0;
            }
        }

        removed.insert(v.index());
        for n in neighbors {
            removed.insert(n.index());
        }

        loop {
            let isolated: Vec<NodeIndex> = h
                .node_indices()
                .filter(|&n| !removed.contains(&n.index()))
                .filter(|&n| {
                    h.neighbors(n).count() == 0
                        || h.neighbors(n).all(|nb| removed.contains(&nb.index()))
                })
                .collect();

            if isolated.is_empty() {
                break;
            }

            let mut ns = HashSet::new();
            for &s in &isolated {
                ns.extend(graph.neighbors(s));
            }

            for &z in &ns {
                let isolated_neighbors = isolated
                    .iter()
                    .filter(|&&s| graph.contains_edge(z, s))
                    .count();

                if isolated_neighbors >= 2 {
                    genes[z.index()] = 2;
                    for &s in &isolated {
                        if graph.contains_edge(z, s) {
                            genes[s.index()] = 0;
                        }
                    }
                } else {
                    genes[z.index()] = 2;
                }
            }

            for &s in &isolated {
                if genes[s.index()] == 0 {
                    genes[s.index()] = 0;
                }
            }

            for s in isolated {
                removed.insert(s.index());
            }
        }
    }

    Chromosome::new(genes)
}

/// A heuristic function to generate a `Chromosome` by assigning a default label to all vertices.
///
/// # Overview
/// This heuristic generates a `Chromosome` where all vertices in the graph are assigned the same label (`1`).
/// It serves as a baseline or trivial solution, ensuring all vertices satisfy a minimum labeling constraint.
///
/// # Arguments
/// - `graph`: A reference to the undirected graph (`UnGraph`) for which the chromosome is generated.
///
/// # Returns
/// - A `Chromosome` where all genes are assigned the label `1`.
#[must_use]
pub fn h5(graph: &UnGraph<u32, ()>) -> Chromosome {
    let genes = vec![1u8; graph.node_count()];
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

    #[test]
    fn test_h2() {
        let mut graph = UnGraph::<u32, ()>::new_undirected();
        let v0 = graph.add_node(0); // Nó 0
        let v1 = graph.add_node(1); // Nó 1
        let v2 = graph.add_node(2); // Nó 2
        let v3 = graph.add_node(3); // Nó 3

        graph.add_edge(v0, v1, ());
        graph.add_edge(v1, v2, ());
        graph.add_edge(v2, v3, ());
        graph.add_edge(v3, v0, ());

        let chromosome: Chromosome = h2(&graph);
        assert!(is_valid(&chromosome, &graph));
    }

    #[test]
    fn test_h3() {
        let mut graph = UnGraph::<u32, ()>::new_undirected();
        let v0 = graph.add_node(0); // Nó 0
        let v1 = graph.add_node(1); // Nó 1
        let v2 = graph.add_node(2); // Nó 2
        let v3 = graph.add_node(3); // Nó 3

        graph.add_edge(v0, v1, ());
        graph.add_edge(v1, v2, ());
        graph.add_edge(v2, v3, ());
        graph.add_edge(v3, v0, ());

        let chromosome: Chromosome = h3(&graph);
        assert!(is_valid(&chromosome, &graph));
    }

    #[test]
    fn test_h4() {
        let mut graph = UnGraph::<u32, ()>::new_undirected();
        let v0 = graph.add_node(0); // Nó 0
        let v1 = graph.add_node(1); // Nó 1
        let v2 = graph.add_node(2); // Nó 2
        let v3 = graph.add_node(3); // Nó 3

        graph.add_edge(v0, v1, ());
        graph.add_edge(v1, v2, ());
        graph.add_edge(v2, v3, ());
        graph.add_edge(v3, v0, ());

        let chromosome: Chromosome = h4(&graph);
        assert!(is_valid(&chromosome, &graph));
    }

    #[test]
    fn test_h5() {
        let mut graph = UnGraph::<u32, ()>::new_undirected();
        let v0 = graph.add_node(0); // Nó 0
        let v1 = graph.add_node(1); // Nó 1
        let v2 = graph.add_node(2); // Nó 2
        let v3 = graph.add_node(3); // Nó 3

        graph.add_edge(v0, v1, ());
        graph.add_edge(v1, v2, ());
        graph.add_edge(v2, v3, ());
        graph.add_edge(v3, v0, ());

        let chromosome: Chromosome = h5(&graph);
        assert!(is_valid(&chromosome, &graph));
    }
}
