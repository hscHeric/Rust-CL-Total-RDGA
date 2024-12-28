use rand::seq::SliceRandom;

use crate::graph::SimpleGraph;

#[derive(Debug, Clone)]
pub struct Chromosome {
    genes: Vec<u8>,
    fitness: Option<usize>,
}

impl Chromosome {
    pub fn new(genes: Vec<u8>) -> Self {
        Self {
            genes,
            fitness: None,
        }
    }

    fn evaluate_fitness(&mut self) {
        self.fitness = Some(self.genes.iter().copied().map(usize::from).sum())
    }

    pub fn fitness(&mut self) -> usize {
        if self.fitness.is_none() {
            self.evaluate_fitness();
        }

        self.fitness.unwrap()
    }

    pub fn genes(&self) -> Vec<u8> {
        self.genes.clone()
    }

    pub fn is_valid_to_total_roman_domination(&self, graph: &SimpleGraph) -> bool {
        let genes = self.genes();

        for vertex in 0..graph.vertex_count() {
            if let Ok(neighbors) = graph.neighbors(vertex) {
                match genes[vertex] {
                    0 => {
                        if !neighbors.iter().any(|&v| genes[v] == 2) {
                            return false;
                        }
                    }
                    1 | 2 => {
                        if !neighbors.iter().any(|&v| genes[v] > 0) {
                            return false;
                        }
                    }
                    _ => return false, // Valores inválidos
                }
            } else {
                return false; // Erro ao obter vizinhos
            }
        }

        true
    }

