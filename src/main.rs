use std::{env, process::exit};

use cl_total_rdga::utils::build_graph_from_edges;
use kambo_graph::{graphs::simple::UndirectedGraph, utils::edge_list::parse_edge_list, Graph};
use rayon::ThreadPoolBuilder;

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
    let mut graph: UndirectedGraph<usize> =
        build_graph_from_edges(edges).unwrap_or(UndirectedGraph::new_directed());

    if graph.has_isolated_vertex() {
        graph.remove_isolated_vertices().unwrap();
    }

    // Parse dos valores opcionais ( O padrão para os valores opcionais vai ser o mesmo do CL-RD)
    let max_stagnant = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(100);
    let generations = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(1000);
    let tournament_size = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(5);
    let crossover_rate = args.get(6).and_then(|s| s.parse().ok()).unwrap_or(0.9);
    let pop_size = args
        .get(7)
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| ((graph.vertex_count() as f64 / 1.5).ceil() as usize).max(1));
}
