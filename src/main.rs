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
struct AlgorithmParams {
    max_stagnant: usize,
    generations: usize,
    tournament_size: usize,
    crossover_rate: f64,
    population_factor: f64,
    file_path: String,
    trials: usize,
    output_file: String,
}

#[derive(Debug)]
struct TrialResult {
    graph_name: String,
    node_count: usize,
    edge_count: usize,
    fitness: usize,
    elapsed_micros: u128,
}

impl Default for AlgorithmParams {
    fn default() -> Self {
        Self {
            max_stagnant: 100,
            generations: 1000,
            tournament_size: 5,
            crossover_rate: 0.9,
            population_factor: 1.5,
            file_path: String::new(),
            trials: 1,
            output_file: String::from("results.csv"),
        }
    }
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

fn parse_args() -> Result<AlgorithmParams, String> {
    let mut params = AlgorithmParams::default();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Usage: ./cl-total-rdga <graph_file> [options]\n\
            Options:\n\
            --crossover VALUE\n\
            --stagnation VALUE\n\
            --generations VALUE\n\
            --population VALUE\n\
            --tournament VALUE\n\
            --trials VALUE\n\
            --output FILE"
            .to_string());
    }

    params.file_path = args[1].clone();

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--crossover" => {
                if i + 1 < args.len() {
                    params.crossover_rate = args[i + 1]
                        .parse()
                        .map_err(|_| format!("Invalid crossover value: {}", args[i + 1]))?;
                    i += 2;
                } else {
                    return Err("Missing value for --crossover".to_string());
                }
            }
            "--stagnation" => {
                if i + 1 < args.len() {
                    params.max_stagnant = args[i + 1]
                        .parse()
                        .map_err(|_| format!("Invalid stagnation value: {}", args[i + 1]))?;
                    i += 2;
                } else {
                    return Err("Missing value for --stagnation".to_string());
                }
            }
            "--generations" => {
                if i + 1 < args.len() {
                    params.generations = args[i + 1]
                        .parse()
                        .map_err(|_| format!("Invalid generations value: {}", args[i + 1]))?;
                    i += 2;
                } else {
                    return Err("Missing value for --generations".to_string());
                }
            }
            "--population" => {
                if i + 1 < args.len() {
                    params.population_factor = args[i + 1]
                        .parse()
                        .map_err(|_| format!("Invalid population value: {}", args[i + 1]))?;
                    i += 2;
                } else {
                    return Err("Missing value for --population".to_string());
                }
            }
            "--tournament" => {
                if i + 1 < args.len() {
                    params.tournament_size = args[i + 1]
                        .parse()
                        .map_err(|_| format!("Invalid tournament value: {}", args[i + 1]))?;
                    i += 2;
                } else {
                    return Err("Missing value for --tournament".to_string());
                }
            }
            "--trials" => {
                if i + 1 < args.len() {
                    params.trials = args[i + 1]
                        .parse()
                        .map_err(|_| format!("Invalid trials value: {}", args[i + 1]))?;
                    i += 2;
                } else {
                    return Err("Missing value for --trials".to_string());
                }
            }
            "--output" => {
                if i + 1 < args.len() {
                    params.output_file = args[i + 1].clone();
                    i += 2;
                } else {
                    return Err("Missing value for --output".to_string());
                }
            }
            _ => return Err(format!("Unknown argument: {}", args[i])),
        }
    }

    Ok(params)
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

    let params = match parse_args() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    info!("Starting genetic algorithm execution");

    info!("Building graph from file: {}", params.file_path);
    let graph = build_graph(&params.file_path);

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

    let pop_size = (graph.order() as f64 / params.population_factor).round() as usize;

    debug!("Using population size: {}", pop_size);

    let heuristics: Vec<Heuristic> = vec![h1, h2, h3, h4, h5, h1];
    let crossover = SinglePoint::new(params.crossover_rate);
    let selector = KTournament::new(params.tournament_size);

    info!("Starting {} trials", params.trials);
    let results = Mutex::new(Vec::with_capacity(params.trials));

    let start_time = Instant::now();
    (0..params.trials).into_par_iter().for_each(|trial| {
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
        let graph_name = params
            .file_path
            .split('/')
            .last()
            .unwrap_or("unknown")
            .to_string();

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
    if let Err(e) = write_results_to_csv(&results, &params.output_file) {
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
