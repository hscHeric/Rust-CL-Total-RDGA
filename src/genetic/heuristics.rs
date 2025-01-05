use rand::seq::SliceRandom;

use crate::graph::SimpleGraph;

use super::Chromosome;

pub fn h1(graph: &SimpleGraph) -> Option<Chromosome> {
    let vertex_count = graph.vertex_count();
    if vertex_count == 0 {
        return None; // Grafo vazio, não há como gerar cromossomo
    }

    let mut genes = vec![3; vertex_count];
    let mut rng = rand::thread_rng();

    let mut vertex_degrees: Vec<(usize, usize)> = graph
        .adjacency_list
        .iter()
        .map(|(&vertex, neighbors)| (vertex, neighbors.len()))
        .collect();

    vertex_degrees.sort_by_key(|&(_, degree)| std::cmp::Reverse(degree));

    let mut modified = true;
    while modified {
        modified = false; // Assume que nenhuma modificação será feita

        for (vertex, degree) in &vertex_degrees {
            if genes[*vertex] == 3 {
                // Caso f[v] == 3
                if *degree == 1 {
                    genes[*vertex] = 1; // Define f[v] = 1
                    if let Ok(neighbors) = graph.neighbors(*vertex) {
                        for &neighbor in neighbors {
                            if genes[neighbor] == 3 {
                                genes[neighbor] = 1; // Define f[u] = 1 para o vizinho
                                modified = true; // Modificação realizada
                            }
                        }
                    }
                } else if *degree >= 2 {
                    genes[*vertex] = 2; // Define f[v] = 2
                    if let Ok(neighbors) = graph.neighbors(*vertex) {
                        let mut unprocessed_neighbors: Vec<_> = neighbors
                            .iter()
                            .filter(|&&neighbor| genes[neighbor] == 3)
                            .copied()
                            .collect();

                        if let Some(&selected_neighbor) = unprocessed_neighbors.choose(&mut rng) {
                            genes[selected_neighbor] = 1; // Define vizinho aleatório como 1
                            unprocessed_neighbors.retain(|&n| n != selected_neighbor);
                            modified = true; // Modificação realizada
                        }

                        for neighbor in unprocessed_neighbors {
                            genes[neighbor] = 0; // Define todos os outros vizinhos como 0
                            modified = true; // Modificação realizada
                        }
                    }
                }
            } else {
                // Caso f[v] != 3
                if genes[*vertex] == 1 {
                    // Caso f[v] == 1
                    if let Ok(neighbors) = graph.neighbors(*vertex) {
                        let has_one_or_two =
                            neighbors.iter().any(|&n| genes[n] == 1 || genes[n] == 2);
                        if !has_one_or_two {
                            // Escolhe um vizinho aleatório e define como 1
                            if let Some(&selected_neighbor) = neighbors
                                .iter()
                                .filter(|&&n| genes[n] == 3)
                                .copied()
                                .collect::<Vec<_>>()
                                .choose(&mut rng)
                            {
                                genes[selected_neighbor] = 1;
                                modified = true; // Modificação realizada
                            }
                        }
                    }
                } else if genes[*vertex] == 2 {
                    // Caso f[v] == 2
                    if let Ok(neighbors) = graph.neighbors(*vertex) {
                        let has_one_or_two =
                            neighbors.iter().any(|&n| genes[n] == 1 || genes[n] == 2);
                        if !has_one_or_two {
                            // Escolhe um vizinho aleatório e define como 1
                            if let Some(&selected_neighbor) = neighbors
                                .iter()
                                .filter(|&&n| genes[n] == 3)
                                .copied()
                                .collect::<Vec<_>>()
                                .choose(&mut rng)
                            {
                                genes[selected_neighbor] = 1;
                                modified = true; // Modificação realizada
                            }
                        }
                    }
                } else if genes[*vertex] == 0 {
                    // Caso f[v] == 0
                    if let Ok(neighbors) = graph.neighbors(*vertex) {
                        let has_two = neighbors.iter().any(|&n| genes[n] == 2);
                        if !has_two {
                            // Escolhe um vizinho aleatório e define como 2
                            if let Some(&selected_neighbor) = neighbors
                                .iter()
                                .filter(|&&n| genes[n] == 3)
                                .copied()
                                .collect::<Vec<_>>()
                                .choose(&mut rng)
                            {
                                genes[selected_neighbor] = 2;
                                modified = true;
                            }
                        }
                    }
                }
            }
        }
    }

    // Valida o cromossomo gerado
    let chromosome = Chromosome::new(genes);
    if chromosome.is_valid_to_total_roman_domination(graph) {
        Some(chromosome)
    } else {
        Some(chromosome.fix_chromosome(graph))
    }
}

