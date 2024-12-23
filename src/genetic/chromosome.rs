use crate::graph::{self, SimpleGraph};

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
                    1 => {
                        if !neighbors.iter().any(|&v| genes[v] > 0) {
                            return false;
                        }
                    }
                    2 => {
                        continue;
                    }
                    _ => return false, // Valores inválidos
                }
            } else {
                return false; // Erro ao obter vizinhos
            }
        }

        true
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
}
