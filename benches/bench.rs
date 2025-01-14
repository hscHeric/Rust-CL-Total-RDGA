use cl_total_rdga::genetic::{
    crossover::{CrossoverStrategy, OnePointCrossover},
    heuristics::{h1, h2, h3, h4, h5},
    selection::{KTournamentSelection, SelectionStrategy},
    Chromosome, Population,
};
use criterion::{criterion_group, criterion_main, Criterion};
use kambo_graph::{graphs::simple::UndirectedGraph, Graph, GraphMut};

/// Cria um grafo para os benchmarks.
pub fn create_test_graph() -> UndirectedGraph<usize> {
    use std::{
        env,
        fs::File,
        io::{self, BufRead},
    };

    let file_path = "benches/sherman4_normalized.txt";
    let base_path = env::current_dir().expect("Failed to determine current directory");
    let full_path = base_path.join(file_path);

    println!("Attempting to load graph from: {:?}", full_path);

    let file = File::open(&full_path).unwrap_or_else(|_| {
        panic!("Failed to open the file at: {:?}", full_path);
    });

    let reader = io::BufReader::new(file);
    let mut edges = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.unwrap_or_else(|_| {
            panic!(
                "Failed to read line {} in file: {:?}",
                line_number + 1,
                full_path
            );
        });

        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            panic!("Invalid edge format at line {}: {}", line_number + 1, line);
        }

        let u = parts[0].parse::<usize>().unwrap_or_else(|_| {
            panic!(
                "Invalid vertex format at line {}: {}",
                line_number + 1,
                line
            );
        });
        let v = parts[1].parse::<usize>().unwrap_or_else(|_| {
            panic!(
                "Invalid vertex format at line {}: {}",
                line_number + 1,
                line
            );
        });

        edges.push((u, v));
    }

    if edges.is_empty() {
        panic!(
            "The edge list is empty. Please ensure the file {:?} contains valid edges.",
            full_path
        );
    }

    let mut graph = UndirectedGraph::new_undirected();
    for &(u, v) in &edges {
        graph.add_vertex(u).unwrap_or_default();
        graph.add_vertex(v).unwrap_or_default();
        graph.add_edge(&u, &v).unwrap_or_else(|err| {
            panic!("Failed to add edge ({}, {}): {:?}", u, v, err);
        });
    }

    println!(
        "Graph created successfully with {} vertices and {} edges.",
        graph.order(),
        graph.edge_count()
    );

    graph
}

/// Benchmark para a criação de cromossomos.
fn benchmark_chromosome(c: &mut Criterion) {
    let graph = create_test_graph();
    let chromosome = h1(&graph).expect("h1 returned None");

    c.bench_function("Chromosome::new", |b| {
        b.iter(|| Chromosome::new(chromosome.genes().clone().to_vec()))
    });
}

/// Benchmark para validação de cromossomos.
fn benchmark_is_valid_to_total_roman_domination(c: &mut Criterion) {
    let graph = create_test_graph();
    let chromosome = h2(&graph).expect("h2 returned None");

    c.bench_function("Chromosome::is_valid_to_total_roman_domination", |b| {
        b.iter(|| chromosome.is_valid_to_total_roman_domination(&graph))
    });
}

/// Benchmark para correção de cromossomos.
fn benchmark_fix_chromosome(c: &mut Criterion) {
    let graph = create_test_graph();
    let genes = vec![0; graph.order()];
    let chromosome = Chromosome::new(genes);

    c.bench_function("Chromosome::fix_chromosome", |b| {
        b.iter(|| chromosome.fix_chromosome(&graph))
    });
}

/// Benchmark para as heurísticas.
fn benchmark_heuristics(c: &mut Criterion) {
    let graph = create_test_graph();

    c.bench_function("h1", |b| b.iter(|| h1(&graph).expect("h1 returned None")));
    c.bench_function("h2", |b| b.iter(|| h2(&graph).expect("h2 returned None")));
    c.bench_function("h3", |b| b.iter(|| h3(&graph).expect("h3 returned None")));
    c.bench_function("h4", |b| b.iter(|| h4(&graph).expect("h4 returned None")));
    c.bench_function("h5", |b| b.iter(|| h5(&graph).expect("h5 returned None")));
}

/// Benchmark para a criação de uma população.
fn benchmark_population(c: &mut Criterion) {
    let graph = create_test_graph();
    let heuristics = vec![h1, h2, h3, h4, h5];

    c.bench_function("Population::new", |b| {
        b.iter(|| Population::new(&graph, heuristics.clone(), 10))
    });
}

/// Benchmark para crossover.
fn benchmark_crossover(c: &mut Criterion) {
    let graph = create_test_graph();
    let population = Population::new(&graph, vec![h1, h2, h3, h4, h5], 10).unwrap();
    let strategy = OnePointCrossover {
        crossover_rate: 0.7,
    };

    c.bench_function("OnePointCrossover::crossover", |b| {
        b.iter(|| strategy.crossover(&population, &graph))
    });
}

/// Benchmark para seleção.
fn benchmark_selection(c: &mut Criterion) {
    let graph = create_test_graph();
    let population = Population::new(&graph, vec![h1, h2, h3, h4, h5], 10).unwrap();
    let selection = KTournamentSelection { tournament_size: 3 };

    c.bench_function("KTournamentSelection::select", |b| {
        b.iter(|| selection.select(&population))
    });
}

// Agrupamento dos benchmarks
criterion_group!(
    benches,
    benchmark_chromosome,
    benchmark_is_valid_to_total_roman_domination,
    benchmark_fix_chromosome,
    benchmark_heuristics,
    benchmark_population,
    benchmark_crossover,
    benchmark_selection
);
criterion_main!(benches);
