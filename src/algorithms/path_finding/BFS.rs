use std::collections::{HashMap, HashSet, VecDeque};
use crate::graph::NodeId;
use super::{Pathfinder, PathResult};
use super::common_pathfinder::build_adjacency_list;

pub struct BfsPathfinder;

impl<N, E> Pathfinder<N, E> for BfsPathfinder{
    fn find_path(&self, 
                 graph: &crate::graph::Graph<N, E>, 
                 start: NodeId, 
                 target: NodeId) -> Option<PathResult>{
        let adj = build_adjacency_list(graph);

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut came_from = HashMap::new();
        
        queue.push_back(start);
        visited.insert(start);

        let mut found = false;
        while let Some(current) = queue.pop_front(){
            if current == target{
                found = true;
                break;
            }

            for &(next, _) in &adj[current]{
                if visited.insert(next){
                    came_from.insert(next, current);
                    queue.push_back(next);
                }
            }
        }

        if !found{
            return None;
        }
        
        let mut path = Vec::new();
        let mut current = target;
        while current != start{

            path.push(current);
            current = came_from[&current];
        }
        path.push(start);
        path.reverse();

        Some(PathResult{
                path: path.clone(),
                total_cost : (path.len() -1) as f64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Importiert die zentrale Test-Suite aus dem Nachbarmodul
    use super::super::path_tests::run_standard_pathfinder_tests;

    #[test]
    fn run_bfs_standard_tests() {
        run_standard_pathfinder_tests(BfsPathfinder);
    }
}