pub fn h0(graph: &SimpleGraph) -> Option<Chromosome> {
    let vertex_count = graph.vertex_count();
    if vertex_count == 0 {
        return None; // Grafo vazio, não há como gerar cromossomo
    }

    let mut genes = vec![3; vertex_count];
    let mut rng = rand::thread_rng();

    // Gera uma ordem aleatória dos vértices
    let mut vertices: Vec<usize> = (0..vertex_count).collect();
    vertices.shuffle(&mut rng);

    let mut modified = true;
    while modified {
        modified = false; // Assume que nenhuma modificação será feita

        for &vertex in &vertices {
            if genes[vertex] == 3 {
                if let Ok(neighbors) = graph.neighbors(vertex) {
                    let degree = neighbors.len();

                    if degree == 1 {
                        // Caso f[v] == 3 e grau == 1, define f[v] = 1
                        genes[vertex] = 1;
                        for &neighbor in neighbors {
                            if genes[neighbor] == 3 {
                                genes[neighbor] = 1; // Define f[u] = 1 para o vizinho
                                modified = true;
                            }
                        }
                    } else if degree >= 2 {
                        // Caso f[v] == 3 e grau >= 2, define f[v] = 2
                        genes[vertex] = 2;

                        let mut unprocessed_neighbors: Vec<_> = neighbors
                            .iter()
                            .filter(|&&n| genes[n] == 3)
                            .copied()
                            .collect();

                        if let Some(&selected_neighbor) = unprocessed_neighbors.choose(&mut rng) {
                            genes[selected_neighbor] = 1; // Define um vizinho aleatório como 1
                            unprocessed_neighbors.retain(|&n| n != selected_neighbor);
                            modified = true;
                        }

                        for neighbor in unprocessed_neighbors {
                            genes[neighbor] = 0; // Define todos os outros vizinhos como 0
                            modified = true;
                        }
                    }
                }
            } else {
                // Caso f[v] já esteja definido
                if genes[vertex] == 1 {
                    if let Ok(neighbors) = graph.neighbors(vertex) {
                        let has_one_or_two =
                            neighbors.iter().any(|&n| genes[n] == 1 || genes[n] == 2);
                        if !has_one_or_two {
                            if let Some(&selected_neighbor) = neighbors
                                .iter()
                                .filter(|&&n| genes[n] == 3)
                                .copied()
                                .collect::<Vec<_>>()
                                .choose(&mut rng)
                            {
                                genes[selected_neighbor] = 1;
                                modified = true;
                            }
                        }
                    }
                } else if genes[vertex] == 2 {
                    if let Ok(neighbors) = graph.neighbors(vertex) {
                        let has_one_or_two =
                            neighbors.iter().any(|&n| genes[n] == 1 || genes[n] == 2);
                        if !has_one_or_two {
                            if let Some(&selected_neighbor) = neighbors
                                .iter()
                                .filter(|&&n| genes[n] == 3)
                                .copied()
                                .collect::<Vec<_>>()
                                .choose(&mut rng)
                            {
                                genes[selected_neighbor] = 1;
                                modified = true;
                            }
                        }
                    }
                } else if genes[vertex] == 0 {
                    if let Ok(neighbors) = graph.neighbors(vertex) {
                        let has_two = neighbors.iter().any(|&n| genes[n] == 2);
                        if !has_two {
                            if let Some(&selected_neighbor) = neighbors
                                .iter()
                                .filter(|&&n| genes[n] == 3)
                                .copied()
                                .collect::<Vec<_>>()
                                .choose(&mut rng)
                            {
                                genes[selected_neighbor] = 2;
                                modified = true;
                            }
                        }
                    }
                }
            }
        }
    }

    let chromosome = Chromosome::new(genes);
    if chromosome.is_valid_to_total_roman_domination(graph) {
        Some(chromosome)
    } else {
        Some(chromosome.fix_chromosome(graph))
    }
}

pub fn h2(graph: &SimpleGraph) -> Option<Chromosome> {
    let vertex_count = graph.vertex_count();
    if vertex_count == 0 {
        return None; // Grafo vazio, não há como gerar cromossomo
    }

    let mut genes = vec![3; vertex_count];
    let mut vertex_degrees: Vec<(usize, usize)> = graph
        .adjacency_list
        .iter()
        .map(|(&vertex, neighbors)| (vertex, neighbors.len()))
        .collect();

    vertex_degrees.sort_by_key(|&(_, degree)| std::cmp::Reverse(degree));

    let mut modified = true;
    while modified {
        modified = false; // Assume que nenhuma modificação será feita

        for (vertex, degree) in &vertex_degrees {
            if genes[*vertex] == 3 {
                // Caso f[v] == 3
                if *degree == 1 {
                    genes[*vertex] = 1; // Define f[v] = 1
                    if let Ok(neighbors) = graph.neighbors(*vertex) {
                        for &neighbor in neighbors {
                            if genes[neighbor] == 3 {
                                genes[neighbor] = 1; // Define f[u] = 1 para o vizinho
                                modified = true; // Modificação realizada
                            }
                        }
                    }
                } else if *degree >= 2 {
                    genes[*vertex] = 2; // Define f[v] = 2
                    if let Ok(neighbors) = graph.neighbors(*vertex) {
                        let mut unprocessed_neighbors: Vec<_> = neighbors
                            .iter()
                            .filter(|&&neighbor| genes[neighbor] == 3)
                            .copied()
                            .collect();

                        // Seleciona o vizinho de maior grau
                        if let Some(&selected_neighbor) = unprocessed_neighbors
                            .iter()
                            .max_by_key(|&&neighbor| graph.adjacency_list[&neighbor].len())
                        {
                            genes[selected_neighbor] = 1; // Define vizinho com maior grau como 1
                            unprocessed_neighbors.retain(|&n| n != selected_neighbor);
                            modified = true; // Modificação realizada
                        }

                        for neighbor in unprocessed_neighbors {
                            genes[neighbor] = 0; // Define todos os outros vizinhos como 0
                            modified = true; // Modificação realizada
                        }
                    }
                }
            }
        }
    }

    // Valida o cromossomo gerado
    let chromosome = Chromosome::new(genes);
    if chromosome.is_valid_to_total_roman_domination(graph) {
        Some(chromosome)
    } else {
        Some(chromosome.fix_chromosome(graph))
    }
}
