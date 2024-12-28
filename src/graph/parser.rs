use std::{
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
