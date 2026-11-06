use crate::graph::{Graph, NodeId};

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

pub(in crate::algorithms::path_finding) fn build_adjacency_list<N, E>(graph : &Graph<N, E>) -> Vec<Vec<(NodeId, &E)>>{
    let mut adj = vec![Vec::new(); graph.nodes().len()];

    for edge in graph.edges(){
        if edge.source < adj.len() && edge.target  < adj.len(){
            adj[edge.source].push((edge.target, &edge.data));
        }
    }

    adj
}
