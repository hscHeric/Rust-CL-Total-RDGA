use kambo_graph::graphs::simple::UndirectedGraph;
use rand::{seq::SliceRandom, Rng};

use super::{Chromosome, Population};

/// Representa uma estratégia de cruzamento genético.
///
/// O cruzamento genético combina cromossomos de uma população para gerar novos indivíduos
/// com características herdadas dos pais.
pub trait CrossoverStrategy {
    /// Aplica a estratégia de cruzamento a uma população.
    ///
    /// # Parâmetros
    /// - `population`: A população original.
    /// - `graph`: O grafo usado para validar os indivíduos gerados.
    ///
    /// # Retorno
    /// Retorna uma nova `Population` contendo os indivíduos resultantes do cruzamento.
    fn crossover(&self, population: &Population, graph: &UndirectedGraph<usize>) -> Population;
}

/// Estratégia de cruzamento de dois pontos.
///
/// Realiza cruzamentos entre dois pais selecionados, trocando segmentos de genes entre
/// dois pontos aleatórios.
pub struct TwoPointCrossover {
    /// Taxa de cruzamento, um valor entre 0.0 e 1.0 que define a probabilidade de cruzamento.
    pub crossover_rate: f64,
}

impl CrossoverStrategy for TwoPointCrossover {
    fn crossover(&self, population: &Population, graph: &UndirectedGraph<usize>) -> Population {
        if self.crossover_rate == 0.0 {
            return population.clone();
        }

        let mut rng = rand::thread_rng();
        let mut new_individuals = Vec::with_capacity(population.size());
        let shuffled_individuals = population.individuals();

        let mut shuffled_individuals = shuffled_individuals.to_vec();
        shuffled_individuals.shuffle(&mut rng);

        for pair in shuffled_individuals.chunks(2) {
            if pair.len() == 2 {
                let (parent_a, parent_b) = (&pair[0], &pair[1]);

                if rng.gen_bool(self.crossover_rate) {
                    let (child_a, child_b) = two_point_crossover(parent_a, parent_b);
                    new_individuals.push(child_a);
                    new_individuals.push(child_b);
                } else {
                    new_individuals.push(parent_a.clone());
                    new_individuals.push(parent_b.clone());
                }
            } else {
                new_individuals.push(pair[0].clone());
            }
        }

        Population::new_from_individuals(new_individuals).validate_population(graph)
    }
}

/// Realiza o cruzamento de dois pontos entre dois cromossomos.
///
/// # Parâmetros
/// - `parent_a`: O primeiro cromossomo pai.
/// - `parent_b`: O segundo cromossomo pai.
///
/// # Retorno
/// Retorna uma tupla contendo os dois cromossomos filhos resultantes do cruzamento.
fn two_point_crossover(parent_a: &Chromosome, parent_b: &Chromosome) -> (Chromosome, Chromosome) {
    let mut rng = rand::thread_rng();
    let len = parent_a.genes().len();

    let mut points = [rng.gen_range(0..len), rng.gen_range(0..len)];
    points.sort();

    let (start, end) = (points[0], points[1]);

    let childa_genes = parent_a.genes()[..start]
        .iter()
        .chain(parent_b.genes()[start..end].iter())
        .chain(parent_a.genes()[end..].iter())
        .cloned()
        .collect();

    let childb_genes = parent_b.genes()[..start]
        .iter()
        .chain(parent_a.genes()[start..end].iter())
        .chain(parent_b.genes()[end..].iter())
        .cloned()
        .collect();

    (Chromosome::new(childa_genes), Chromosome::new(childb_genes))
}

/// Estratégia de cruzamento de um ponto.
///
/// Realiza cruzamentos entre dois pais selecionados, trocando os genes após um único
/// ponto aleatório.
pub struct OnePointCrossover {
    pub crossover_rate: f64,
}

/// Realiza o cruzamento de um ponto entre dois cromossomos.
///
/// # Parâmetros
/// - `parent_a`: O primeiro cromossomo pai.
/// - `parent_b`: O segundo cromossomo pai.
///
/// # Retorno
/// Retorna uma tupla contendo os dois cromossomos filhos resultantes do cruzamento.
fn one_point_crossover(parent_a: &Chromosome, parent_b: &Chromosome) -> (Chromosome, Chromosome) {
    let mut rng = rand::thread_rng();
    let len = parent_a.genes().len();
    let cut_point = rng.gen_range(0..len);

    let mut offspring_a_genes = Vec::with_capacity(len);
    let mut offspring_b_genes = Vec::with_capacity(len);

    // Usando extend_from_slice para copiar os genes de forma mais eficiente
    offspring_a_genes.extend_from_slice(&parent_a.genes()[..cut_point]);
    offspring_a_genes.extend_from_slice(&parent_b.genes()[cut_point..]);

    offspring_b_genes.extend_from_slice(&parent_b.genes()[..cut_point]);
    offspring_b_genes.extend_from_slice(&parent_a.genes()[cut_point..]);

    (
        Chromosome::new(offspring_a_genes),
        Chromosome::new(offspring_b_genes),
    )
}

