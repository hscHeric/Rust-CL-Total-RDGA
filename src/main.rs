use std::time::Instant;

use cl_total_rdga::{
    genetic::{self, h1, CrossoverStrategy, SelectionStrategy},
    graph,
};
use genetic::{Chromosome, KTournamentSelection, Population, TwoPointCrossover};
use graph::{SimpleGraph, SimpleGraphGenerator};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let vertex_count = rng.gen_range(0..10);
    let edge_probability = rng.gen();

    let graph_generator = SimpleGraphGenerator::new(vertex_count, edge_probability);
    let edges = match graph_generator.generate() {
        Ok(edges) => edges,
        Err(err) => {
            eprintln!("Erro ao gerar o grafo: {}", err);
            return;
        }
    };

    let graph = match SimpleGraph::from_edges(edges) {
        Ok(graph) => graph,
        Err(err) => {
            eprintln!("Erro ao criar o grafo: {:?}", err);
            return;
        }
    };

    println!(
        "Grafo gerado com {} vértices e {} arestas.",
        graph.vertex_count(),
        graph.edge_count()
    );

    let population_size = 100;
    let max_generations = 500;
    let max_stagnant_generations = 50;
    let tournament_size = 5;
    let crossover_rate = 0.8;

    let heuristics: Vec<fn(&SimpleGraph) -> Option<Chromosome>> = vec![h1];
    let population = match Population::new(&graph, heuristics, population_size) {
        Ok(population) => population,
        Err(err) => {
            eprintln!("Erro ao criar a população inicial: {:?}", err);
            return;
        }
    };

    for individual in population.individuals() {
        println!("individual_fitness = {}", individual.fitness())
    }

    let selection_strategy = KTournamentSelection { tournament_size };
    let crossover_strategy = TwoPointCrossover { crossover_rate };

    let start_time = Instant::now();

    let mut best_solution = population.best_individual().unwrap();
    for generation in 0..max_generations {
        let selected_population = selection_strategy.select(&population);

        let offspring_population = crossover_strategy.crossover(&selected_population, &graph);

        let new_best_solution = offspring_population.best_individual().unwrap();
        if new_best_solution.fitness() < best_solution.fitness() {
            best_solution = new_best_solution;
        }

        if generation >= max_stagnant_generations {
            break;
        }

        println!(
            "Geração {}: Melhor fitness = {}",
            generation,
            best_solution.fitness(),
        );
    }

    let elapsed_time = start_time.elapsed();
    println!("Tempo total de execução: {:.2?}", elapsed_time);
    println!(
        "Melhor solução encontrada: Fitness = {}",
        best_solution.fitness(),
    );
}
