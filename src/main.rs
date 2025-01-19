use cl_total_rdga::{
    genetic::{h1, h2, h3, h4, h5, Heuristic, KTournament, Population, SinglePoint},
    utils::build_graph,
};
use kambo_graph::Graph;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{env, process::exit, sync::Mutex};

#[derive(Debug)]
struct TrialResult {
    fitness: usize,
}

#[derive(Debug)]
struct AlgorithmParams {
    max_stagnant: usize,
    generations: usize,
    tournament_size: usize,
    crossover_rate: f64,
    pop_size: usize,
}

fn parse_args() -> Result<(String, usize, AlgorithmParams), String> {
    let args: Vec<String> = env::args().collect();
    let mut instance_path = String::new();
    let mut trials = 1;
    let mut params = AlgorithmParams {
        max_stagnant: 100,
        generations: 1000,
        tournament_size: 5,
        crossover_rate: 0.9,
        pop_size: 50,
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--instance" => {
                i += 1;
                if i < args.len() {
                    // Remove qualquer ocorrência duplicada de "instances/" no caminho
                    let path = args[i].clone();
                    instance_path = if path.contains("instances/instances/") {
                        path.replace("instances/instances/", "instances/")
                    } else {
                        path
                    };
                }
            }
            "--trials" => {
                i += 1;
                if i < args.len() {
                    trials = args[i].parse().map_err(|_| "Invalid trials parameter")?;
                }
            }
            "--population" => {
                i += 1;
                if i < args.len() {
                    params.pop_size = args[i].parse().map_err(|_| "Invalid population size")?;
                }
            }
            "--generations" => {
                i += 1;
                if i < args.len() {
                    params.generations = args[i].parse().map_err(|_| "Invalid generations")?;
                }
            }
            "--tournament" => {
                i += 1;
                if i < args.len() {
                    params.tournament_size =
                        args[i].parse().map_err(|_| "Invalid tournament size")?;
                }
            }
            "--crossover" => {
                i += 1;
                if i < args.len() {
                    params.crossover_rate =
                        args[i].parse().map_err(|_| "Invalid crossover rate")?;
                }
            }
            "--stagnation" => {
                i += 1;
                if i < args.len() {
                    params.max_stagnant =
                        args[i].parse().map_err(|_| "Invalid stagnation limit")?;
                }
            }
            _ => {
                return Err(format!("Unknown parameter: {}", args[i]));
            }
        }
        i += 1;
    }

    if instance_path.is_empty() {
        return Err("Missing required parameter: --instance".to_string());
    }

    Ok((instance_path, trials, params))
}

fn main() {
    let (instance_path, trials, params) = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    };

    let graph = build_graph(&instance_path);
    if graph.order() == 0 {
        eprintln!("The graph has no nodes. Exiting.");
        exit(1);
    }

    let pop_size = if params.pop_size == 0 {
        ((graph.order() as f64 / 1.5).ceil() as usize).max(1)
    } else {
        params.pop_size
    };

    let heuristics: Vec<Heuristic> = vec![h1, h2, h3, h4, h5, h1];
    let crossover = SinglePoint::new(params.crossover_rate);
    let selector = KTournament::new(params.tournament_size);

    let results = Mutex::new(Vec::with_capacity(trials));

    (0..trials).into_par_iter().for_each(|_| {
        let mut population = Population::new(pop_size, &heuristics, &graph);
        let mut best_solution = population
            .best_chromosome()
            .expect("Failed to retrieve the best individual")
            .clone();

        let mut stagnant_generations = 0;
        for _ in 0..params.generations {
            population.envolve(&selector, &crossover, &graph);
            let new_best_solution = population
                .best_chromosome()
                .expect("Failed to retrieve the best individual")
                .clone();

            if new_best_solution.fitness() < best_solution.fitness() {
                best_solution = new_best_solution;
                stagnant_generations = 0;
            } else {
                stagnant_generations += 1;
            }

            if stagnant_generations >= params.max_stagnant {
                break;
            }
        }

        results.lock().unwrap().push(TrialResult {
            fitness: best_solution.fitness(),
        });
    });

    let results = results.into_inner().unwrap();
    let best_fitness = results
        .iter()
        .min_by_key(|r| r.fitness)
        .map(|r| r.fitness)
        .unwrap_or(usize::MAX);

    println!("{}", best_fitness);
}
