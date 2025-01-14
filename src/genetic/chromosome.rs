use petgraph::graph::UnGraph;

/// Structure representing a chromosome in the CL-Total-RDGA.
///
/// Each chromosome stores a configuration of labels \{0, 1, 2\} for the vertices of a graph,
/// with the goal of satisfying the conditions of total Roman domination.
///
/// # Fields
/// - `genes: Vec<u8>`: A vector that stores the labels for each vertex in the graph.
///   - `0`: Must have a vertex labeled with the value `2` in its neighborhood.
///   - `1 | 2`: Must have a vertex labeled with `f > 0` in its neighborhood.
///
/// # Example
/// ```rust
/// let genes = vec![0, 1, 2];
/// let chromosome = Chromosome::new(genes);
/// println!("{:?}", chromosome.genes());
/// ```

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<u8>,
}

impl Chromosome {
    /// Creates a new chromosome from a vector of genes.
    ///
    /// # Parameters
    /// - `genes: Vec<u8>`: The vector containing the initial labels for the vertices.
    ///
    /// # Returns
    /// - Returns a new instance of `Chromosome`.
    ///
    /// # Example
    /// ```rust
    /// let genes = vec![0, 1, 2];
    /// let chromosome = Chromosome::new(genes);
    /// ```

    #[inline]
    #[must_use]
    pub fn new(genes: Vec<u8>) -> Self {
        Self { genes }
    }

    /// Calculates the "fitness" value of the chromosome.
    ///
    /// The fitness is defined as the total weight of the total Roman domination function,
    /// which is the sum of all values in the gene vector.
    ///
    /// # Returns
    /// - A `u32` value corresponding to the sum of the genes.
    ///
    /// # Example
    /// ```rust
    /// let chromosome = Chromosome::new(vec![1, 2, 0, 1]);
    /// assert_eq!(chromosome.fitness(), 4); // Sum of the genes
    /// ```

    #[inline]
    #[must_use]
    pub fn fitness(&self) -> u32 {
        self.genes.iter().map(|&x| u32::from(x)).sum()
    }

    /// Returns a slice containing the genes of the chromosome.
    ///
    /// # Returns
    /// - A slice of the gene vector (`&[u8]`).
    ///
    /// # Example
    /// ```rust
    /// let genes = vec![0, 1, 2];
    /// let chromosome = Chromosome::new(genes.clone());
    /// assert_eq!(chromosome.genes(), &genes);
    /// ```

    #[inline]
    #[must_use]
    pub fn genes(&self) -> &[u8] {
        &self.genes
    }

    /// Adjusts the chromosome's genes based on the relationships in the given graph.
    ///
    /// This function enforces the rules of the Total Roman Domination problem to ensure
    /// the `genes` vector satisfies the problem's constraints.
    ///
    /// # Rules Applied:
    /// 1. **Vertices with label `0`**:
    ///    - If none of its neighbors have the label `2`, the first neighbor with label `0`
    ///      is updated to `2`.
    /// 2. **Vertices with label `1` or `2`**:
    ///    - If none of its neighbors have a label greater than `0`, the first neighbor with
    ///      label `0` is updated to `1`.
    /// 3. **Invalid labels**:
    ///    - The function will panic if any gene in the `genes` vector has a value other than
    ///      `0`, `1`, or `2`.
    ///
    /// # Panics
    /// This function will panic in the following situations:
    /// 1. **Invalid label in the `genes` vector**:
    ///    - If the `genes` vector contains a value other than `0`, `1`, or `2`.
    ///    - Panic message: `"Invalid label found! Index: <index>, Value: <value>. \
    ///      Valid labels are: 0, 1, or 2."`
    ///
    /// 2. **Index out of bounds**:
    ///    - If the graph has more vertices than the size of the `genes` vector, leading to
    ///      an out-of-bounds access.
    ///    - Panic message: `"Index out of bounds! Index: <index>. Ensure the `genes` vector \
    ///      is consistent with the graph."`
    ///
    /// # Parameters
    /// - `graph: &UnGraph<usize, ()>`:
    ///   An undirected graph representing the relationships between vertices.
    ///
    /// # Returns
    /// - None. The `genes` vector is updated in place to reflect the adjustments.
    ///
    /// # Examples
    /// ```rust
    /// use petgraph::graph::UnGraph;
    /// let mut graph = UnGraph::<usize, ()>::new_undirected();
    /// let v0 = graph.add_node(0);
    /// let v1 = graph.add_node(1);
    /// let v2 = graph.add_node(2);
    /// graph.add_edge(v0, v1, ());
    /// graph.add_edge(v1, v2, ());
    ///
    /// let mut chromosome = Chromosome::new(vec![0, 0, 1]);
    /// chromosome.fix(&graph);
    /// println!("{:?}", chromosome.genes());
    /// ```
    ///
    /// # Notes
    /// - Ensure that the size of the `genes` vector matches the number of vertices in the graph
    ///   (`graph.node_count()`).
    /// - Verify that all values in the `genes` vector are either `0`, `1`, or `2` before calling this function.

