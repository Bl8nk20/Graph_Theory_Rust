use crate::graph::{Graph, NodeId};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PathResult{
    pub path : Vec<NodeId>,
    pub total_cost : f64,
}

pub trait Pathfinder<N, E>{
    fn find_path(&self, 
                 graph : &Graph<N, E>, 
                 start: NodeId, 
                 end: NodeId) -> Option<PathResult>;
}

pub fn validate_nodes<N, E>(graph: &Graph<N, E>, 
                            start: NodeId, 
                            target: NodeId) -> bool{
    let nodes_len = graph.nodes().len();

    start < nodes_len && target < nodes_len
}

pub(in crate::algorithms::path_finding) fn build_adjacency_list<N, E>(graph : &Graph<N, E>) -> Vec<Vec<(NodeId, &E)>>{
    let mut adj = vec![Vec::new(); graph.nodes().len()];

    for edge in graph.edges(){
        if edge.source < adj.len() && edge.target  < adj.len(){
            adj[edge.source].push((edge.target, &edge.data));
        }
    }

    adj
}

pub fn reconstruct_path(start: NodeId, 
                        target: NodeId, 
                        came_from: &HashMap<NodeId, NodeId>) -> Option<Vec<NodeId>> {
    let mut path = Vec::new();
    let mut current = target;
    
    while current != start {
        path.push(current);
        current = *came_from.get(&current)?;
    }
    path.push(start);
    path.reverse();
    
    Some(path)
}
