use rand::{seq::IteratorRandom, Rng};

use crate::graph::SimpleGraph;

use super::Chromosome;

#[derive(Debug, Default)]
pub struct Population {
    individuals: Vec<Chromosome>,
    size: usize,
}

#[derive(Debug)]
pub enum PopulationError {
    NotEnoughHeuristics,
    InvalidPopulationSize,
    HeuristicFailed,
    PopulationEmpyt,
    IsolatedVerticesFound,
}

impl Population {
    pub fn new<F>(
        graph: &SimpleGraph,
        heuristics: Vec<F>,
        size: usize,
    ) -> Result<Self, PopulationError>
    where
        F: Fn(&SimpleGraph) -> Option<Chromosome>,
    {
        if !graph.get_isolated_vertices().is_empty() {
            return Err(PopulationError::IsolatedVerticesFound);
        }

        let min_size = heuristics.len() + 1;
        if size < min_size {
            return Err(PopulationError::InvalidPopulationSize);
        }

        let mut individuals: Vec<Chromosome> = Vec::with_capacity(size);

        for heuristic in heuristics {
            match heuristic(graph) {
                Some(chromosome) => individuals.push(chromosome),
                None => return Err(PopulationError::HeuristicFailed),
            }
        }

        while individuals.len() < size {
            let chromosome = Self::generate_random_valid_chromosome(graph);
            individuals.push(chromosome);
        }

        let size = individuals.len();

        Ok(Self { individuals, size })
    }

