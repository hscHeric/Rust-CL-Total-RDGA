use kambo_graph::{graphs::simple::UndirectedGraph, Graph, GraphMut};

use super::Chromosome;

pub fn h1(graph: &UndirectedGraph<usize>) -> Option<Chromosome> {
    let mut h = graph.clone();
    let mut genes = vec![3u8; graph.order()];

    let mut vertices: Vec<_> = h.vertices().cloned().collect();
    vertices.sort();

    for v in vertices {
        if genes[v] == 3 {
            let degree = h.degree(&v).unwrap_or(0);

            if degree == 0 {
                genes[v] = 1;

                if let Some(neighbors) = graph.neighbors(&v) {
                    let mut neighbors_vec: Vec<_> = neighbors.cloned().collect();
                    neighbors_vec.sort(); // Ordena os vizinhos para consistência

                    if !neighbors_vec.iter().any(|&n| h.degree(&n).unwrap_or(0) > 0) {
                        if let Some(&neighbor) = neighbors_vec.first() {
                            genes[neighbor] = 1;
                        }
                    }
                }

                let _ = h.remove_vertex(&v);
            } else if degree == 1 {
                genes[v] = 1;
                let neighbors_vec: Vec<_> = h.neighbors(&v).unwrap().cloned().collect();
                if let Some(&neighbor) = neighbors_vec.first() {
                    genes[neighbor] = 1;
                    h.remove_vertex(&v).ok();
                    h.remove_vertex(&neighbor).ok();
                }
            } else {
                genes[v] = 2;
                let mut neighbors_vec: Vec<_> = h.neighbors(&v).unwrap().cloned().collect();
                neighbors_vec.sort(); // Ordena os vizinhos para consistência

                if let Some(&first_neighbor) = neighbors_vec.first() {
                    genes[first_neighbor] = 1;
                }

                for &neighbor in neighbors_vec.iter().skip(1) {
                    genes[neighbor] = 0;
                }

                h.remove_vertex(&v).ok();
                for neighbor in neighbors_vec {
                    h.remove_vertex(&neighbor).ok();
                }
            }
        }
    }

    Some(Chromosome::new(genes))
}

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
