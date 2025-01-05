use std::process::exit;
use std::{env, time::Instant};

use cl_total_rdga::graph::parser::normalize_edges;
use cl_total_rdga::{
    genetic::{
        h1, heuristics::h0, Chromosome, CrossoverStrategy, KTournamentSelection, Population,
        SelectionStrategy, TwoPointCrossover,
    },
    graph::{parser::from_edge_list_file, SimpleGraph},
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "Uso: {} <file_path> <trials> [max_stagnant] [generations] [tournament_size] [crossover_prob] [pop_size]",
            args[0]
        );
        exit(1);
    }

    // Parse dos argumentos obrigatórios
    let file_path = &args[1];
    let trials: usize = match args[2].parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Erro: 'trials' deve ser um número inteiro válido.");
            exit(1);
        }
    };

    // Criação do grafo
    let edge_list = from_edge_list_file(file_path);
    let edge_list = normalize_edges(edge_list);
    let graph = match SimpleGraph::from_edges(edge_list) {
        Ok(graph) => graph,
        Err(err) => {
            eprintln!("Erro ao criar o grafo: {:?}", err);
            exit(1);
        }
    };

    // Parse dos valores opcionais ( O padrão para os valores opcionais vai ser o mesmo do CL-RD)
    let max_stagnant = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(100);
    let generations = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(1000);
    let tournament_size = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(5);
    let crossover_rate = args.get(6).and_then(|s| s.parse().ok()).unwrap_or(0.9);
    let pop_size = args
        .get(7)
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| ((graph.vertex_count() as f64 / 1.5).ceil() as usize).max(1));

    let heuristics: Vec<fn(&SimpleGraph) -> Option<Chromosome>> = vec![h1, h0];
    let selection_strategy = KTournamentSelection { tournament_size };
    let crossover_strategy = TwoPointCrossover { crossover_rate };

    println!("graph_name,graph_order,graph_size,fitness_value,elapsed_time(microsecond)");
    (0..trials).into_par_iter().for_each(|trial| {
        let start_time = Instant::now();
        let mut population = Population::new(&graph, heuristics.clone(), pop_size)
            .expect("Erro ao criar a população inicial");

        let mut best_solution = population
            .best_individual()
            .expect("Erro ao obter o melhor indivíduo inicial");

        let mut stagnant_generations = 0;
        for generation in 0..generations {
            let selected_population = selection_strategy.select(&population);
            let offspring_population = crossover_strategy.crossover(&selected_population, &graph);
            population = offspring_population.validate_population(&graph);

            let new_best_solution = population
                .best_individual()
                .expect("Erro ao obter o melhor indivíduo");

            if new_best_solution.fitness() < best_solution.fitness() {
                best_solution = new_best_solution;
                stagnant_generations = 0;
            } else {
                stagnant_generations += 1;
            }

            if stagnant_generations >= max_stagnant {
                break;
            }
        }

        let elapsed_time = start_time.elapsed();
        let graph_name = file_path.split('/').last().unwrap_or("unknown");

        println!(
            "{},{},{},{},{}",
            graph_name,
            graph.vertex_count(),
            graph.edge_count(),
            best_solution.fitness(),
            elapsed_time.as_micros()
        );
    });
}
