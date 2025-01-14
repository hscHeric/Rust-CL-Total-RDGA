/// Módulo que contém a implementação da estrutura `Chromosome`.
///
/// Este módulo define a estrutura `Chromosome`, que é usada para representar
/// um cromossomo no contexto de algoritmos genéticos. A estrutura inclui
/// métodos para criação, avaliação (fitness) e correção (`fix`) com base em um
/// grafo não direcionado.
///
/// # Estrutura do Módulo
/// - **`Chromosome`**: Estrutura principal que armazena os genes e possui métodos associados.
/// - **Métodos Principais**:
///     - `new`: Cria um novo cromossomo.
///     - `fitness`: Calcula a aptidão (`fitness`) do cromossomo.
///     - `genes`: Retorna os genes do cromossomo.
///     - `fix`: Corrige os genes do cromossomo com base em um grafo.
///
/// # Exemplo de Uso
/// ```rust
/// use crate::chromosome::Chromosome;
/// use petgraph::graph::UnGraph;
///
/// // Criar um cromossomo
/// let mut chromosome = Chromosome::new(vec![0, 0, 0]);
///
/// // Criar um grafo simples
/// let mut graph = UnGraph::<usize, ()>::new_undirected();
/// let v0 = graph.add_node(0);
/// let v1 = graph.add_node(1);
/// let v2 = graph.add_node(2);
/// graph.add_edge(v0, v1, ());
/// graph.add_edge(v1, v2, ());
///
/// // Corrigir o cromossomo
/// chromosome.fix(&graph);
///
/// // Verificar os genes corrigidos
/// println!("{:?}", chromosome.genes());
/// ```
pub mod chromosome;

pub use chromosome::Chromosome;
