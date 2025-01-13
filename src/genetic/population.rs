use kambo_graph::graphs::simple::UndirectedGraph;
use kambo_graph::Graph;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::Chromosome;

/// Representa uma população de cromossomos em um algoritmo genético.
#[derive(Debug, Clone, Default)]
pub struct Population {
    individuals: Vec<Chromosome>,
    size: usize,
}

/// Define os possíveis erros que podem ocorrer ao manipular uma população.
#[derive(Debug)]
pub enum PopulationError {
    /// Não há heurísticas suficientes para gerar cromossomos.
    NotEnoughHeuristics,
    /// O tamanho da população é inválido (menor que o número de heurísticas).
    InvalidPopulationSize,
    /// Uma heurística falhou ao gerar um cromossomo.
    HeuristicFailed,
    /// A população está vazia.
    PopulationEmpyt,
    /// O grafo contém vértices isolados, impossibilitando a criação de cromossomos válidos.
    IsolatedVerticesFound,
}

impl Population {
    /// Cria uma nova população a partir de um grafo e de heurísticas.
    ///
    /// # Parâmetros
    /// - `graph`: O grafo usado para validar os cromossomos gerados.
    /// - `heuristics`: Um vetor de funções heurísticas para criar cromossomos
    ///     (vai gerar uma
    ///     solução para as heuristicas de 0..n-1, a heuristica n preenche a população com as que
    ///     faltarem).
    /// - `size`: O tamanho desejado da população.
    ///
    /// # Retorno
    /// Retorna um `Result` com a população criada ou um erro (`PopulationError`) caso a criação falhe.
    ///
    /// # Erros
    /// - `PopulationError::IsolatedVerticesFound`: Caso o grafo tenha vértices isolados.
    /// - `PopulationError::NotEnoughHeuristics`: Se o vetor de heurísticas estiver vazio.
    /// - `PopulationError::InvalidPopulationSize`: Se o tamanho solicitado for menor que o número de heurísticas.
    /// - `PopulationError::HeuristicFailed`: Se alguma heurística falhar ao criar um cromossomo.
    pub fn new<F>(
        graph: &UndirectedGraph<usize>,
        mut heuristics: Vec<F>,
        size: usize,
    ) -> Result<Self, PopulationError>
    where
        F: Fn(&UndirectedGraph<usize>) -> Option<Chromosome>,
    {
        if graph.has_isolated_vertex() {
            return Err(PopulationError::IsolatedVerticesFound);
        }

        if heuristics.is_empty() {
            return Err(PopulationError::NotEnoughHeuristics);
        }

        let min_size = heuristics.len();
        if size < min_size {
            return Err(PopulationError::InvalidPopulationSize);
        }

        let last_heuristic = heuristics.pop().unwrap();

        let mut individuals: Vec<Chromosome> = Vec::with_capacity(size);

        for heuristic in heuristics {
            match heuristic(graph) {
                Some(chromosome) => individuals.push(chromosome),
                None => return Err(PopulationError::HeuristicFailed),
            }
        }

        while individuals.len() < size {
            match last_heuristic(graph) {
                Some(chromosome) => individuals.push(chromosome),
                None => return Err(PopulationError::HeuristicFailed),
            }
        }

        let size = individuals.len();

        Ok(Self { individuals, size })
    }

    /// Retorna o tamanho da população.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Retorna uma referência imutável aos indivíduos da população.
    pub fn individuals(&self) -> &[Chromosome] {
        &self.individuals
    }

    /// Adiciona um novo indivíduo à população.
    ///
    /// # Parâmetros
    /// - `individual`: O cromossomo a ser adicionado à população.
    pub fn add_individual(&mut self, individual: Chromosome) {
        self.individuals.push(individual);
        self.size = self.individuals.len();
    }

    /// Retorna o melhor indivíduo da população com base no fitness.
    ///
    /// # Retorno
    /// Retorna um `Result` com o melhor cromossomo ou um erro se a população estiver vazia.
    ///
    /// # Erros
    /// - `PopulationError::PopulationEmpyt`: Se a população estiver vazia.
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

    /// Valida a população ajustando os cromossomos inválidos para o dominação romana total.
    ///
    /// # Parâmetros
    /// - `graph`: O grafo usado para validar os cromossomos.
    ///
    /// # Retorno
    /// Retorna uma nova população com cromossomos corrigidos.
    pub fn validate_population(&self, graph: &UndirectedGraph<usize>) -> Population {
        let validated_individuals: Vec<Chromosome> = self
            .individuals
            .par_iter() // Itera sobre os indivíduos em paralelo
            .map(|individual| {
                if individual.is_valid_to_total_roman_domination(graph) {
                    // Se válido, move diretamente (sem clonagem)
                    individual.clone()
                } else {
                    // Corrige o cromossomo e retorna a versão corrigida
                    individual.fix_chromosome(graph)
                }
            })
            .collect();

        // Cria a nova população a partir dos indivíduos validados
        Population::new_from_individuals(validated_individuals)
    }

