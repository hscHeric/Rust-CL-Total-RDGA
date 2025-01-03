use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

pub fn from_edge_list_file(file_path: &str) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();

    if let Ok(file) = File::open(file_path) {
        let reader = io::BufReader::new(file);

        reader.lines().for_each(|line| {
            if let Ok(line) = line {
                let mut parts = line.split_whitespace();
                if let (Some(u), Some(v)) = (parts.next(), parts.next()) {
                    if let (Ok(u), Ok(v)) = (u.parse::<usize>(), v.parse::<usize>()) {
                        edges.push((u, v));
                    }
                }
            }
        });
    }

    edges
}

pub fn normalize_edges(edges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut vertex_map = HashMap::new();
    let mut next_index = 0;

    let mut normalized_edges = Vec::new();

    for (u, v) in edges {
        if u == v {
            // Ignora self-loops
            continue;
        }

        let normalized_u = *vertex_map.entry(u).or_insert_with(|| {
            let current = next_index;
            next_index += 1;
            current
        });

        let normalized_v = *vertex_map.entry(v).or_insert_with(|| {
            let current = next_index;
            next_index += 1;
            current
        });

        normalized_edges.push((normalized_u, normalized_v));
    }

    normalized_edges
}
