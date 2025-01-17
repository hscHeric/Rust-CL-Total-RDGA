use kambo_graph::{graphs::simple::UndirectedGraph, Graph, GraphMut};
use rand::seq::IteratorRandom;

use super::chromosome::Chromosome;

/// Aliases to representation of a Heuristic
pub type Heuristic = fn(&UndirectedGraph<usize>) -> Chromosome;

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
pub fn h1(graph: &UndirectedGraph<usize>) -> Chromosome {
    // Inicializa um vetor de genes com valores 0.
    // O tamanho do vetor é igual ao número de vértices no grafo.
    let mut genes = vec![0u8; graph.order()];

    // Faz uma cópia do grafo original para ser manipulado sem alterar o original.
    let mut h = graph.clone();

    // Cria um gerador de números aleatórios para escolher vértices aleatoriamente.
    let mut rng = rand::thread_rng();

    // Enquanto o grafo h ainda tiver vértices...
    while let Some(v) = h.vertices().choose(&mut rng).copied() {
        // Passo 4: Define f(v) = 2, marcando o vértice v com a cor 2.
        genes[v] = 2;

        // Obtém os vizinhos de v no grafo `h`.
        let neighbors: Vec<usize> = h
            .neighbors(&v)
            .map(|n| n.copied().collect())
            .unwrap_or_default();

        // Passo 5: Se v tem vizinhos, escolha um (o primeiro da lista) e defina f(u) = 1.
        if let Some(first_neighbor) = neighbors.first() {
            genes[*first_neighbor] = 1;

            // Passo 6: Para os demais vizinhos de v, define f(w) = 0.
            for w in neighbors.iter().skip(1) {
                genes[*w] = 0;
            }
        }

        // Passo 7: Remove o vértice `v` e seus vizinhos do grafo `h`.
        let _ = h.remove_vertex(&v);
        for neighbor in neighbors {
            let _ = h.remove_vertex(&neighbor);
        }

        // Passo 8: Enquanto houver vértices isolados em h...
        let isolated_vertices = h.get_isolated_vertices();
        for z in isolated_vertices {
            // Caso contrário, define f(z) = 1.
            genes[z] = 1;
            let has_neighbor_with_1 = graph
                .neighbors(&z)
                .is_some_and(|mut neighbors| neighbors.any(|n| genes[*n] == 1));

            // Verifica se `z` tem vizinhos no grafo original com f = 1.
            if !has_neighbor_with_1 {
                // Se não há vizinhos com f = 1, escolhe um vizinho com f = 0 e define f = 1.
                if let Some(mut neighbors) = graph.neighbors(&z) {
                    if let Some(first) = neighbors.find(|&n| genes[*n] == 0) {
                        genes[*first] = 1;
                    }
                }
            }

            // Passo 12: Remove o vértice `z` do grafo `h`.
            let _ = h.remove_vertex(&z);
        }
    }

    // Retorna a solução como um Chromosome, encapsulando o vetor de genes.
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
pub fn h2(graph: &UndirectedGraph<usize>) -> Option<Chromosome> {
    // Inicializa um vetor de genes com valores 0.
    // O tamanho do vetor é igual ao número de vértices no grafo.
    let mut genes = vec![0u8; graph.order()];

    // Faz uma cópia do grafo original para ser manipulado sem alterar o original.
    let mut h = graph.clone();

    // Enquanto o grafo h ainda tiver vértices... (Já captura o v = vértice de maior grau do grafo)
    while let Some(v) = h.vertices().max_by_key(|&vertex| h.degree(vertex)).copied() {
        // Passo 4: Define f(v) = 2, marcando o vértice v com a cor 2.
        genes[v] = 2;

        // Obtém os vizinhos de v no grafo `h`.
        let neighbors: Vec<usize> = h
            .neighbors(&v)
            .map(|n| n.copied().collect())
            .unwrap_or_default();

        // Passo 5: Se v tem vizinhos, escolha um (o primeiro da lista) e defina f(u) = 1.
        if let Some(first_neighbor) = neighbors.first() {
            genes[*first_neighbor] = 1;

            // Passo 6: Para os demais vizinhos de v, define f(w) = 0.
            for w in neighbors.iter().skip(1) {
                genes[*w] = 0;
            }
        }

        // Passo 7: Remove o vértice `v` e seus vizinhos do grafo `h`.
        let _ = h.remove_vertex(&v);
        for neighbor in neighbors {
            let _ = h.remove_vertex(&neighbor);
        }

        // Passo 8: Enquanto houver vértices isolados em h...
        let isolated_vertices = h.get_isolated_vertices();
        for z in isolated_vertices {
            genes[z] = 1;
            let has_neighbor_with_1 = graph
                .neighbors(&z)
                .is_some_and(|mut neighbors| neighbors.any(|n| genes[*n] == 1));

            // Verifica se `z` tem vizinhos no grafo original com f = 1.
            if !has_neighbor_with_1 {
                // Se não há vizinhos com f = 1, escolhe um vizinho com f = 0 e define f = 1.
                if let Some(mut neighbors) = graph.neighbors(&z) {
                    if let Some(first) = neighbors.find(|&n| genes[*n] == 0) {
                        genes[*first] = 1;
                    }
                }
            }

            // Passo 12: Remove o vértice `z` do grafo `h`.
            let _ = h.remove_vertex(&z);
        }
    }

    // Retorna a solução como um Chromosome, encapsulando o vetor de genes.
    Some(Chromosome::new(genes))
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
pub fn h3(graph: &UndirectedGraph<usize>) -> Option<Chromosome> {
    // Inicializa um vetor de genes com valores 0.
    // O tamanho do vetor é igual ao número de vértices no grafo.
    let mut genes = vec![0u8; graph.order()];

    // Faz uma cópia do grafo original para ser manipulado sem alterar o original.
    let mut h = graph.clone();

    // Enquanto o grafo h ainda tiver vértices... (Já captura o v = vértice de maior grau do grafo)
    while let Some(v) = h.vertices().max_by_key(|&vertex| h.degree(vertex)).copied() {
        // Passo 4: Define f(v) = 2, marcando o vértice v com a cor 2.
        genes[v] = 2;

        // Obtém os vizinhos de v no grafo `h`.
        let mut neighbors: Vec<usize> = h
            .neighbors(&v)
            .map(|n| n.copied().collect())
            .unwrap_or_default();

        // Ordena os vizinhos de forma decrescente pelo grau
        neighbors.sort_by(|&a, &b| h.degree(&b).cmp(&h.degree(&a)));

        // Passo 5: Se v tem vizinhos, escolha um (o primeiro da lista, ou seja, o com maior grau) e defina f(u) = 1.
        if let Some(first_neighbor) = neighbors.first() {
            genes[*first_neighbor] = 1;

            // Passo 6: Para os demais vizinhos de v, define f(w) = 0.
            for w in neighbors.iter().skip(1) {
                genes[*w] = 0;
            }
        }

        // Passo 7: Remove o vértice `v` e seus vizinhos do grafo `h`.
        let _ = h.remove_vertex(&v);
        for neighbor in neighbors {
            let _ = h.remove_vertex(&neighbor);
        }

        // Passo 8: Enquanto houver vértices isolados em h...
        let isolated_vertices = h.get_isolated_vertices();
        for z in isolated_vertices {
            // Caso contrário, define f(z) = 1.
            genes[z] = 1;
            let has_neighbor_with_1 = graph
                .neighbors(&z)
                .is_some_and(|mut neighbors| neighbors.any(|n| genes[*n] == 1));

            // Verifica se `z` tem vizinhos no grafo original com f = 1.
            if !has_neighbor_with_1 {
                // Se não há vizinhos com f = 1, escolhe um vizinho com f = 0 e define f = 1.
                if let Some(mut neighbors) = graph.neighbors(&z) {
                    if let Some(first) = neighbors.find(|&n| genes[*n] == 0) {
                        genes[*first] = 1;
                    }
                }
            }

            // Passo 12: Remove o vértice `z` do grafo `h`.
            let _ = h.remove_vertex(&z);
        }
    }

    // Retorna a solução como um Chromosome, encapsulando o vetor de genes.
    Some(Chromosome::new(genes))
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
pub fn h4(graph: &UndirectedGraph<usize>) -> Option<Chromosome> {
    // Inicializa um vetor de genes com valores 0.
    // O tamanho do vetor é igual ao número de vértices no grafo.
    let mut genes = vec![0u8; graph.order()];

    // Faz uma cópia do grafo original para ser manipulado sem alterar o original.
    let mut h = graph.clone();

    // Enquanto o grafo h ainda tiver vértices... (Já captura o v = vértice de maior grau do grafo)
    while let Some(v) = h.vertices().max_by_key(|&vertex| h.degree(vertex)).copied() {
        // Passo 4: Define f(v) = 2, marcando o vértice v com a cor 2.
        genes[v] = 2;

        // Obtém os vizinhos de v no grafo `h`.
        let mut neighbors: Vec<usize> = h
            .neighbors(&v)
            .map(|n| n.copied().collect())
            .unwrap_or_default();

        // Ordena os vizinhos de forma decrescente pelo grau
        neighbors.sort_by(|&a, &b| h.degree(&b).cmp(&h.degree(&a)));

        // Passo 5: Se v tem vizinhos, escolha um (o primeiro da lista, ou seja, o com maior grau) e defina f(u) = 1.
        if let Some(first_neighbor) = neighbors.first() {
            genes[*first_neighbor] = 1;

            // Passo 6: Para os demais vizinhos de v, define f(w) = 0.
            for w in neighbors.iter().skip(1) {
                genes[*w] = 0;
            }
        }

        // Passo 7: Remove o vértice `v` e seus vizinhos do grafo `h`.
        let _ = h.remove_vertex(&v);
        for neighbor in neighbors {
            let _ = h.remove_vertex(&neighbor);
        }

        // Passo 8-14: Processa vértices isolados
        loop {
            // Encontra vértices isolados em H
            let isolated: Vec<usize> = h
                .vertices()
                .filter(|&v| h.degree(v).unwrap_or(0) == 0)
                .copied()
                .collect();

            if isolated.is_empty() {
                break;
            }

            // Encontra os vizinhos dos vértices isolados no grafo original
            let mut ns: Vec<usize> = Vec::new();
            for &s in &isolated {
                ns.extend(
                    graph
                        .neighbors(&s)
                        .map(|n| n.copied().collect::<Vec<_>>())
                        .unwrap_or_default(),
                );
            }
            ns.sort_unstable();
            ns.dedup();

            // Para cada vizinho z em N(S)
            for &z in &ns {
                // Conta quantos vizinhos z tem em S
                let isolated_neighbors = isolated
                    .iter()
                    .filter(|&&s| graph.contains_edge(&z, &s))
                    .count();

                if isolated_neighbors >= 2 {
                    // Se z tem 2 ou mais vizinhos em S, define f(z) = 2
                    genes[z] = 2;
                    // E seus vizinhos em S recebem 0
                    for &s in &isolated {
                        if graph.contains_edge(&z, &s) {
                            genes[s] = 0;
                        }
                    }
                } else {
                    // Caso contrário, também recebe 2
                    genes[z] = 2;
                }
            }

            // Pinta com 0 os vértices restantes em S
            for &s in &isolated {
                if genes[s] == 0 {
                    genes[s] = 0;
                }
            }

            // Remove todos os vértices de S do grafo H
            for s in isolated {
                let _ = h.remove_vertex(&s);
            }
        }
    }

    // Retorna a solução como um Chromosome, encapsulando o vetor de genes.
    Some(Chromosome::new(genes))
}

// pub fn h2(graph: &UndirectedGraph<usize>) -> Option<Chromosome> {
//     let mut genes = vec![0u8; graph.order()];
//     let mut h = graph.clone();
//
//     // Passo 2: Enquanto tiver vértices em H faça (já capturo o vértice de maior grau):
//     while let Some(v) = h.vertices().max_by_key(|&v| h.degree(v)).cloned() {
//         genes[v] = 2;
//
//         // Obtém os vizinhos de v no grafo `h`.
//         let neighbors: Vec<usize> = h
//             .neighbors(&v)
//             .map(|n| n.cloned().collect())
//             .unwrap_or_default();
//
//         // Passo 5: Se v tem vizinhos, escolha um (o primeiro da lista) e defina f(u) = 1.
//         if let Some(first_neighbor) = neighbors.first() {
//             genes[*first_neighbor] = 1;
//
//             // Passo 6: Para os demais vizinhos de v, define f(w) = 0.
//             for w in neighbors.iter().skip(1) {
//                 genes[*w] = 0;
//             }
//         }
//
//         // Passo 7: Remove o vértice `v` e seus vizinhos do grafo `h`.
//         let _ = h.remove_vertex(&v);
//         for neighbor in neighbors {
//             let _ = h.remove_vertex(&neighbor);
//         }
//
//         // Passo 8: Enquanto houver vértices isolados em h...
//         let isolated_vertices = h.get_isolated_vertices();
//         for z in isolated_vertices {
//             // Verifica se `z` tem vizinhos no grafo original `graph` com f = 2.
//             let has_neighbor_with_2 = graph
//                 .neighbors(&z)
//                 .map(|mut neighbors| neighbors.any(|n| genes[*n] == 2))
//                 .unwrap_or(false);
//
//             // Se existe algum vizinho com f = 2, define f(z) = 0.
//             if has_neighbor_with_2 {
//                 genes[z] = 0;
//             } else {
//                 // Caso contrário, define f(z) = 1.
//                 genes[z] = 1;
//                 let has_neighbor_with_1 = graph
//                     .neighbors(&z)
//                     .map(|mut neighbors| neighbors.any(|n| genes[*n] == 1))
//                     .unwrap_or(false);
//
//                 // Verifica se `z` tem vizinhos no grafo original com f = 1.
//                 if !has_neighbor_with_1 {
//                     // Se não há vizinhos com f = 1, escolhe um vizinho com f = 0 e define f = 1.
//                     if let Some(mut neighbors) = graph.neighbors(&z) {
//                         if let Some(first) = neighbors.find(|&n| genes[*n] == 0) {
//                             genes[*first] = 1;
//                         }
//                     }
//                 }
//             }
//
//             // Passo 12: Remove o vértice `z` do grafo `h`.
//             let _ = h.remove_vertex(&z);
//         }
//     }
//
//     Some(Chromosome::new(genes))
// }

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
pub fn h5(graph: &UndirectedGraph<usize>) -> Option<Chromosome> {
    // Cria um vetor de genes com todos os vértices rotulados com valor 1;
    let genes: Vec<u8> = vec![1; graph.order()];
    Some(Chromosome::new(genes))
}
