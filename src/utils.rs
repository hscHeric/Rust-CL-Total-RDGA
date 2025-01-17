use std::{
    fs::File,
    io::{self, BufRead},
};

use kambo_graph::{graphs::simple::UndirectedGraph, GraphMut};

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
pub fn build_graph(file_path: &str) -> UndirectedGraph<usize> {
    let file = File::open(file_path).expect("Failed to open the file");
    let reader = io::BufReader::new(file);

    let mut graph = UndirectedGraph::<usize>::new_undirected();

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

        let u: usize = parts[0].parse().expect("Failed to parse vertex 'u'");
        let v: usize = parts[1].parse().expect("Failed to parse vertex 'v'");

        graph.add_vertex(u).ok();
        graph.add_vertex(v).ok();
        graph.add_edge(&u, &v).expect("Failed to add edge");
    }

    graph
}
