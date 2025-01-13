use std::collections::HashMap;

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

    // Criamos um conjunto de vértices únicos para adicionar primeiro
    let mut vertices: Vec<usize> = edges.iter().flat_map(|(u, v, _)| vec![*u, *v]).collect();
    vertices.sort_unstable();
    vertices.dedup();

    // Adicionamos todos os vértices primeiro
    for &vertex in &vertices {
        if !graph.contains_vertex(&vertex) {
            graph.add_vertex(vertex)?;
        }
    }

    // Adicionamos todas as arestas
    for (u, v, _) in edges {
        if !graph.contains_edge(&u, &v) {
            graph.add_edge(&u, &v)?;
        }
    }

    Ok(graph)
}

/// Normaliza um grafo não direcionado renumerando seus vértices de forma sequencial.
///
/// Esta função recebe um grafo não direcionado existente e produz um novo grafo
/// onde os vértices são renumerados para inteiros de `0` a `n-1`, onde `n` é a
/// ordem do grafo (número de vértices). A renumeração é feita com base na ordem
/// de iteração sobre os vértices no grafo original.
///
/// A estrutura do grafo, incluindo as arestas, é preservada durante a normalização.
///
/// # Parâmetros
/// - `graph`: Referência para o grafo não direcionado a ser normalizado.
///
/// # Retorno
/// Um novo grafo `UndirectedGraph<usize>` com os vértices renumerados e as
/// arestas ajustadas de acordo.
///
/// # Complexidade
/// - **Tempo**: \(O(V + E)\), onde \(V\) é o número de vértices e \(E\) é o número de arestas.
/// - **Espaço**: \(O(V + E)\), já que a função cria um novo grafo e um mapeamento de vértices.
///
/// # Panics
/// Esta função pode gerar pânico caso alguma operação no novo grafo (como adicionar
/// um vértice ou uma aresta) falhe, o que pode ocorrer devido a limitações na implementação
/// ou restrições específicas do grafo.
pub fn normalize_graph(graph: &UndirectedGraph<usize>) -> UndirectedGraph<usize> {
    let mut normalized_graph = UndirectedGraph::new_undirected();
    let mut vertex_mapping = HashMap::new();

    // Primeiro passo: criar todos os vértices e o mapeamento
    for (new_index, vertex) in graph.vertices().enumerate() {
        vertex_mapping.insert(*vertex, new_index);
        // Podemos usar unwrap aqui pois sabemos que os índices são únicos
        normalized_graph.add_vertex(new_index).unwrap();
    }

    // Segundo passo: adicionar todas as arestas
    let mut edges_to_add = Vec::new();

    for vertex in graph.vertices() {
        if let Some(neighbors) = graph.neighbors(vertex) {
            let new_u = vertex_mapping[vertex];
            for neighbor in neighbors {
                let new_v = vertex_mapping[neighbor];
                // Evitamos adicionar a mesma aresta duas vezes
                if new_u < new_v && !normalized_graph.contains_edge(&new_u, &new_v) {
                    edges_to_add.push((new_u, new_v));
                }
            }
        }
    }

    // Adicionamos todas as arestas de uma vez
    for (new_u, new_v) in edges_to_add {
        normalized_graph.add_edge(&new_u, &new_v).unwrap();
    }

    normalized_graph
}
