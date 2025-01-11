use kambo_graph::{graphs::simple::UndirectedGraph, Graph};

use super::Chromosome;

// pub fn h1(graph: &UndirectedGraph<usize>) -> Option<Chromosome> {
//     let mut labels = vec![0u8; graph.order()];
//     let mut working_graph = graph.clone();
//
//     loop {
//         let vertex_with_edges = working_graph
//             .vertices()
//             .find(|&&v| working_graph.degree(&v).unwrap_or(0) > 0)
//             .cloned();
//
//         let vertex = match vertex_with_edges {
//             Some(v) => v,
//             None => break,
//         };
//
//         let neighbors: Vec<_> = working_graph
//             .neighbors(&vertex)
//             .map(|iter| iter.cloned().collect())
//             .unwrap_or_default();
//
//         if !neighbors.is_empty() {
//             labels[vertex] = 2;
//
//             labels[neighbors[0]] = 1;
//             for &neighbor in &neighbors[1..] {
//                 labels[neighbor] = 0;
//             }
//
//             working_graph.remove_vertex(&vertex).ok();
//             for neighbor in neighbors {
//                 working_graph.remove_vertex(&neighbor).ok();
//             }
//         }
//     }
//
//     let isolated_vertices: Vec<_> = working_graph.get_isolated_vertices();
//     for isolated_vertex in isolated_vertices {
//         if let Some(neighbors) = graph.neighbors(&isolated_vertex) {
//             let neighbors: Vec<_> = neighbors.cloned().collect();
//             if let Some(&neighbor) = neighbors.first() {
//                 if labels[neighbor] != 2 {
//                     labels[neighbor] = 2;
//                     labels[isolated_vertex] = 1;
//                 }
//             }
//         }
//     }
//
//     let chromosome = Chromosome::new(labels);
//     if chromosome.is_valid_to_total_roman_domination(graph) {
//         Some(chromosome)
//     } else {
//         None
//     }
// }

pub fn h2(graph: &UndirectedGraph<usize>) -> Option<Chromosome> {
    todo!()
}

pub fn h3(graph: &UndirectedGraph<usize>) -> Option<Chromosome> {
    todo!()
}

pub fn h4(graph: &UndirectedGraph<usize>) -> Option<Chromosome> {
    todo!()
}

pub fn h5(graph: &UndirectedGraph<usize>) -> Option<Chromosome> {
    let genes: Vec<u8> = vec![1; graph.order()];
    Some(Chromosome::new(genes))
}
