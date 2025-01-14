use cl_total_rdga::genetic::chromosome::Chromosome;
use petgraph::graph::UnGraph;

fn main() {
    println!("Hello");
    let mut graph = UnGraph::<usize, ()>::new_undirected();
    let v0 = graph.add_node(0); // Nó 0
    let v1 = graph.add_node(1); // Nó 1
    let v2 = graph.add_node(2); // Nó 2
    let v3 = graph.add_node(3); // Nó 3

    graph.add_edge(v0, v1, ());
    graph.add_edge(v1, v2, ());
    graph.add_edge(v2, v3, ());
    graph.add_edge(v3, v0, ());

    let genes = vec![0, 0, 1, 0];
    let mut chromosome = Chromosome::new(genes);
    chromosome.fix(&graph);

    println!("Chromosome: {:?}", chromosome.genes());
}
