use cl_total_rdga::{
    genetic::{h1, h2, h3, h4, h5, Heuristic, KTournament, Population, SinglePoint},
    utils::build_graph,
};
use kambo_graph::Graph;
use std::{env, process::exit};

#[derive(Debug)]
struct AlgorithmParams {
    max_stagnant: usize,
    generations: usize,
    tournament_size: usize,
    crossover_rate: f64,
    pop_factor: f64, // População será calculada como graph.order / pop_factor
}

fn parse_args() -> Result<(String, AlgorithmParams), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 9 {
        return Err("Insufficient arguments provided".to_string());
    }

    // Ignorar os primeiros 3 argumentos fixos
    let mut args_iter = args.iter().skip(4);

    let instance_path = args_iter.next().ok_or("Missing instance path")?.clone();

    let mut params = AlgorithmParams {
        max_stagnant: 100,
        generations: 1000,
        tournament_size: 5,
        crossover_rate: 0.9,
        pop_factor: 1.0,
    };

    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "--population" => {
                if let Some(value) = args_iter.next() {
                    params.pop_factor = value.parse().map_err(|_| "Invalid population factor")?;
                }
            }
            "--generations" => {
                if let Some(value) = args_iter.next() {
                    params.generations = value.parse().map_err(|_| "Invalid generations")?;
                }
            }
            "--tournament" => {
                if let Some(value) = args_iter.next() {
                    params.tournament_size =
                        value.parse().map_err(|_| "Invalid tournament size")?;
                }
            }
            "--crossover" => {
                if let Some(value) = args_iter.next() {
                    params.crossover_rate = value.parse().map_err(|_| "Invalid crossover rate")?;
                }
            }
            "--stagnation" => {
                if let Some(value) = args_iter.next() {
                    params.max_stagnant = value.parse().map_err(|_| "Invalid stagnation limit")?;
                }
            }
            _ => {}
        }
    }

    Ok((instance_path, params))
}

fn main() {
    let (instance_path, params) = match parse_args() {
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

    let pop_size = ((graph.order() as f64 / params.pop_factor).ceil() as usize).max(1);

    let heuristics: Vec<Heuristic> = vec![h1, h2, h3, h4, h5, h1];
    let crossover = SinglePoint::new(params.crossover_rate);
    let selector = KTournament::new(params.tournament_size);

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

    let best_fitness = best_solution.fitness();

    println!("{}", best_fitness);
}
