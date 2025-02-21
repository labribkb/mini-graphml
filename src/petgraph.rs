use std::collections::HashMap;

use petgraph;

use crate::Graph;

impl crate::GraphML<()> {
    pub fn into_petgraph(self) -> petgraph::Graph<String, String> {
        self.graph.into_petgraph()
    }
}

impl crate::Graph<()> {
    /// Build a standard petgraph.
    /// Consume the original graph by stealing the strings and no copy pasting them
    pub fn into_petgraph(self) -> petgraph::Graph<String, String> {
        let Graph { nodes, edges, .. } = self;
        let nodes = nodes.0;
        let edges = edges.0;

        let mut g = petgraph::Graph::<String, String>::new();
        let mut node_id_to_idx = HashMap::new();

        for (id, n) in nodes.into_iter() {
            let idx = g.add_node(id.to_owned());
            node_id_to_idx.insert(id, idx);
        }

        for (id, e) in edges.into_iter() {
            let id_src = e.src_id();
            let id_tgt = e.tgt_id();
            g.add_edge(
                *node_id_to_idx.get(id_src).unwrap(),
                *node_id_to_idx.get(id_tgt).unwrap(),
                id,
            );
        }

        g
    }
}