    pub fn fix(&mut self, graph: &UnGraph<usize, ()>) {
        let mut visited = vec![false; self.genes.len()];

        while visited.iter().any(|&n| !n) {
            for vertex in graph.node_indices() {
                if visited[vertex.index()] {
                    continue;
                }

                visited[vertex.index()] = true;
                let neighbors: Vec<_> = graph.neighbors(vertex).collect();

                // Usa o método `get` ao invés de indexação direta
                match self.genes.get(vertex.index()) {
                    Some(&0) => {
                        // Verifica se existe vizinho com rótulo 2
                        if !neighbors
                            .iter()
                            .any(|&n| self.genes.get(n.index()) == Some(&2))
                        {
                            // Seleciona o primeiro vizinho com rótulo 0 e o rotula como 2
                            if let Some(&neighbor) = neighbors
                                .iter()
                                .find(|&&n| self.genes.get(n.index()) == Some(&0))
                            {
                                if let Some(gene) = self.genes.get_mut(neighbor.index()) {
                                    *gene = 2;
                                    visited[neighbor.index()] = false;
                                }
                            }
                        }
                    }
                    Some(&1 | &2) => {
                        if !neighbors
                            .iter()
                            .any(|&n| self.genes.get(n.index()).map_or(false, |&v| v > 0))
                        {
                            if let Some(&neighbor) = neighbors
                                .iter()
                                .find(|&&n| self.genes.get(n.index()) == Some(&0))
                            {
                                if let Some(gene) = self.genes.get_mut(neighbor.index()) {
                                    *gene = 1;
                                    visited[neighbor.index()] = false;
                                }
                            }
                        }
                    }
                    Some(&invalid) => {
                        panic!(
                            "Vértice com rótulo inválido encontrado! Índice: {}, Valor: {}. \
                        Os rótulos válidos são: 0, 1, ou 2.",
                            vertex.index(),
                            invalid
                        );
                    }
                    None => {
                        panic!(
                            "Tentativa de acessar índice fora dos limites! Índice: {}. \
                        Verifique se o vetor de genes está consistente com o grafo.",
                            vertex.index()
                        );
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fitness_calculation() {
        let chromosome = Chromosome::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(chromosome.fitness(), 15);
    }

    #[test]
    fn test_empty_chromosome() {
        let chromosome = Chromosome::new(vec![]);
        assert_eq!(chromosome.fitness(), 0);
    }

    #[test]
    fn test_genes_access() {
        let genes = vec![1, 2, 3, 4, 5];
        let chromosome = Chromosome::new(genes.clone());
        assert_eq!(chromosome.genes(), &genes);
    }
    #[test]
    fn test_fix() {
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

        assert_eq!(vec![2, 2, 1, 0], chromosome.genes());
    }
}