impl CrossoverStrategy for OnePointCrossover {
    fn crossover(&self, population: &Population, graph: &UndirectedGraph<usize>) -> Population {
        if self.crossover_rate == 0.0 {
            return population.clone();
        }

        let mut rng = rand::thread_rng();
        let population_size = population.size();
        let mut new_individuals = Vec::with_capacity(population_size);

        // Criamos um vetor de índices e embaralhamos ele em vez dos indivíduos
        let mut indices: Vec<usize> = (0..population_size).collect();
        indices.shuffle(&mut rng);

        // Processamos os pares de índices
        for chunk in indices.chunks(2) {
            if chunk.len() == 2 {
                let (idx_a, idx_b) = (chunk[0], chunk[1]);
                let parent_a = &population.individuals()[idx_a];
                let parent_b = &population.individuals()[idx_b];

                if rng.gen_bool(self.crossover_rate) {
                    let (child_a, child_b) = one_point_crossover(parent_a, parent_b);
                    new_individuals.push(child_a);
                    new_individuals.push(child_b);
                } else {
                    new_individuals.push(parent_a.clone());
                    new_individuals.push(parent_b.clone());
                }
            } else {
                // Caso ímpar, apenas adiciona o último indivíduo
                new_individuals.push(population.individuals()[chunk[0]].clone());
            }
        }

        Population::new_from_individuals(new_individuals).validate_population(graph)
    }
}

#[cfg(test)]
mod tests {
    use kambo_graph::GraphMut;

    use super::*;

    fn create_test_graph() -> UndirectedGraph<usize> {
        let mut graph = UndirectedGraph::<usize>::new_undirected();
        graph.add_vertex(0).unwrap();
        graph.add_vertex(1).unwrap();
        graph.add_vertex(2).unwrap();
        graph.add_vertex(3).unwrap();

        let _ = graph.add_edge(&0, &1);
        let _ = graph.add_edge(&1, &2);
        let _ = graph.add_edge(&2, &3);
        graph
    }

    #[test]
    fn test_two_point_crossover_basic() {
        let parent_a = Chromosome::new(vec![2, 1, 1, 2]);
        let parent_b = Chromosome::new(vec![1, 2, 2, 1]);

        for _ in 0..100 {
            let (child_a, child_b) = two_point_crossover(&parent_a, &parent_b);

            assert_eq!(child_a.genes().len(), 4);
            assert_eq!(child_b.genes().len(), 4);

            for &gene in child_a.genes() {
                assert!(gene <= 2, "Invalid gene in child_a: {}", gene);
            }
            for &gene in child_b.genes() {
                assert!(gene <= 2, "Invalid gene in child_b: {}", gene);
            }
        }
    }

    #[test]
    fn test_crossover_strategy_with_validation() {
        let graph = create_test_graph();

        let individuals = vec![
            Chromosome::new(vec![2, 1, 1, 2]),
            Chromosome::new(vec![1, 2, 2, 1]),
            Chromosome::new(vec![2, 2, 1, 1]),
            Chromosome::new(vec![1, 1, 2, 2]),
        ];
        let initial_population = Population::new_from_individuals(individuals);

        let strategy = TwoPointCrossover {
            crossover_rate: 1.0,
        };

        let new_population = strategy.crossover(&initial_population, &graph);

        assert_eq!(new_population.size(), initial_population.size());

        for individual in new_population.individuals() {
            assert!(
                individual.is_valid_to_total_roman_domination(&graph),
                "Invalid chromosome after crossover: {:?}",
                individual.genes()
            );
        }
    }

    #[test]
    fn test_crossover_rate_zero() {
        let graph = create_test_graph();

        let individuals = vec![
            Chromosome::new(vec![2, 1, 1, 2]),
            Chromosome::new(vec![1, 2, 2, 1]),
        ];
        let initial_population = Population::new_from_individuals(individuals);

        let strategy = TwoPointCrossover {
            crossover_rate: 0.0,
        };

        let new_population = strategy.crossover(&initial_population, &graph);

        let new_individuals = new_population.individuals();
        let initial_individuals = initial_population.individuals();

        for (idx, (new_ind, old_ind)) in new_individuals
            .iter()
            .zip(initial_individuals.iter())
            .enumerate()
        {
            assert_eq!(
                new_ind.genes(),
                old_ind.genes(),
                "Different genes at index {}: Original {:?}, New {:?}",
                idx,
                old_ind.genes(),
                new_ind.genes()
            );
        }
    }

    #[test]
    fn test_crossover_maintains_fitness() {
        let graph = create_test_graph();

        let parent_a = Chromosome::new(vec![2, 1, 1, 2]);
        let parent_b = Chromosome::new(vec![1, 2, 2, 1]);

        let (child_a, child_b) = two_point_crossover(&parent_a, &parent_b);

        assert_eq!(
            child_a.fitness(),
            child_a.genes().iter().map(|&x| x as usize).sum::<usize>()
        );
        assert_eq!(
            child_b.fitness(),
            child_b.genes().iter().map(|&x| x as usize).sum::<usize>()
        );

        let valid_child_a = child_a.fix_chromosome(&graph);
        let valid_child_b = child_b.fix_chromosome(&graph);

        assert!(
            valid_child_a.is_valid_to_total_roman_domination(&graph),
            "Child A invalid after fix: {:?}",
            valid_child_a.genes()
        );
        assert!(
            valid_child_b.is_valid_to_total_roman_domination(&graph),
            "Child B invalid after fix: {:?}",
            valid_child_b.genes()
        );
    }

    #[test]
    fn test_population_size_after_crossover() {
        let graph = create_test_graph();

        let individuals = vec![
            Chromosome::new(vec![2, 1, 1, 2]),
            Chromosome::new(vec![1, 2, 2, 1]),
            Chromosome::new(vec![2, 2, 1, 1]),
            Chromosome::new(vec![1, 1, 2, 2]),
        ];
        let initial_population = Population::new_from_individuals(individuals);

        let strategy = TwoPointCrossover {
            crossover_rate: 0.5,
        };

        let new_population = strategy.crossover(&initial_population, &graph);

        assert_eq!(
            new_population.size(),
            initial_population.size(),
            "The population size after crossover should remain the same"
        );
    }
}
