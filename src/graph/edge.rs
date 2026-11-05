use super::node::NodeId;

#[derive(Debug, Clone)]
pub struct Edge<E>{
    pub source : NodeId,
    pub target : NodeId,
    pub data : E,
}