    fn generate_random_valid_chromosome(graph: &SimpleGraph) -> Chromosome {
        let mut rng = rand::thread_rng();
        let mut retries = 0;
        const MAX_RETRIES: u32 = 100;

        while retries < MAX_RETRIES {
            let mut genes = vec![0; graph.vertex_count()];

            // Primeira fase: atribuição aleatória inicial
            (0..graph.vertex_count()).for_each(|i| {
                genes[i] = rng.gen_range(0..=2);
            });

            // Segunda fase: garantir que vértices com valor 0 tenham um vizinho com valor 2
            let mut changed = true;
            while changed {
                changed = false;
                for vertex in 0..graph.vertex_count() {
                    if genes[vertex] == 0 {
                        if let Ok(neighbors) = graph.neighbors(vertex) {
                            if neighbors.is_empty() {
                                // Vértice isolado não pode ter valor 0
                                genes[vertex] = 2;
                                changed = true;
                                continue;
                            }

                            if !neighbors.iter().any(|&v| genes[v] == 2) {
                                // Se não tem vizinho com valor 2, escolhe um aleatório para ser 2
                                if let Some(&random_neighbor) = neighbors.iter().choose(&mut rng) {
                                    genes[random_neighbor] = 2;
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }

            // Terceira fase: garantir que vértices com valor 1 tenham um vizinho com valor > 0
            changed = true;
            while changed {
                changed = false;
                for vertex in 0..graph.vertex_count() {
                    if genes[vertex] == 1 {
                        if let Ok(neighbors) = graph.neighbors(vertex) {
                            if neighbors.is_empty() {
                                // Vértice isolado não pode ter valor 1
                                genes[vertex] = 2;
                                changed = true;
                                continue;
                            }

                            if !neighbors.iter().any(|&v| genes[v] > 0) {
                                // Se não tem vizinho com valor > 0, escolhe um aleatório
                                if let Some(&random_neighbor) = neighbors.iter().choose(&mut rng) {
                                    genes[random_neighbor] = 1;
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }

            // Verifica se o cromossomo é válido
            let chromosome = Chromosome::new(genes);
            if chromosome.is_valid_to_total_roman_domination(graph) {
                return chromosome;
            }

            retries += 1;
        }

        let genes = vec![1; graph.vertex_count()];
        Chromosome::new(genes)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn individuals(&self) -> Vec<Chromosome> {
        self.individuals.clone()
    }

    pub fn best_individual(&mut self) -> Result<Chromosome, PopulationError> {
        if self.individuals.is_empty() {
            return Err(PopulationError::PopulationEmpyt);
        }

        let mut best_fitness = usize::MAX;
        let mut best_index = 0;

        for (index, individual) in self.individuals.iter_mut().enumerate() {
            let fitness = individual.fitness();
            if fitness < best_fitness {
                best_fitness = fitness;
                best_index = index;
            }
        }

        Ok(self.individuals[best_index].clone())
    }

    pub fn add_individual(&mut self, individual: Chromosome) {
        self.individuals.push(individual);
        self.size = self.individuals.len();
    }

    pub fn new_from_individuals(individuals: Vec<Chromosome>) -> Population {
        let size = individuals.len();
        Self { individuals, size }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> SimpleGraph {
        let mut graph = SimpleGraph::new();
        for i in 0..5 {
            graph.add_vertex(i).unwrap();
        }
        graph.add_edge(0, 1).unwrap();
        graph.add_edge(1, 2).unwrap();
        graph.add_edge(2, 3).unwrap();
        graph.add_edge(3, 4).unwrap();
        graph.add_edge(4, 0).unwrap();
        graph
    }

    fn create_small_test_graph() -> SimpleGraph {
        let mut graph = SimpleGraph::new();
        graph.add_vertex(0).unwrap();
        graph.add_vertex(1).unwrap();
        graph.add_vertex(2).unwrap();
        graph.add_edge(0, 1).unwrap();
        graph.add_edge(1, 2).unwrap();
        graph
    }

    fn heuristic_one(_graph: &SimpleGraph) -> Option<Chromosome> {
        let genes = vec![2, 0, 1, 2, 0];
        Some(Chromosome::new(genes))
    }

    fn heuristic_two(_graph: &SimpleGraph) -> Option<Chromosome> {
        let genes = vec![1, 1, 1, 1, 1];
        Some(Chromosome::new(genes))
    }

    fn failing_heuristic(_graph: &SimpleGraph) -> Option<Chromosome> {
        None
    }

    #[test]
    fn test_population_creation_with_valid_inputs() {
        let graph = create_test_graph();
        let heuristics: Vec<fn(&SimpleGraph) -> Option<Chromosome>> =
            vec![heuristic_one, heuristic_two];
        let population_size = 5;

        let population = Population::new(&graph, heuristics, population_size);

        assert!(population.is_ok());
        let population = population.unwrap();
        assert_eq!(population.size(), population_size);
        assert_eq!(population.individuals().len(), population_size);
    }

    #[test]
    fn test_population_creation_with_invalid_size() {
        let graph = create_test_graph();
        let heuristics: Vec<fn(&SimpleGraph) -> Option<Chromosome>> =
            vec![heuristic_one, heuristic_two];
        let population_size = 2; // Less than minimum required (heuristics.len() + 1)

        let result = Population::new(&graph, heuristics, population_size);
        assert!(matches!(
            result,
            Err(PopulationError::InvalidPopulationSize)
        ));
    }

    #[test]
    fn test_population_creation_with_isolated_vertices() {
        let mut graph = SimpleGraph::new();
        graph.add_vertex(0).unwrap();
        graph.add_vertex(1).unwrap(); // Isolated vertices

        let heuristics: Vec<fn(&SimpleGraph) -> Option<Chromosome>> = vec![heuristic_one];
        let population_size = 3;

        let result = Population::new(&graph, heuristics, population_size);
        assert!(matches!(
            result,
            Err(PopulationError::IsolatedVerticesFound)
        ));
    }

    #[test]
    fn test_population_creation_with_failed_heuristic() {
        let graph = create_test_graph();
        let heuristics: Vec<fn(&SimpleGraph) -> Option<Chromosome>> = vec![failing_heuristic];
        let population_size = 3;

        let result = Population::new(&graph, heuristics, population_size);
        assert!(matches!(result, Err(PopulationError::HeuristicFailed)));
    }

    #[test]
    fn test_population_random_chromosome_generation_and_validity() {
        let graph = create_test_graph();
        let heuristics: Vec<fn(&SimpleGraph) -> Option<Chromosome>> = vec![heuristic_one];
        let population_size = 5;

        let population = Population::new(&graph, heuristics, population_size).unwrap();

        for individual in population.individuals() {
            assert_eq!(individual.genes().len(), graph.vertex_count());
            assert!(individual.is_valid_to_total_roman_domination(&graph));
        }
    }

    #[test]
    fn test_best_individual_selection() {
        let graph = create_test_graph();
        let heuristics: Vec<fn(&SimpleGraph) -> Option<Chromosome>> = vec![
            |_| Some(Chromosome::new(vec![2, 2, 2, 2, 2])), // fitness = 10
            |_| Some(Chromosome::new(vec![1, 1, 1, 1, 1])), // fitness = 5
            |_| Some(Chromosome::new(vec![2, 0, 2, 0, 2])), // fitness = 6
        ];
        let population_size = 4;

        let mut population = Population::new(&graph, heuristics, population_size).unwrap();
        let mut best = population.best_individual().unwrap();

        assert_eq!(best.fitness(), 5);
    }

    #[test]
    fn test_empty_population_best_individual() {
        let mut population = Population {
            individuals: vec![],
            size: 0,
        };
        assert!(population.best_individual().is_err());
    }

    #[test]
    fn test_population_with_minimum_size() {
        let graph = create_small_test_graph();
        let heuristics: Vec<fn(&SimpleGraph) -> Option<Chromosome>> = vec![heuristic_one];
        let population_size = 2; // Minimum size (1 heuristic + 1)

        let population = Population::new(&graph, heuristics, population_size).unwrap();
        assert_eq!(population.size(), 2);
        assert_eq!(population.individuals().len(), 2);
    }

    #[test]
    fn test_population_individuals_clone() {
        let graph = create_test_graph();
        let heuristics: Vec<fn(&SimpleGraph) -> Option<Chromosome>> = vec![heuristic_one];
        let population_size = 3;

        let population = Population::new(&graph, heuristics, population_size).unwrap();
        let individuals1 = population.individuals();
        let individuals2 = population.individuals();

        assert_eq!(individuals1.len(), individuals2.len());
        for (ind1, ind2) in individuals1.iter().zip(individuals2.iter()) {
            assert_eq!(ind1.genes(), ind2.genes());
        }
    }

    #[test]
    fn test_generate_random_valid_chromosome() {
        let graph = create_test_graph();
        let chromosome = Population::generate_random_valid_chromosome(&graph);

        assert_eq!(chromosome.genes().len(), graph.vertex_count());
        assert!(chromosome.is_valid_to_total_roman_domination(&graph));

        for gene in chromosome.genes() {
            assert!(gene <= 2);
        }
    }
}
