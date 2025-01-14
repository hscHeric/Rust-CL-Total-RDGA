use petgraph::{
    graph::UnGraph,
    visit::{EdgeRef, NodeIndexable},
};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

/// Removes isolated vertices (nodes with no edges) from the graph.
///
/// # Arguments
///
/// * `graph` - A mutable reference to an undirected graph (`UnGraph<u32, ()>`).
pub fn remove_isolated_vertices(graph: &mut UnGraph<u32, ()>) {
    let isolated_nodes: Vec<_> = graph
        .node_indices()
        .filter(|&n| graph.neighbors(n).next().is_none())
        .collect();
    for node in isolated_nodes {
        graph.remove_node(node);
    }
}

/// Normalizes the graph so that node indices are consecutive integers starting from zero.
///
/// This function creates a new graph with the same structure but ensures
/// that node indices are compact and sequential. The node weights are preserved.
///
/// # Arguments
///
/// * `graph` - A mutable reference to an undirected graph (`UnGraph<u32, ()>`).
///
/// # Panics
///
/// * unwrap
pub fn normalize_graph(graph: &mut UnGraph<u32, ()>) {
    let mut new_graph = UnGraph::<u32, ()>::new_undirected();
    let mut mapping = vec![None; graph.node_bound()];

    // Map old indices to new consecutive indices
    for node in graph.node_indices() {
        let weight = *graph.node_weight(node).unwrap();
        let new_index = new_graph.add_node(weight);
        mapping[node.index()] = Some(new_index);
    }

    // Add edges with normalized indices
    for edge in graph.edge_references() {
        let source = mapping[edge.source().index()].unwrap();
        let target = mapping[edge.target().index()].unwrap();
        new_graph.add_edge(source, target, ());
    }

    *graph = new_graph;
}

/// Builds a graph from an edge list file.
///
/// The file should contain one edge per line in the format:
/// `<node1> <node2>`, where `<node1>` and `<node2>` are integers representing
/// the connected nodes. Nodes will be automatically added to the graph
/// as they are encountered.
///
/// This function also removes isolated nodes and normalizes the graph.
///
/// # Arguments
///
/// * `file_path` - Path to the edge list file.
///
/// # Returns
///
/// A new `UnGraph<u32, ()>` representing the graph described in the file.
///
/// # Panics
///
/// This function will panic if the file cannot be opened or if there are invalid
/// lines in the file that cannot be parsed as integers or invalid line
#[must_use]
pub fn build_graph(file_path: &str) -> UnGraph<u32, ()> {
    let file = File::open(file_path).expect("Failed to open the file");
    let reader = io::BufReader::new(file);

    let mut graph = UnGraph::<u32, ()>::new_undirected();
    let mut node_mapping = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read a line from the file");
        let parts: Vec<_> = line.split_whitespace().collect();

        assert!(parts.len() == 2, "Invalid Line");

        let u: u32 = parts[0].parse().expect("Failed to parse node as u32");
        let v: u32 = parts[1].parse().expect("Failed to parse node as u32");

        // Ignore self-loops
        if u == v {
            continue;
        }

        let u_index = *node_mapping.entry(u).or_insert_with(|| graph.add_node(u));
        let v_index = *node_mapping.entry(v).or_insert_with(|| graph.add_node(v));
        graph.add_edge(u_index, v_index, ());
    }

    remove_isolated_vertices(&mut graph);

    normalize_graph(&mut graph);

    graph
}
