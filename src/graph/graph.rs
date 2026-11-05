use super::node::Node;
use super::edge::Edge;

#[derive(Debug, Clone)]
pub struct Graph<N, E> {
    nodes: Vec<Node<N>>,
    edges: Vec<Edge<E>>,
}

impl<N, E> Graph<N, E> {
    pub(crate) fn new(nodes: Vec<Node<N>>, edges: Vec<Edge<E>>) -> Self {
        Self { nodes, edges }
    }

    pub fn nodes(&self) -> &[Node<N>] {
        &self.nodes
    }

    pub fn edges(&self) -> &[Edge<E>] {
        &self.edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::builder::GraphBuilder;

    #[test]
    fn test_graph_getters_and_integrity() {
        // Wir nutzen den Builder, um einen validen Graphen zu erzeugen
        let mut builder = GraphBuilder::new();
        builder.add_node("A", "Node A Data");
        builder.add_node("B", "Node B Data");
        builder.add_edge("A", "B", "Edge A-B");

        let graph = builder.build().expect("Graph sollte bauen");

        // Teste Knoten-Zugriffe
        let nodes = graph.nodes();
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].id, 0);
        assert_eq!(nodes[0].data, "Node A Data");
        assert_eq!(nodes[1].id, 1);
        assert_eq!(nodes[1].data, "Node B Data");

        // Teste Kanten-Zugriffe
        let edges = graph.edges();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].source, 0); // "A" hat ID 0
        assert_eq!(edges[0].target, 1); // "B" hat ID 1
        assert_eq!(edges[0].data, "Edge A-B");
    }

    #[test]
    fn test_empty_graph() {
        // Ein leerer Graph über den Builder (falls erlaubt)
        let builder: GraphBuilder<(), ()> = GraphBuilder::new();
        let graph = builder.build().expect("Leerer Graph sollte bauen");

        assert_eq!(graph.nodes().len(), 0);
        assert_eq!(graph.edges().len(), 0);
    }
}
