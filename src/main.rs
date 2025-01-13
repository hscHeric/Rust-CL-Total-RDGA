use std::{env, process::exit, sync::Mutex, time::Instant};

use cl_total_rdga::{
    genetic::{
        crossover::OnePointCrossover, h1, h2, h3, h4, h5, CrossoverStrategy, Heuristic,
        KTournamentSelection, Population, SelectionStrategy,
    },
    utils::{build_graph_from_edges, normalize_graph},
};
use kambo_graph::{graphs::simple::UndirectedGraph, utils::edge_list::parse_edge_list, Graph};
use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    ThreadPoolBuilder,
};

pub fn main() {
    let num_cpus = num_cpus::get();
    ThreadPoolBuilder::new()
        .num_threads(num_cpus)
        .build_global()
        .expect("Falha ao configurar o pool de threads do Rayon");

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
    let edges: Vec<(usize, usize, Option<i32>)> = parse_edge_list(file_path).unwrap_or_default();
    let graph: UndirectedGraph<usize> =
        build_graph_from_edges(edges).unwrap_or(UndirectedGraph::new_directed());
    let graph = normalize_graph(&graph);

    // Parse dos valores opcionais ( O padrão para os valores opcionais vai ser o mesmo do CL-RD)
    let max_stagnant = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(100);
    let generations = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(1000);
    let tournament_size = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(5);
    let crossover_rate = args.get(6).and_then(|s| s.parse().ok()).unwrap_or(0.9);
    let pop_size = args
        .get(7)
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| ((graph.order() as f64 / 1.5).ceil() as usize).max(1));

    let heuristics: Vec<Heuristic> = vec![h1, h2, h3, h4, h5, h1];
    let crossover_strategy = OnePointCrossover { crossover_rate };
    let selection_strategy = KTournamentSelection { tournament_size };

    let results = Mutex::new(Vec::new());
    (0..trials).into_par_iter().for_each(|_| {
        let start_time = Instant::now();
        let mut population = Population::new(&graph, heuristics.clone(), pop_size)
            .expect("Erro ao criar a população inicial");

        let mut best_solution = population
            .best_individual()
            .expect("Erro ao obter o melhor indivíduo inicial");

        let mut stagnant_generations = 0;
        for _ in 0..generations {
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

        let result = format!(
            "{},{},{},{},{}",
            graph_name,
            graph.order(),
            graph.edge_count(),
            best_solution.fitness(),
            (elapsed_time.as_secs_f64() / 60.0)
        );
        results.lock().unwrap().push(result);
    });

    println!("graph_name,graph_order,graph_size,fitness_value,elapsed_time(microsecond)");
    let results = results.into_inner().unwrap();
    for result in results {
        println!("{}", result);
    }
}
