use chrono::Local;
use cl_total_rdga::{
    genetic::{
        crossover::SinglePoint, h1, h2, h3, h4, h5, heuristics::Heuristic, selection::KTournament,
        Population,
    },
    utils::build_graph,
};
use env_logger::{Builder, Target};
use log::{debug, error, info, warn, LevelFilter};
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::{
    env,
    fs::{File, OpenOptions},
    io::Write,
    process::exit,
    sync::Mutex,
    time::Instant,
};

// Struct para organizar os resultados
#[derive(Debug)]
struct TrialResult {
    graph_name: String,
    node_count: usize,
    edge_count: usize,
    fitness: u32,
    elapsed_micros: u128,
}

// Struct para parâmetros do algoritmo
struct AlgorithmParams {
    max_stagnant: usize,
    generations: usize,
    tournament_size: usize,
    crossover_rate: f64,
    pop_size: usize,
}

fn setup_logger(log_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .create(true)
        .write(true) // Abre o arquivo para escrita
        .append(true) // Adiciona no final do arquivo, sem sobrescrever
        .open(log_file)?;

    Builder::new()
        .target(Target::Pipe(Box::new(file)))
        .filter_level(LevelFilter::Info) // Configura o nível de log
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();

    Ok(())
}

fn parse_args() -> Result<(String, usize, AlgorithmParams), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err(format!(
            "Usage: {} <file_path> <trials> [max_stagnant] [generations] [tournament_size] [crossover_prob] [pop_size]",
            args[0]
        ));
    }

    let file_path = args[1].clone();
    let trials = args[2].parse().map_err(|_| "Invalid trials parameter")?;

    Ok((
        file_path,
        trials,
        AlgorithmParams {
            max_stagnant: args.get(3).and_then(|s| s.parse().ok()).unwrap_or(100),
            generations: args.get(4).and_then(|s| s.parse().ok()).unwrap_or(1000),
            tournament_size: args.get(5).and_then(|s| s.parse().ok()).unwrap_or(5),
            crossover_rate: args.get(6).and_then(|s| s.parse().ok()).unwrap_or(0.9),
            pop_size: args.get(7).and_then(|s| s.parse().ok()).unwrap_or(0),
        },
    ))
}

fn write_results_to_csv(results: &[TrialResult], output_file: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_file)?;

    // Escreve o cabeçalho se o arquivo estiver vazio
    if file.metadata()?.len() == 0 {
        writeln!(
            file,
            "graph_name,graph_order,graph_size,fitness_value,elapsed_time(microsecond)"
        )?;
    }

    for result in results {
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
    // Configura o threadpool do Rayon
    let num_cpus = num_cpus::get();
    ThreadPoolBuilder::new()
        .num_threads(num_cpus)
        .build_global()
        .expect("Failed to configure Rayon thread pool");

    // Parse os argumentos
    let (file_path, trials, params) = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    };

    // Configura o logger
    if let Err(e) = setup_logger("execution.log") {
        eprintln!("Failed to setup logger: {}", e);
        exit(1);
    }

    // Carrega o grafo
    info!("Reading edge list from file: {}", file_path);
    let graph = build_graph(&file_path);
    if graph.node_count() == 0 {
        error!("The graph has no nodes. Exiting.");
        exit(1);
    }

    // Ajusta o tamanho da população se não foi especificado
    let pop_size = if params.pop_size == 0 {
        ((graph.node_count() as f64 / 1.5).ceil() as usize).max(1)
    } else {
        params.pop_size
    };

    // Log dos parâmetros
    info!("Starting genetic algorithm with the following parameters:");
    info!("Max stagnant generations: {}", params.max_stagnant);
    info!("Generations: {}", params.generations);
    info!("Tournament size: {}", params.tournament_size);
    info!("Crossover rate: {}", params.crossover_rate);
    info!("Population size: {}", pop_size);

    let heuristics: Vec<Heuristic> = vec![h1, h2, h3, h4, h5, h1];
    let crossover = SinglePoint::new(params.crossover_rate);
    let selector = KTournament::new(params.tournament_size);

    // Executa os trials em paralelo
    let results = Mutex::new(Vec::new());
    (0..trials).into_par_iter().for_each(|trial| {
        let start_time = Instant::now();
        info!("Starting trial {}", trial);

        let mut population = Population::new(pop_size, &heuristics, &graph);
        let mut best_solution = population
            .best_chromosome()
            .expect("Failed to retrieve the best initial individual")
            .clone();

        let mut stagnant_generations = 0;
        for gen in 0..params.generations {
            info!("Generation {} in trial {}", gen, trial);

            population.envolve(&selector, &crossover, &graph);

            let new_best_solution = population
                .best_chromosome()
                .expect("Failed to retrieve the best individual")
                .clone();

            if new_best_solution.fitness() < best_solution.fitness() {
                best_solution = new_best_solution;
                stagnant_generations = 0;
                info!(
                    "Trial {}: New best solution found in generation {} with fitness {}",
                    trial,
                    gen,
                    best_solution.fitness()
                );
            } else {
                stagnant_generations += 1;
            }

            if stagnant_generations >= params.max_stagnant {
                warn!(
                    "Trial {}: Early stopping due to {} stagnant generations.",
                    trial, params.max_stagnant
                );
                break;
            }
        }

        let elapsed_time = start_time.elapsed();
        let graph_name = file_path.split('/').last().unwrap_or("unknown").to_string();

        results.lock().unwrap().push(TrialResult {
            graph_name,
            node_count: graph.node_count(),
            edge_count: graph.edge_count(),
            fitness: best_solution.fitness(),
            elapsed_micros: elapsed_time.as_micros(),
        });
        info!("Trial {} completed in {:?}", trial, elapsed_time);
    });

    // Gera o nome do arquivo de saída com timestamp
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let output_file = format!("results_{}.csv", timestamp);

    // Escreve os resultados no arquivo CSV
    let results = results.into_inner().unwrap();
    if let Err(e) = write_results_to_csv(&results, &output_file) {
        error!("Failed to write results to CSV: {}", e);
        exit(1);
    }

    info!("Results written to {}", output_file);
}
