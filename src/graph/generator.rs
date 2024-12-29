use rand::Rng;

pub struct SimpleGraphGenerator {
    vertex_count: usize,
    edge_probability: f64,
}

impl SimpleGraphGenerator {
    pub fn new(vertex_count: usize, edge_probability: f64) -> Self {
        if !(0.0..=1.0).contains(&edge_probability) {
            panic!("A probabilidade de aresta deve estar entre 0.0 e 1.0.");
        }
        Self {
            vertex_count,
            edge_probability,
        }
    }

    pub fn generate(&self) -> Result<Vec<(usize, usize)>, String> {
        if self.vertex_count == 0 {
            return Err("O número de vértices deve ser maior que zero.".into());
        }

        let mut edges = Vec::new();
        let mut rng = rand::thread_rng();

        for u in 0..self.vertex_count {
            for v in (u + 1)..self.vertex_count {
                if rng.gen_bool(self.edge_probability) {
                    edges.push((u, v));
                }
            }
        }

        let mut connected = vec![false; self.vertex_count];
        for &(u, v) in &edges {
            connected[u] = true;
            connected[v] = true;
        }

        for u in 0..self.vertex_count {
            if !connected[u] {
                let mut added = false;
                while !added {
                    let random_neighbor = rng.gen_range(0..self.vertex_count);
                    if random_neighbor != u
                        && !edges.contains(&(u.min(random_neighbor), u.max(random_neighbor)))
                    {
                        edges.push((u.min(random_neighbor), u.max(random_neighbor)));
                        connected[u] = true;
                        connected[random_neighbor] = true;
                        added = true;
                    }
                }
            }
        }

        Ok(edges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_simple_graph() {
        let generator = SimpleGraphGenerator::new(5, 0.3);
        let edges = generator.generate().unwrap();

        assert!(!edges.is_empty(), "O grafo não deveria estar vazio.");

        let mut edge_set = std::collections::HashSet::new();
        for &(u, v) in &edges {
            assert_ne!(u, v, "Não deve haver laços.");
            assert!(
                edge_set.insert((u.min(v), u.max(v))),
                "Não deve haver arestas múltiplas."
            );
        }
    }

    #[test]
    fn test_all_vertices_connected() {
        let generator = SimpleGraphGenerator::new(10, 0.0); // Probabilidade 0, força conectividade
        let edges = generator.generate().unwrap();

        let mut connected = [false; 10];
        for &(u, v) in &edges {
            connected[u] = true;
            connected[v] = true;
        }

        assert!(
            connected.iter().all(|&is_connected| is_connected),
            "Todos os vértices devem estar conectados."
        );
    }

    #[test]
    fn test_zero_vertices() {
        let generator = SimpleGraphGenerator::new(0, 0.5);
        assert!(generator.generate().is_err());
    }
}
