use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::Dfs;
use std::collections::HashMap;

pub struct MyGraph {
    graph: Graph<&'static str, ()>,
    node_indices: HashMap<&'static str, NodeIndex>,
}

impl MyGraph {
    pub fn new() -> Self {
        let mut graph = Graph::<&'static str, ()>::new();
        let mut node_indices = HashMap::new();

        // Adiciona os nós
        for &name in &["1", "2", "3", "4", "5", "6"] {
            let idx = graph.add_node(name);
            node_indices.insert(name, idx);
        }

        // Adiciona as arestas para formar um grafo conectado
        let edges = vec![
            ("1", "2"),
            ("1", "3"),
            ("2", "4"),
            ("2", "5"),
            ("3", "6"),
        ];

        for (a, b) in edges {
            let a_idx = node_indices[a];
            let b_idx = node_indices[b];
            graph.add_edge(a_idx, b_idx, ());
        }

        Self { graph, node_indices }
    }

    pub fn dfs_from_node1(&self) -> Vec<&'static str> {
        let start = self.node_indices["1"];
        let mut dfs = Dfs::new(&self.graph, start);
        let mut visited = Vec::new();

        while let Some(nx) = dfs.next(&self.graph) {
            visited.push(self.graph[nx]);
        }

        visited
    }
}

// -----------------------------------------------------------
// TESTES
// -----------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_visits_all_nodes() {
        let g = MyGraph::new();
        let result = g.dfs_from_node1();

        let mut sorted_result = result.clone();
        sorted_result.sort();

        let mut expected = vec!["1", "2", "3", "4", "5", "6"];
        expected.sort();

        assert_eq!(sorted_result, expected, "Todos os nós devem ser visitados");
    }

    #[test]
    fn test_dfs_starts_at_node1() {
        let g = MyGraph::new();
        let result = g.dfs_from_node1();
        assert_eq!(result.first(), Some(&"1"), "DFS deve começar pelo nó 1");
    }
}