    pub fn fix_chromosome(&mut self, graph: &SimpleGraph) {
        let mut rng = rand::thread_rng();
        let vertex_count = graph.vertex_count();

        for vertex in 0..vertex_count {
            if let Ok(neighbors) = graph.neighbors(vertex) {
                let neighbors_vec: Vec<usize> = neighbors.iter().copied().collect();

                match self.genes[vertex] {
                    // Caso f(v) = 0
                    0 => {
                        // Verifica se existe vizinho com rótulo 2
                        let has_neighbor_with_2 = neighbors_vec.iter().any(|&n| self.genes[n] == 2);

                        // Se não existe vizinho com rótulo 2, seleciona aleatoriamente um vizinho
                        // e rotula com 1
                        if !has_neighbor_with_2 && !neighbors_vec.is_empty() {
                            if let Some(&random_neighbor) = neighbors_vec.choose(&mut rng) {
                                self.genes[random_neighbor] = 1;
                            }
                        }
                    }
                    // Caso f(v) > 0
                    1 | 2 => {
                        // Verifica se existe vizinho com rótulo > 0
                        let has_neighbor_greater_than_0 =
                            neighbors_vec.iter().any(|&n| self.genes[n] > 0);

                        // Se não existe vizinho com rótulo > 0, seleciona aleatoriamente um vizinho
                        // e rotula com 1
                        if !has_neighbor_greater_than_0 && !neighbors_vec.is_empty() {
                            if let Some(&random_neighbor) = neighbors_vec.choose(&mut rng) {
                                self.genes[random_neighbor] = 1;
                            }
                        }
                    }
                    // Caso inválido (não deveria ocorrer)
                    _ => self.genes[vertex] = 0,
                }
            }
        }
        // Reseta o fitness já que o cromossomo foi modificado
        self.fitness = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chromosome_creation() {
        let genes = vec![1, 0, 1, 1];
        let chromosome = Chromosome::new(genes.clone());
        assert_eq!(chromosome.genes(), genes);
        assert!(chromosome.fitness.is_none());
    }

    #[test]
    fn test_chromosome_fitness() {
        let genes = vec![1, 0, 1, 1];
        let mut chromosome = Chromosome::new(genes);
        assert_eq!(chromosome.fitness(), 3); // 1 + 0 + 1 + 1 = 3
    }

    #[test]
    fn test_chromosome_fitness_cached() {
        let genes = vec![1, 1, 1, 1];
        let mut chromosome = Chromosome::new(genes);
        let fitness_first = chromosome.fitness();
        let fitness_cached = chromosome.fitness();
        assert_eq!(fitness_first, fitness_cached);
    }

    #[test]
    fn test_valid_solution() {
        let mut graph = SimpleGraph::new();

        // Cria um grafo com 5 vértices conectados em ciclo
        for i in 0..5 {
            graph.add_vertex(i).unwrap();
        }
        graph.add_edge(0, 1).unwrap();
        graph.add_edge(1, 2).unwrap();
        graph.add_edge(2, 3).unwrap();
        graph.add_edge(3, 4).unwrap();
        graph.add_edge(4, 0).unwrap();

        // Solução válida
        let valid_chromosome = Chromosome::new(vec![2, 0, 0, 2, 1]);

        assert!(
            valid_chromosome.is_valid_to_total_roman_domination(&graph),
            "The chromosome should be valid"
        );
    }

    #[test]
    fn test_invalid_solution_vertex_0() {
        let mut graph = SimpleGraph::new();

        // Cria um grafo com 5 vértices conectados em ciclo
        for i in 0..5 {
            graph.add_vertex(i).unwrap();
        }
        graph.add_edge(0, 1).unwrap();
        graph.add_edge(1, 2).unwrap();
        graph.add_edge(2, 3).unwrap();
        graph.add_edge(3, 4).unwrap();
        graph.add_edge(4, 0).unwrap();

        // Solução inválida: vértice 0 com f(v) = 0 não tem vizinho com f(u) = 2
        let invalid_chromosome = Chromosome::new(vec![0, 0, 1, 2, 0]);

        assert!(
            !invalid_chromosome.is_valid_to_total_roman_domination(&graph),
            "The chromosome should be invalid because vertex 0 is not protected"
        );
    }

    #[test]
    fn test_invalid_solution_vertex_3() {
        let mut graph = SimpleGraph::new();

        // Cria um grafo com 5 vértices conectados em ciclo
        for i in 0..5 {
            graph.add_vertex(i).unwrap();
        }
        graph.add_edge(0, 1).unwrap();
        graph.add_edge(1, 2).unwrap();
        graph.add_edge(2, 3).unwrap();
        graph.add_edge(3, 4).unwrap();
        graph.add_edge(4, 0).unwrap();

        // Solução inválida: vértice 3 com f(v) = 2 não tem vizinho com f(u) > 0
        let invalid_chromosome = Chromosome::new(vec![2, 0, 0, 2, 0]);

        assert!(
            !invalid_chromosome.is_valid_to_total_roman_domination(&graph),
            "The chromosome should be invalid because vertex 3 lacks a neighbor with f(u) > 0"
        );
    }

    #[test]
    fn test_invalid_solution_invalid_gene() {
        let mut graph = SimpleGraph::new();

        // Cria um grafo com 5 vértices conectados em ciclo
        for i in 0..5 {
            graph.add_vertex(i).unwrap();
        }
        graph.add_edge(0, 1).unwrap();
        graph.add_edge(1, 2).unwrap();
        graph.add_edge(2, 3).unwrap();
        graph.add_edge(3, 4).unwrap();
        graph.add_edge(4, 0).unwrap();

        // Solução inválida: vértice 2 com um valor de gene inválido (f(v) = 3)
        let invalid_chromosome = Chromosome::new(vec![2, 1, 3, 0, 1]);

        assert!(
            !invalid_chromosome.is_valid_to_total_roman_domination(&graph),
            "The chromosome should be invalid due to an invalid gene value"
        );
    }

    #[test]
    fn test_empty_graph() {
        let graph = SimpleGraph::new();

        // Cromossomo vazio para um grafo vazio
        let empty_chromosome = Chromosome::new(vec![]);

        assert!(
            empty_chromosome.is_valid_to_total_roman_domination(&graph),
            "An empty chromosome should be valid for an empty graph"
        );
    }

    #[test]
    fn test_single_vertex_graph_valid() {
        let mut graph = SimpleGraph::new();

        graph.add_vertex(0).unwrap();

        // Solução inválida: vértice isolado com f(v) = 2
        let valid_chromosome = Chromosome::new(vec![2]);

        assert!(
            !valid_chromosome.is_valid_to_total_roman_domination(&graph),
            "The chromosome should be invalid for a single vertex with f(v) = 2"
        );
    }

    #[test]
    fn test_single_vertex_graph_invalid() {
        let mut graph = SimpleGraph::new();

        graph.add_vertex(0).unwrap();

        // Solução inválida: vértice isolado com f(v) = 0
        let invalid_chromosome = Chromosome::new(vec![0]);

        assert!(
            !invalid_chromosome.is_valid_to_total_roman_domination(&graph),
            "The chromosome should be invalid for a single vertex with f(v) = 0"
        );
    }
}
