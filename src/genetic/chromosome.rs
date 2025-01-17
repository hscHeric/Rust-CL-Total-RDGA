use std::collections::HashMap;

use kambo_graph::{graphs::simple::UndirectedGraph, Graph};

/// Structure representing a chromosome in the CL-Total-RDGA.
///
/// Each chromosome stores a configuration of labels \{0, 1, 2\} for the vertices of a graph,
/// with the goal of satisfying the conditions of total Roman domination.
///
/// # Fields
/// - `genes: Vec<u8>`: A vector that stores the labels for each vertex in the graph.
///   - `0`: Must have a vertex labeled with the value `2` in its neighborhood.
///   - `1 | 2`: Must have a vertex labeled with `f > 0` in its neighborhood.
/// - `neighbors_cache`: cache
#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<u8>,
    neighbors_cache: Option<NeighborsCache>,
}

#[derive(Clone, Debug)]
struct NeighborsCache {
    has_one_neighbor: Vec<bool>,
    has_two_neighbor: Vec<bool>,
    vertex_neighbors: HashMap<u32, Vec<u32>>,
}

impl Chromosome {
    /// Creates a new chromosome from a vector of genes.
    ///
    /// # Parameters
    /// - `genes: Vec<u8>`: The vector containing the initial labels for the vertices.
    ///
    /// # Returns
    /// - Returns a new instance of `Chromosome`.
    #[inline]
    #[must_use]
    pub fn new(genes: Vec<u8>) -> Self {
        Self {
            genes,
            neighbors_cache: None,
        }
    }

    /// Calculates the "fitness" value of the chromosome.
    ///
    /// The fitness is defined as the total weight of the total Roman domination function,
    /// which is the sum of all values in the gene vector.
    ///
    /// # Returns
    /// - A `u32` value corresponding to the sum of the genes.
    #[inline]
    #[must_use]
    pub fn fitness(&self) -> usize {
        self.genes.iter().map(|&x| usize::from(x)).sum()
    }

    /// Returns a slice containing the genes of the chromosome.
    ///
    /// # Returns
    /// - A slice of the gene vector (`&[u8]`).
    #[inline]
    #[must_use]
    pub fn genes(&self) -> &[u8] {
        &self.genes
    }

    fn initialize_cache(&mut self, graph: &UndirectedGraph<u32>) {
        let vertex_count = self.genes.len();
        let mut cache = NeighborsCache {
            has_one_neighbor: vec![false; vertex_count],
            has_two_neighbor: vec![false; vertex_count],
            vertex_neighbors: HashMap::with_capacity(vertex_count),
        };

        for vertex in graph.vertices() {
            let neighbors: Vec<u32> = graph
                .neighbors(vertex)
                .map(|n| n.copied().collect())
                .unwrap_or_default();

            cache.vertex_neighbors.insert(*vertex, neighbors);
        }

        self.update_cache(&mut cache);
        self.neighbors_cache = Some(cache);
    }

    fn update_cache(&self, cache: &mut NeighborsCache) {
        for (v, neighbors) in &cache.vertex_neighbors {
            cache.has_one_neighbor[*v as usize] =
                neighbors.iter().any(|&n| self.genes[n as usize] > 0);

            cache.has_two_neighbor[*v as usize] =
                neighbors.iter().any(|&n| self.genes[n as usize] == 2);
        }
    }

    /// Corrects the chromosome's gene configuration to satisfy the conditions of total Roman domination.
    ///
    /// This method ensures that each vertex in the graph satisfies the following conditions:
    /// - Vertices labeled `0` must have at least one neighbor labeled `2`.
    /// - Vertices labeled `1` or `2` must have at least one neighbor with a label greater than `0`.
    ///
    /// The method iteratively adjusts the labels of vertices in the chromosome until these conditions are met.
    ///
    /// # Parameters
    /// - `graph: &UndirectedGraph<u32>`: The graph representing the structure and relationships
    ///   between vertices. The graph is used to determine the neighbors of each vertex.
    ///
    /// # Details
    /// - If the `neighbors_cache` is not initialized, this method initializes it before proceeding.
    /// - The method uses a cache to track which vertices have neighbors with specific labels (`1` or `2`)
    ///   to optimize label correction.
    /// - It modifies the gene vector in-place, ensuring all conditions of total Roman domination
    ///   are satisfied.
    ///
    /// # Panics
    /// - The method will `panic` if a vertex has an invalid label. Valid labels are:
    ///   - `0`: Requires at least one neighbor labeled `2`.
    ///   - `1`: Requires at least one neighbor labeled with a value greater than `0`.
    ///   - `2`: Requires at least one neighbor labeled with a value greater than `0`.
    ///
    ///   The panic message includes the vertex index and the invalid label value, providing context for debugging:
    ///   ```text
    ///   Vertex with invalid label found! Index: {vertex_idx}, Value: {invalid}.
    ///   Valid labels are: 0, 1, or 2.
    pub fn fix(&mut self, graph: &UndirectedGraph<u32>) {
        if self.neighbors_cache.is_none() {
            self.initialize_cache(graph);
        }

        let mut cache = self.neighbors_cache.take().unwrap();
        let mut modified = true;
        let mut visited = vec![false; self.genes.len()];

        while modified {
            modified = false;

            for vertex in graph.vertices() {
                let vertex_idx = *vertex as usize;
                if visited[vertex_idx] {
                    continue;
                }

                visited[vertex_idx] = true;
                let neighbors = cache.vertex_neighbors.get(vertex).unwrap();
                match self.genes.get(vertex_idx) {
                    Some(&0) => {
                        if !cache.has_two_neighbor[vertex_idx] {
                            if let Some(&neighbor_idx) =
                                neighbors.iter().find(|&&n| self.genes[n as usize] == 0)
                            {
                                self.genes[neighbor_idx as usize] = 2;
                                visited[neighbor_idx as usize] = false;
                                modified = true;
                            }
                        }
                    }
                    Some(&1 | &2) => {
                        if !cache.has_one_neighbor[vertex_idx] {
                            // Encontra o primeiro vizinho com valor 0 para atualizar para 1
                            if let Some(&neighbor_idx) =
                                neighbors.iter().find(|&&n| self.genes[n as usize] == 0)
                            {
                                self.genes[neighbor_idx as usize] = 1;
                                visited[neighbor_idx as usize] = false;
                                modified = true;
                            }
                        }
                    }

                    Some(&invalid) => {
                        panic!(
                        "Vértice com rótulo inválido encontrado! Índice: {vertex}, Valor: {invalid}. \
                            Os rótulos válidos são: 0, 1, ou 2."
                    );
                    }
                    None => {
                        panic!(
                            "Tentativa de acessar índice fora dos limites! Índice: {vertex}. \
                        Verifique se o vetor de genes está consistente com o grafo.",
                        );
                    }
                }
            }

            if modified {
                self.update_cache(&mut cache);
            }
        }
        self.neighbors_cache = Some(cache);
    }
}
