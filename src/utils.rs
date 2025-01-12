use kambo_graph::{graphs::simple::UndirectedGraph, Graph, GraphError, GraphMut};

/// Constrói um grafo não direcionado (`UndirectedGraph`) a partir de uma lista de arestas.
///
/// # Argumentos
///
/// * `edges` - Um vetor de arestas representadas como tuplas `(u, v, Option<()>)`,
///   - `u` e `v` são os vértices conectados por uma aresta.
///   - O terceiro elemento da tupla (`Option<()>`) é ignorado, pois o grafo não é ponderado.
///
/// # Retorno
///
/// Retorna um `Result` contendo:
/// - `Ok(UndirectedGraph<usize>)` se o grafo for construído com sucesso.
/// - `Err(GraphError)` se ocorrer algum erro durante a construção (por exemplo, ao adicionar vértices ou arestas).
///
/// # Comportamento
///
/// - Adiciona os vértices `u` e `v` ao grafo se ainda não existirem.
/// - Adiciona uma aresta entre `u` e `v` somente se a aresta ainda não existir no grafo.
///
/// # Erros
///
/// Esta função pode retornar:
/// - `GraphError::VertexAlreadyExists` se a tentativa de adicionar um vértice duplicado falhar.
/// - `GraphError::EdgeAlreadyExists` se a tentativa de adicionar uma aresta duplicada falhar.

pub fn build_graph_from_edges(
    edges: Vec<(usize, usize, Option<i32>)>,
) -> Result<UndirectedGraph<usize>, GraphError> {
    let mut graph: UndirectedGraph<usize> = UndirectedGraph::new_undirected();

    for (u, v, _) in edges {
        if !graph.contains_vertex(&u) {
            graph.add_vertex(u)?;
        }

        if !graph.contains_vertex(&v) {
            graph.add_vertex(v)?;
        }

        if !graph.contains_edge(&u, &v) {
            graph.add_edge(&u, &v)?;
        }
    }
    Ok(graph)
}
