use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum GraphError {
    VertexAlreadyExists,
    VertexNotFound,
    EdgeAlreadyExists,
    EdgeNotFound,
    SelfLoopNotAllowed,
}

/// Simple Graph
#[derive(Debug, Clone)]
pub struct SimpleGraph {
    pub adjacency_list: HashMap<usize, HashSet<usize>>,
}

impl SimpleGraph {
    pub fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: usize) -> Result<(), GraphError> {
        if self.adjacency_list.contains_key(&vertex) {
            return Err(GraphError::VertexAlreadyExists);
        }
        self.adjacency_list.insert(vertex, HashSet::new());
        Ok(())
    }

    pub fn add_edge(&mut self, u: usize, v: usize) -> Result<(), GraphError> {
        if u == v {
            return Err(GraphError::SelfLoopNotAllowed);
        }

        if !self.adjacency_list.contains_key(&u) || !self.adjacency_list.contains_key(&v) {
            return Err(GraphError::VertexNotFound);
        }

        if self.adjacency_list[&u].contains(&v) {
            return Err(GraphError::EdgeAlreadyExists);
        }

        self.adjacency_list.get_mut(&u).unwrap().insert(v);
        self.adjacency_list.get_mut(&v).unwrap().insert(u);

        Ok(())
    }

    pub fn remove_edge(&mut self, u: usize, v: usize) -> Result<(), GraphError> {
        if !self.adjacency_list.contains_key(&u) || !self.adjacency_list.contains_key(&v) {
            return Err(GraphError::VertexNotFound);
        }

        if !self.adjacency_list[&u].contains(&v) {
            return Err(GraphError::EdgeNotFound);
        }

        self.adjacency_list.get_mut(&u).unwrap().remove(&v);
        self.adjacency_list.get_mut(&v).unwrap().remove(&u);
        Ok(())
    }

    pub fn remove_vertex(&mut self, vertex: usize) -> Result<(), GraphError> {
        if !self.adjacency_list.contains_key(&vertex) {
            return Err(GraphError::VertexNotFound);
        }

        if let Some(neighbors) = self.adjacency_list.remove(&vertex) {
            for &neighbor in &neighbors {
                if let Some(neighbors_set) = self.adjacency_list.get_mut(&neighbor) {
                    neighbors_set.remove(&vertex);
                }
            }
        }
        Ok(())
    }

    pub fn neighbors(&self, vertex: usize) -> Result<&HashSet<usize>, GraphError> {
        self.adjacency_list
            .get(&vertex)
            .ok_or(GraphError::VertexNotFound)
    }

    pub fn has_edge(&self, u: usize, v: usize) -> bool {
        self.adjacency_list
            .get(&u)
            .map_or(false, |neighbors| neighbors.contains(&v))
    }

    pub fn vertex_count(&self) -> usize {
        self.adjacency_list.len()
    }

    pub fn edge_count(&self) -> usize {
        self.adjacency_list
            .values()
            .map(|neighbors| neighbors.len())
            .sum::<usize>()
            / 2
    }

    pub fn get_isolated_vertices(&self) -> HashSet<usize> {
        self.adjacency_list
            .iter()
            .filter(|(_, neighbors)| neighbors.is_empty())
            .map(|(&vertex, _)| vertex)
            .collect()
    }

    pub fn is_isolated(&self, vertex: usize) -> Result<bool, GraphError> {
        self.adjacency_list
            .get(&vertex)
            .map(|neighbors| neighbors.is_empty())
            .ok_or(GraphError::VertexNotFound)
    }

    pub fn from_edges(edges: Vec<(usize, usize)>) -> Result<Self, GraphError> {
        let mut graph = SimpleGraph::new();

        for (u, v) in edges {
            if !graph.adjacency_list.contains_key(&u) {
                graph.add_vertex(u)?;
            }
            if !graph.adjacency_list.contains_key(&v) {
                graph.add_vertex(v)?;
            }
            graph.add_edge(u, v)?;
        }

        Ok(graph)
    }
}

impl Default for SimpleGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vertex() {
        let mut graph = SimpleGraph::new();
        assert!(graph.add_vertex(1).is_ok());
        assert!(graph.add_vertex(1).is_err());
    }

    #[test]
    fn test_add_edge() {
        let mut graph = SimpleGraph::new();
        graph.add_vertex(1).unwrap();
        graph.add_vertex(2).unwrap();
        assert!(graph.add_edge(1, 2).is_ok());
        assert!(graph.add_edge(1, 2).is_err());
        assert!(graph.add_edge(1, 1).is_err());
    }

    #[test]
    fn test_remove_edge() {
        let mut graph = SimpleGraph::new();
        graph.add_vertex(1).unwrap();
        graph.add_vertex(2).unwrap();
        graph.add_edge(1, 2).unwrap();
        assert!(graph.remove_edge(1, 2).is_ok());
        assert!(graph.remove_edge(1, 2).is_err());
    }

    #[test]
    fn test_remove_vertex() {
        let mut graph = SimpleGraph::new();
        graph.add_vertex(1).unwrap();
        graph.add_vertex(2).unwrap();
        graph.add_edge(1, 2).unwrap();
        assert!(graph.remove_vertex(1).is_ok());
        assert!(graph.remove_vertex(1).is_err());
    }

    #[test]
    fn test_neighbors() {
        let mut graph = SimpleGraph::new();
        graph.add_vertex(1).unwrap();
        graph.add_vertex(2).unwrap();
        graph.add_edge(1, 2).unwrap();
        assert_eq!(graph.neighbors(1).unwrap().len(), 1);
        assert!(graph.neighbors(3).is_err());
    }
}
