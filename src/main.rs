use std::{
    env,
    fs::OpenOptions,
    io::{self, Write},
    process::exit,
    sync::Mutex,
    time::Instant,
};

use cl_total_rdga::{
    genetic::{h1, h2, h3, h4, h5, Heuristic, KTournament, Population, SinglePoint},
    utils::build_graph,
};
use env_logger::{Builder, Target};
use kambo_graph::Graph;
use log::{debug, error, info, LevelFilter};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug)]
struct TrialResult {
    graph_name: String,
    node_count: usize,
    edge_count: usize,
    fitness: usize,
    elapsed_micros: u128,
}

#[derive(Debug)]
struct AlgorithmParams {
    max_stagnant: usize,
    generations: usize,
    tournament_size: usize,
    crossover_rate: f64,
    pop_size: usize,
}

fn setup_logger() -> Result<(), io::Error> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("execution.log")?;

    Builder::new()
        .target(Target::Pipe(Box::new(file)))
        .filter_level(LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();

    Ok(())
}

fn parse_args() -> Result<(String, usize, String, AlgorithmParams), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        return Err(format!(
            "Usage: {} <file_path> <trials> <output_file> [max_stagnant] [generations] [tournament_size] [crossover_prob] [pop_size]",
            args[0]
        ));
    }

    let file_path = args[1].clone();
    let trials = args[2].parse().map_err(|_| "Invalid trials parameter")?;
    let output_file = args[3].clone();

    let params = AlgorithmParams {
        max_stagnant: args.get(4).and_then(|s| s.parse().ok()).unwrap_or(100),
        generations: args.get(5).and_then(|s| s.parse().ok()).unwrap_or(1000),
        tournament_size: args.get(6).and_then(|s| s.parse().ok()).unwrap_or(5),
        crossover_rate: args.get(7).and_then(|s| s.parse().ok()).unwrap_or(0.9),
        pop_size: args.get(8).and_then(|s| s.parse().ok()).unwrap_or(50),
    };

    info!(
        "Parsed arguments - File: {}, Trials: {}, Output: {}",
        file_path, trials, output_file
    );
    debug!("Algorithm parameters: {:?}", params);

    Ok((file_path, trials, output_file, params))
}

fn write_results_to_csv(results: &[TrialResult], output_file: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_file)
        .map_err(|e| {
            error!("Failed to open output file: {}", e);
            e
        })?;

    if file.metadata()?.len() == 0 {
        debug!("Creating new CSV file with header");
        writeln!(
            file,
            "graph_name,graph_order,graph_size,fitness_value,elapsed_time(microsecond)"
        )?;
    }

    for result in results {
        debug!("Writing result: {:?}", result);
        writeln!(
            file,
            "{},{},{},{},{}",
            result.graph_name,
            result.node_count,
            result.edge_count,
            result.fitness,
            result.elapsed_micros
        )?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = setup_logger() {
        eprintln!("Failed to setup logger: {}", e);
        exit(1);
    }

    info!("Starting genetic algorithm execution");

    let (file_path, trials, output_file, params) = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            error!("Error parsing arguments: {}", e);
            eprintln!("Error: {}", e);
            exit(1);
        }
    };

    let start_time = Instant::now();
    info!("Building graph from file: {}", file_path);
    let graph = build_graph(&file_path);

    if graph.order() == 0 {
        error!("Graph has no nodes");
        eprintln!("The graph has no nodes. Exiting.");
        exit(1);
    }

    info!(
        "Graph loaded - Nodes: {}, Edges: {}",
        graph.order(),
        graph.edge_count()
    );

    let pop_size = if params.pop_size == 0 {
        ((graph.order() as f64 / 1.5).ceil() as usize).max(1)
    } else {
        params.pop_size
    };

    debug!("Using population size: {}", pop_size);

    let heuristics: Vec<Heuristic> = vec![h1, h2, h3, h4, h5, h1];
    let crossover = SinglePoint::new(params.crossover_rate);
    let selector = KTournament::new(params.tournament_size);

    info!("Starting {} trials", trials);
    let results = Mutex::new(Vec::with_capacity(trials));

    (0..trials).into_par_iter().for_each(|trial| {
        info!("Starting trial {}", trial + 1);
        let trial_start = Instant::now();

        let mut population = Population::new(pop_size, &heuristics, &graph);
        debug!("Initial population created for trial {}", trial + 1);

        let mut best_solution = population
            .best_chromosome()
            .expect("Failed to retrieve the best individual")
            .clone();

        debug!("Initial best fitness: {}", best_solution.fitness());

        let mut stagnant_generations = 0;
        for generation in 0..params.generations {
            population.envolve(&selector, &crossover, &graph);
            let new_best_solution = population
                .best_chromosome()
                .expect("Failed to retrieve the best individual")
                .clone();

            if new_best_solution.fitness() < best_solution.fitness() {
                debug!(
                    "Trial {} - Generation {} - New best fitness: {} (improved from {})",
                    trial + 1,
                    generation + 1,
                    new_best_solution.fitness(),
                    best_solution.fitness()
                );
                best_solution = new_best_solution;
                stagnant_generations = 0;
            } else {
                stagnant_generations += 1;
            }

            if stagnant_generations >= params.max_stagnant {
                info!(
                    "Trial {} stopped at generation {} due to stagnation",
                    trial + 1,
                    generation + 1
                );
                break;
            }
        }

        let elapsed_time = trial_start.elapsed();
        let graph_name = file_path.split('/').last().unwrap_or("unknown").to_string();

        info!(
            "Trial {} completed - Final fitness: {}, Time: {:?}",
            trial + 1,
            best_solution.fitness(),
            elapsed_time
        );

        results.lock().unwrap().push(TrialResult {
            graph_name,
            node_count: graph.order(),
            edge_count: graph.edge_count(),
            fitness: best_solution.fitness(),
            elapsed_micros: elapsed_time.as_micros(),
        });
    });

    let results = results.into_inner().unwrap();
    if let Err(e) = write_results_to_csv(&results, &output_file) {
        error!("Failed to write results: {}", e);
        eprintln!("Failed to write results to file: {}", e);
        exit(1);
    }

    let total_time = start_time.elapsed();
    info!(
        "Execution completed in {:.2} seconds",
        total_time.as_secs_f64()
    );
    println!(
        "Execution completed in {:.2} seconds.",
        total_time.as_secs_f64()
    );
}
