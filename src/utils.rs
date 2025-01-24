use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

use kambo_graph::{graphs::simple::UndirectedGraph, Graph, GraphMut};

/// Builds an undirected graph from a file.
///
/// # Arguments
///
/// * `file_path` - The path to the file containing the graph edges.
///
/// # File Format
/// Each line in the file should represent an edge in the format `u v`, where `u` and `v` are vertices.
/// Lines that are empty or start with `#` are ignored.
///
/// # Errors
///
/// This function will panic if:
/// - The file cannot be opened.
/// - A line in the file does not have exactly two values.
/// - A vertex cannot be parsed as an integer.
/// - An edge cannot be added to the graph due to invalid vertices or duplicate edges.
///
/// The resulting graph will have the edges (1-2), (2-3), and (3-4).
///
/// # Panics
/// This function panics if the input format is invalid.
#[must_use]
pub fn build_graph(file_path: &str) -> UndirectedGraph<u32> {
    let file = File::open(file_path).expect("Failed to open the file");
    let reader = io::BufReader::new(file);

    let mut graph = UndirectedGraph::<u32>::new_undirected();

    for line in reader.lines() {
        let line = match line {
            Ok(content) => content.trim().to_string(),
            Err(err) => panic!("Error reading a line from the file: {err}"),
        };

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        assert!(parts.len() == 2, "Invalid format on line: {line}");

        let u: u32 = parts[0].parse().expect("Failed to parse vertex 'u'");
        let v: u32 = parts[1].parse().expect("Failed to parse vertex 'v'");

        graph.add_vertex(u).ok();
        graph.add_vertex(v).ok();
        graph.add_edge(&u, &v).expect("Failed to add edge");
    }

    normalize_graph(&graph)
}

/// Normalizes the vertex indices of a graph to be contiguous from 0 to n-1.
///
/// # Arguments
///
/// * `graph` - A mutable reference to an `UndirectedGraph<usize>` to normalize.
///
/// # Returns
/// A new `UndirectedGraph<usize>` with normalized indices.
fn normalize_graph(graph: &UndirectedGraph<u32>) -> UndirectedGraph<u32> {
    let mut vertex_map = HashMap::new();
    let mut new_index = 0;

    // Cria o grafo normalizado
    let mut normalized_graph = UndirectedGraph::<u32>::new_undirected();

    for &vertex in graph.vertices() {
        vertex_map.entry(vertex).or_insert_with(|| {
            let current_index = new_index;
            normalized_graph.add_vertex(new_index).unwrap();
            new_index += 1;
            current_index
        });
    }

    for (u, neighbors) in graph.vertices().map(|v| (v, graph.neighbors(v).unwrap())) {
        for &v in neighbors {
            let new_u = vertex_map[u];
            let new_v = vertex_map[&v];
            if !normalized_graph.contains_edge(&new_u, &new_v) {
                normalized_graph.add_edge(&new_u, &new_v).unwrap();
            }
        }
    }

    normalized_graph
}
