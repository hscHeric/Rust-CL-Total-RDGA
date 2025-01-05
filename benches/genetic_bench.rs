use cl_total_rdga::{
    genetic::{
        h1, heuristics::h0, Chromosome, CrossoverStrategy, KTournamentSelection, Population,
        SelectionStrategy, TwoPointCrossover,
    },
    graph::{parser::from_edge_list_file, parser::normalize_edges, SimpleGraph},
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const GRAPH_FILE: &str = "benches/sherman4.txt";

fn bench_from_edge_list_file(c: &mut Criterion) {
    c.bench_function("from_edge_list_file", |b| {
        b.iter(|| from_edge_list_file(black_box(GRAPH_FILE)))
    });
}

fn bench_normalize_edges(c: &mut Criterion) {
    let edge_list = from_edge_list_file(GRAPH_FILE);
    c.bench_function("normalize_edges", |b| {
        b.iter(|| normalize_edges(black_box(edge_list.clone())))
    });
}

fn bench_simplegraph_from_edges(c: &mut Criterion) {
    let edge_list = normalize_edges(from_edge_list_file(GRAPH_FILE));
    c.bench_function("SimpleGraph::from_edges", |b| {
        b.iter(|| SimpleGraph::from_edges(black_box(edge_list.clone())))
    });
}

fn bench_population_new(c: &mut Criterion) {
    let edge_list = normalize_edges(from_edge_list_file(GRAPH_FILE));
    let graph = SimpleGraph::from_edges(edge_list).expect("Erro ao criar o grafo");
    let heuristics: Vec<fn(&SimpleGraph) -> Option<Chromosome>> = vec![h1, h0];
    let pop_size = (graph.vertex_count() as f64 / 1.5).ceil() as usize;

    c.bench_function("Population::new", |b| {
        b.iter(|| {
            Population::new(
                black_box(&graph),
                black_box(heuristics.clone()),
                black_box(pop_size),
            )
        })
    });
}

fn bench_k_tournament_selection(c: &mut Criterion) {
    let edge_list = normalize_edges(from_edge_list_file(GRAPH_FILE));
    let graph = SimpleGraph::from_edges(edge_list).expect("Erro ao criar o grafo");
    let heuristics: Vec<fn(&SimpleGraph) -> Option<Chromosome>> = vec![h1, h0];
    let pop_size = (graph.vertex_count() as f64 / 1.5).ceil() as usize;
    let population = Population::new(&graph, heuristics.clone(), pop_size)
        .expect("Erro ao criar a população inicial");
    let selection_strategy = KTournamentSelection { tournament_size: 5 };

    c.bench_function("KTournamentSelection::select", |b| {
        b.iter(|| selection_strategy.select(black_box(&population)))
    });
}

fn bench_crossover(c: &mut Criterion) {
    let edge_list = normalize_edges(from_edge_list_file(GRAPH_FILE));
    let graph = SimpleGraph::from_edges(edge_list).expect("Erro ao criar o grafo");
    let heuristics: Vec<fn(&SimpleGraph) -> Option<Chromosome>> = vec![h1, h0];
    let pop_size = (graph.vertex_count() as f64 / 1.5).ceil() as usize;
    let population = Population::new(&graph, heuristics.clone(), pop_size)
        .expect("Erro ao criar a população inicial");
    let crossover_strategy = TwoPointCrossover {
        crossover_rate: 0.9,
    };

    c.bench_function("TwoPointCrossover::crossover", |b| {
        b.iter(|| crossover_strategy.crossover(black_box(&population), black_box(&graph)))
    });
}

fn bench_validate_population(c: &mut Criterion) {
    let edge_list = normalize_edges(from_edge_list_file(GRAPH_FILE));
    let graph = SimpleGraph::from_edges(edge_list).expect("Erro ao criar o grafo");
    let heuristics: Vec<fn(&SimpleGraph) -> Option<Chromosome>> = vec![h1, h0];
    let pop_size = (graph.vertex_count() as f64 / 1.5).ceil() as usize;
    let population = Population::new(&graph, heuristics.clone(), pop_size)
        .expect("Erro ao criar a população inicial");

    c.bench_function("Population::validate_population", |b| {
        b.iter(|| population.validate_population(black_box(&graph)))
    });
}

fn bench_best_individual(c: &mut Criterion) {
    let edge_list = normalize_edges(from_edge_list_file(GRAPH_FILE));
    let graph = SimpleGraph::from_edges(edge_list).expect("Erro ao criar o grafo");
    let heuristics: Vec<fn(&SimpleGraph) -> Option<Chromosome>> = vec![h1, h0];
    let pop_size = (graph.vertex_count() as f64 / 1.5).ceil() as usize;
    let population = Population::new(&graph, heuristics.clone(), pop_size)
        .expect("Erro ao criar a população inicial");

    c.bench_function("Population::best_individual", |b| {
        b.iter(|| {
            population
                .best_individual()
                .expect("Erro ao obter o melhor indivíduo")
        })
    });
}

criterion_group!(
    benches,
    bench_from_edge_list_file,
    bench_normalize_edges,
    bench_simplegraph_from_edges,
    bench_population_new,
    bench_k_tournament_selection,
    bench_crossover,
    bench_validate_population,
    bench_best_individual
);
criterion_main!(benches);