    /// Cria uma nova população a partir de um vetor de cromossomos existentes.
    ///
    /// # Parâmetros
    /// - `individuals`: O vetor de cromossomos que formará a nova população.
    ///
    /// # Retorno
    /// Retorna uma nova população com os cromossomos fornecidos.
    pub fn new_from_individuals(individuals: Vec<Chromosome>) -> Population {
        let size = individuals.len();
        Self { individuals, size }
    }
}

#[cfg(test)]
mod tests {
    use kambo_graph::GraphMut;

    use super::*;

    fn create_test_graph() -> UndirectedGraph<usize> {
        let mut graph = UndirectedGraph::<usize>::new_undirected();
        for i in 0..5 {
            graph.add_vertex(i).unwrap();
        }
        graph.add_edge(&0, &1).unwrap();
        graph.add_edge(&1, &2).unwrap();
        graph.add_edge(&2, &3).unwrap();
        graph.add_edge(&3, &4).unwrap();
        graph.add_edge(&4, &0).unwrap();
        graph
    }

    fn create_small_test_graph() -> UndirectedGraph<usize> {
        let mut graph = UndirectedGraph::<usize>::new_undirected();
        graph.add_vertex(0).unwrap();
        graph.add_vertex(1).unwrap();
        graph.add_vertex(2).unwrap();
        graph.add_edge(&0, &1).unwrap();
        graph.add_edge(&1, &2).unwrap();
        graph
    }

    fn heuristic_one(_graph: &UndirectedGraph<usize>) -> Option<Chromosome> {
        let genes = vec![2, 0, 1, 2, 1];
        Some(Chromosome::new(genes))
    }

    fn heuristic_two(_graph: &UndirectedGraph<usize>) -> Option<Chromosome> {
        let genes = vec![1, 1, 1, 1, 1];
        Some(Chromosome::new(genes))
    }

    fn failing_heuristic(_graph: &UndirectedGraph<usize>) -> Option<Chromosome> {
        None
    }

    #[test]
    fn test_population_creation_with_valid_inputs() {
        let graph = create_test_graph();
        let heuristics: Vec<fn(&UndirectedGraph<usize>) -> Option<Chromosome>> =
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
        let heuristics: Vec<fn(&UndirectedGraph<usize>) -> Option<Chromosome>> =
            vec![heuristic_one, heuristic_two];
        let population_size = 1;

        let result = Population::new(&graph, heuristics, population_size);
        assert!(matches!(
            result,
            Err(PopulationError::InvalidPopulationSize)
        ));
    }

    #[test]
    fn test_population_creation_with_isolated_vertices() {
        let mut graph = UndirectedGraph::<usize>::new_undirected();
        graph.add_vertex(0).unwrap();
        graph.add_vertex(1).unwrap();

        let heuristics: Vec<fn(&UndirectedGraph<usize>) -> Option<Chromosome>> =
            vec![heuristic_one];
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
        let heuristics: Vec<fn(&UndirectedGraph<usize>) -> Option<Chromosome>> =
            vec![failing_heuristic];
        let population_size = 3;

        let result = Population::new(&graph, heuristics, population_size);
        assert!(matches!(result, Err(PopulationError::HeuristicFailed)));
    }

    #[test]
    fn test_population_random_chromosome_generation_and_validity() {
        let graph = create_test_graph();
        let heuristics: Vec<fn(&UndirectedGraph<usize>) -> Option<Chromosome>> =
            vec![heuristic_one];
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
        let heuristics: Vec<fn(&UndirectedGraph<usize>) -> Option<Chromosome>> = vec![
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
        let heuristics: Vec<fn(&UndirectedGraph<usize>) -> Option<Chromosome>> =
            vec![heuristic_one];
        let population_size = 2;

        let population = Population::new(&graph, heuristics, population_size).unwrap();
        assert_eq!(population.size(), 2);
        assert_eq!(population.individuals().len(), 2);
    }

    #[test]
    fn test_population_individuals_clone() {
        let graph = create_test_graph();
        let heuristics: Vec<fn(&UndirectedGraph<usize>) -> Option<Chromosome>> =
            vec![heuristic_one];
        let population_size = 3;

        let population = Population::new(&graph, heuristics, population_size).unwrap();
        let individuals1 = population.individuals();
        let individuals2 = population.individuals();

        assert_eq!(individuals1.len(), individuals2.len());
        for (ind1, ind2) in individuals1.iter().zip(individuals2.iter()) {
            assert_eq!(ind1.genes(), ind2.genes());
        }
    }
}
