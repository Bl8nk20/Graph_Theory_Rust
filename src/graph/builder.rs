use std::collections::HashMap;
use super::{Graph, Node, NodeId, Edge};

pub struct GraphBuilder<N, E>{
    nodes_data : Vec<N>,
    edges_data : Vec<(String, String, E)>,
    key_to_id : HashMap<String, NodeId>,
}

impl <N, E> GraphBuilder<N, E>{
    pub fn new() -> Self{
        Self{
            nodes_data : Vec::new(),
            edges_data : Vec::new(),
            key_to_id : HashMap::new(),
        }
    }

    pub fn add_node(&mut self, 
                    key:impl Into<String>, 
                    data: N ) -> NodeId{
        let key = key.into();
        let id = self.nodes_data.len();
        
        self.nodes_data.push(data);
        self.key_to_id.insert(key, id);

        id
    }

    pub fn add_edge(&mut self,
                    source_key : impl Into<String>,
                    target_key : impl Into<String>,
                    data : E){
        self.edges_data.push((source_key.into(), target_key.into(), data));
    }

    pub fn build(self) -> Result<Graph<N, E>, String>{
        let nodes = self.nodes_data.into_iter()
                                   .enumerate()
                                   .map(|(id, data)| Node{id, data})
                                   .collect();
        let mut edges = Vec::with_capacity(self.edges_data.len());
        
        for (source_key, target_key, data) in self.edges_data{
            let source = *self.key_to_id
                              .get(&source_key)
                              .ok_or_else(|| format!("Quellknoten '{}' existiert nicht.", source_key))?;
            
            let target = *self.key_to_id
                              .get(&target_key)
                              .ok_or_else(|| format!("Zielknoten '{}' existiert nicht.", target_key))?;

            edges.push(Edge{
                source,
                target,
                data,
            });
        }
        Ok(Graph::new(nodes, edges))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_graph_build() {
        let mut builder = GraphBuilder::new();
        builder.add_node("Berlin", 100);
        builder.add_node("München", 200);
        builder.add_edge("Berlin", "München", 585);

        let graph = builder.build();
        assert!(graph.is_ok(), "Graph sollte erfolgreich gebaut werden");

        let graph = graph.unwrap();
        assert_eq!(graph.nodes().len(), 2);
        assert_eq!(graph.edges().len(), 1);
        
        // Prüfen, ob die Kante korrekt aufgelöst wurde
        let edge = &graph.edges()[0];
        assert_eq!(edge.data, 585);
    }

    #[test]
    fn test_missing_source_node_error() {
        let mut builder = GraphBuilder::new();
        // "Berlin" wurde nicht als Knoten hinzugefügt!
        builder.add_node("München", 200);
        builder.add_edge("Berlin", "München", 585);

        let result = builder.build();
        assert!(result.is_err(), "Bau des Graphen muss fehlschlagen, wenn der Quellknoten fehlt");
    }

    #[test]
    fn test_missing_target_node_error() {
        let mut builder = GraphBuilder::new();
        builder.add_node("Berlin", 100);
        // "München" fehlt als Zielknoten
        builder.add_edge("Berlin", "München", 585);

        let result = builder.build();
        assert!(result.is_err(), "Bau des Graphen muss fehlschlagen, wenn der Zielknoten fehlt");
    }
}
