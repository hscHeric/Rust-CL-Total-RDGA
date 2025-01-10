use kambo_graph::{graphs::simple::UndirectedGraph, Graph};
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct Chromosome {
    genes: Vec<u8>,
    fitness: usize, // Fitness armazenado diretamente
}

impl Chromosome {
    pub fn new(genes: Vec<u8>) -> Self {
        let fitness = genes.iter().copied().map(usize::from).sum();
        Self { genes, fitness }
    }

    pub fn fitness(&self) -> usize {
        self.fitness
    }

    pub fn genes(&self) -> &[u8] {
        &self.genes
    }

    pub fn is_valid_to_total_roman_domination(&self, graph: &UndirectedGraph<usize>) -> bool {
        let genes = &self.genes;

        for vertex in graph.vertices() {
            let neighbors = graph.neighbors(vertex);

            match genes[*vertex] {
                0 => {
                    // Vértice sem proteção, deve ter pelo menos um vizinho com f(u) = 2
                    if neighbors.is_none() || !neighbors.unwrap().any(|v| genes[*v] == 2) {
                        return false;
                    }
                }
                1 | 2 => {
                    // Vértice com proteção parcial/completa deve ter pelo menos um vizinho com f(u) > 0
                    if neighbors.is_none() || !neighbors.unwrap().any(|v| genes[*v] > 0) {
                        return false;
                    }
                }
                _ => {
                    // Valor inválido no gene
                    return false;
                }
            }
        }

        true
    }

    pub fn fix_chromosome(&self, graph: &UndirectedGraph<usize>) -> Chromosome {
        let mut rng = rand::thread_rng();
        let mut new_genes = self.genes.clone();

        for vertex in graph.vertices() {
            // Obtém os vizinhos do vértice
            if let Some(neighbors) = graph.neighbors(vertex) {
                let neighbors_vec: Vec<_> = neighbors.cloned().collect();

                match new_genes[*vertex] {
                    0 => {
                        // Verifica se existe vizinho com rótulo 2
                        if !neighbors_vec.iter().any(|&n| new_genes[n] == 2) {
                            // Seleciona aleatoriamente um vizinho e rotula como 2
                            if let Some(&random_neighbor) = neighbors_vec.choose(&mut rng) {
                                new_genes[random_neighbor] = 2;
                            }
                        }
                    }
                    1 | 2 => {
                        // Verifica se existe vizinho com rótulo > 0
                        if !neighbors_vec.iter().any(|&n| new_genes[n] > 0) {
                            // Seleciona aleatoriamente um vizinho e rotula como 1
                            if let Some(&random_neighbor) = neighbors_vec.choose(&mut rng) {
                                new_genes[random_neighbor] = 1;
                            }
                        }
                    }
                    _ => {
                        // Corrige valores inválidos
                        new_genes[*vertex] = 0;
                    }
                }
            }
        }

        // Retorna o novo cromossomo corrigido
        Chromosome::new(new_genes)
    }
}
#[cfg(test)]
mod tests {
    use kambo_graph::GraphMut;

    use super::*;

    #[test]
    fn test_chromosome_creation() {
        let genes = vec![1, 0, 1, 1];
        let chromosome = Chromosome::new(genes.clone());
        assert_eq!(chromosome.genes(), genes);
        assert_eq!(chromosome.fitness(), 3); // 1 + 0 + 1 + 1 = 3
    }

    #[test]
    fn test_chromosome_fitness() {
        let genes = vec![1, 0, 1, 1];
        let chromosome = Chromosome::new(genes);
        assert_eq!(chromosome.fitness(), 3); // 1 + 0 + 1 + 1 = 3
    }

    #[test]
    fn test_valid_solution() {
        let mut graph = UndirectedGraph::new_undirected();

        for i in 0..5 {
            graph.add_vertex(i).unwrap();
        }
        graph.add_edge(&0, &1).unwrap();
        graph.add_edge(&1, &2).unwrap();
        graph.add_edge(&2, &3).unwrap();
        graph.add_edge(&3, &4).unwrap();
        graph.add_edge(&4, &0).unwrap();

        let valid_chromosome = Chromosome::new(vec![2, 0, 0, 2, 1]);
        assert!(
            valid_chromosome.is_valid_to_total_roman_domination(&graph),
            "The chromosome should be valid"
        );
    }

    #[test]
    fn test_invalid_solution_vertex_3() {
        let mut graph = UndirectedGraph::new_undirected();

        for i in 0..5 {
            graph.add_vertex(i).unwrap();
        }
        graph.add_edge(&0, &1).unwrap();
        graph.add_edge(&1, &2).unwrap();
        graph.add_edge(&2, &3).unwrap();
        graph.add_edge(&3, &4).unwrap();
        graph.add_edge(&4, &0).unwrap();

        let invalid_chromosome = Chromosome::new(vec![2, 0, 0, 2, 0]);

        assert!(
            !invalid_chromosome.is_valid_to_total_roman_domination(&graph),
            "The chromosome should be invalid because vertex 3 lacks a neighbor with f(u) > 0"
        );
    }

    #[test]
    fn test_empty_graph() {
        let graph = UndirectedGraph::new_undirected();

        let empty_chromosome = Chromosome::new(vec![]);

        assert!(
            empty_chromosome.is_valid_to_total_roman_domination(&graph),
            "An empty chromosome should be valid for an empty graph"
        );
    }

    #[test]
    fn test_single_vertex_graph_valid() {
        let mut graph = UndirectedGraph::new_undirected();

        graph.add_vertex(0).unwrap();

        let valid_chromosome = Chromosome::new(vec![2]);

        assert!(
            !valid_chromosome.is_valid_to_total_roman_domination(&graph),
            "The chromosome should be invalid for a single vertex with f(v) = 2"
        );
    }

    #[test]
    fn test_single_vertex_graph_invalid() {
        let mut graph = UndirectedGraph::new_undirected();

        graph.add_vertex(0).unwrap();

        let invalid_chromosome = Chromosome::new(vec![0]);

        assert!(
            !invalid_chromosome.is_valid_to_total_roman_domination(&graph),
            "The chromosome should be invalid for a single vertex with f(v) = 0"
        );
    }
}
