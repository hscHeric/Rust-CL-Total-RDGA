use petgraph::graph::UnGraph;

/// Estrutura que representa um cromossomo no CL-Total-RDGA.
///
/// Cada cromossomo armazena uma configuração de rótulos \{0, 1, 2\} para os vértices de um grafo,
/// com o objetivo de satisfazer as condições de dominação romana total.
///
/// # Campos
/// - `genes: Vec<u8>`: Vetor que armazena os rótulos de cada vértice no grafo.
///   - `0`: Tem que ser um vértice rotulado com valor 2 na sua vizinhança.
///   - `1 | 2`: Tem que ter um vértice rotulado com um f > 0 em sua vizinhança.
///
/// # Exemplo
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
    /// Cria um novo cromossomo a partir de um vetor de genes.
    ///
    /// # Parâmetros
    /// - `genes: Vec<u8>`: O vetor contendo os rótulos iniciais dos vértices.
    ///
    /// # Retorno
    /// - Retorna uma nova instância de `Chromosome`.
    ///
    /// # Exemplo
    /// ```rust
    /// let genes = vec![0, 1, 2];
    /// let chromosome = Chromosome::new(genes);
    /// ```
    #[inline]
    pub fn new(genes: Vec<u8>) -> Self {
        Self { genes }
    }

    /// Calcula o valor de "fitness" (aptidão) do cromossomo.
    ///
    /// O fitness é definido como o peso total da função de dominação romana total,
    /// que é a soma de todos os valores no vetor de genes.
    ///
    /// # Retorno
    /// - Um valor do tipo `u32`, correspondente à soma dos genes.
    ///
    /// # Exemplo
    /// ```rust
    /// let chromosome = Chromosome::new(vec![1, 2, 0, 1]);
    /// assert_eq!(chromosome.fitness(), 4); // Soma dos genes
    /// ```
    #[inline]
    pub fn fitness(&self) -> u32 {
        self.genes.iter().map(|&x| x as u32).sum()
    }

    /// Retorna um slice contendo os genes do cromossomo.
    ///
    /// # Retorno
    /// - Um slice do vetor de genes (`&[u8]`).
    ///
    /// # Exemplo
    /// ```rust
    /// let genes = vec![0, 1, 2];
    /// let chromosome = Chromosome::new(genes.clone());
    /// assert_eq!(chromosome.genes(), &genes);
    /// ```
    #[inline]
    pub fn genes(&self) -> &[u8] {
        &self.genes
    }

    /// Corrige os genes do cromossomo com base nas relações de um grafo.
    ///
    /// A função aplica as regras da dominação romana total para garantir que o
    /// vetor de genes respeite as condições do problema.
    ///
    /// ### Regras Aplicadas:
    /// 1. **Vértices com rótulo `0`**:
    ///    - Se nenhum dos vizinhos tiver rótulo `2`, o primeiro vizinho com rótulo `0` é alterado para `2`.
    /// 2. **Vértices com rótulo `1` ou `2`**:
    ///    - Se nenhum dos vizinhos tiver rótulo maior que `0`, o primeiro vizinho com rótulo `0` é alterado para `1`.
    /// 3. **Rótulos inválidos**:
    ///    - A função lança um `panic!` se algum gene possuir um valor inválido.
    ///
    /// ### Parâmetros:
    /// - `graph: &UnGraph<usize, ()>`:
    ///   Um grafo não direcionado representando as relações entre os vértices.
    ///
    /// ### Retorno:
    /// - Nenhum. Os genes são corrigidos diretamente no vetor `genes`.
    ///
    /// ### Exemplo
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
    pub fn fix(&mut self, graph: &UnGraph<usize, ()>) {
        let mut visited = vec![false; self.genes.len()];

        while visited.iter().any(|&n| !n) {
            for vertex in graph.node_indices() {
                if visited[vertex.index()] {
                    continue;
                }

                visited[vertex.index()] = true;
                let neighbors: Vec<_> = graph.neighbors(vertex).collect();

                match self.genes[vertex.index()] {
                    0 => {
                        // Verifica se existe vizinho com rótulo 2
                        if !neighbors.iter().any(|&n| self.genes[n.index()] == 2) {
                            // Seleciona o primeiro vizinho com rótulo 0 e o rotula como 2
                            if let Some(&neighbor) =
                                neighbors.iter().find(|&&n| self.genes[n.index()] == 0)
                            {
                                self.genes[neighbor.index()] = 2;
                                visited[neighbor.index()] = false;
                            }
                        }
                    }
                    1 | 2 => {
                        if !neighbors.iter().any(|&n| self.genes[n.index()] > 0) {
                            if let Some(&neighbor) =
                                neighbors.iter().find(|&&n| self.genes[n.index()] == 0)
                            {
                                self.genes[neighbor.index()] = 1;
                                visited[neighbor.index()] = false;
                            }
                        }
                    }
                    _ => {
                        panic!(
                            "Vértice com rótulo inválido encontrado! Índice: {}, Valor: {}. \
                        Os rótulos válidos são: 0, 1, ou 2.",
                            vertex.index(),
                            self.genes[vertex.index()]
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

        assert_eq!(vec![2, 2, 1, 0], chromosome.genes())
    }
}
