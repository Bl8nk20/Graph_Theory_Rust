
pub type NodeId = usize;

#[derive(Debug, Clone)]
pub struct Node<N>{
    pub id: NodeId,
    pub data: N,
}

