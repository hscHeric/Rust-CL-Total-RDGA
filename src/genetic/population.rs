use rand::Rng;

use crate::graph::SimpleGraph;

use super::Chromosome;

#[derive(Debug, Clone, Default)]
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

    pub fn generate_random_valid_chromosome(graph: &SimpleGraph) -> Chromosome {
        let mut rng = rand::thread_rng();
        let vertex_count = graph.vertex_count();
        let max_attempts = 100; // Limite de tentativas

        for _ in 0..max_attempts {
            let genes: Vec<u8> = (0..vertex_count).map(|_| rng.gen_range(0..=2)).collect();
            let chromosome = Chromosome::new(genes).fix_chromosome(graph);

            if chromosome.is_valid_to_total_roman_domination(graph) {
                return chromosome;
            }
        }

        // Fallback: tenta gerar um cromossomo válido com todos os vértices dominados
        let fallback_genes = vec![2; vertex_count];
        let fallback_chromosome = Chromosome::new(fallback_genes);

        fallback_chromosome.fix_chromosome(graph)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn individuals(&self) -> &[Chromosome] {
        &self.individuals
    }

    pub fn add_individual(&mut self, individual: Chromosome) {
        self.individuals.push(individual);
        self.size = self.individuals.len();
    }

    pub fn best_individual(&self) -> Result<Chromosome, PopulationError> {
        if self.individuals.is_empty() {
            return Err(PopulationError::PopulationEmpyt);
        }
        let (best_index, _best_fitness) = self
            .individuals
            .iter()
            .enumerate()
            .map(|(index, individual)| (index, individual.fitness()))
            .min_by_key(|&(_, fitness)| fitness)
            .unwrap();

        Ok(self.individuals[best_index].clone())
    }

    pub fn validate_population(&self, graph: &SimpleGraph) -> Population {
        let validated_individuals: Vec<Chromosome> = self
            .individuals
            .iter()
            .map(|individual| {
                if individual.is_valid_to_total_roman_domination(graph) {
                    individual.clone()
                } else {
                    individual.fix_chromosome(graph)
                }
            })
            .collect();

        Population::new_from_individuals(validated_individuals)
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
        let genes = vec![2, 0, 1, 2, 1];
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
        let population_size = 2;

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
        graph.add_vertex(1).unwrap();

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

        for (index, individual) in population.individuals().iter().enumerate() {
            assert!(
                individual.is_valid_to_total_roman_domination(&graph),
                "Individual {} é inválido após fix_chromosome: {:?}",
                index,
                individual.genes()
            );
        }
    }

    #[test]
    fn test_best_individual_selection() {
        let graph = create_test_graph();
        let heuristics: Vec<fn(&SimpleGraph) -> Option<Chromosome>> = vec![
            |_| Some(Chromosome::new(vec![2, 2, 2, 2, 2])),
            |_| Some(Chromosome::new(vec![1, 1, 1, 1, 1])),
            |_| Some(Chromosome::new(vec![2, 0, 2, 0, 2])),
        ];
        let population_size = 4;

        let population = Population::new(&graph, heuristics, population_size).unwrap();
        let best = population.best_individual().unwrap();

        assert_eq!(best.fitness(), 5);
    }

    #[test]
    fn test_empty_population_best_individual() {
        let population = Population {
            individuals: vec![],
            size: 0,
        };
        assert!(population.best_individual().is_err());
    }

    #[test]
    fn test_population_with_minimum_size() {
        let graph = create_small_test_graph();
        let heuristics: Vec<fn(&SimpleGraph) -> Option<Chromosome>> = vec![heuristic_one];
        let population_size = 2;

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

        for &gene in chromosome.genes() {
            assert!(gene <= 2);
        }
    }
}
