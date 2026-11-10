use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::graph::{NodeId, EdgeWeight};
use super::{Pathfinder, PathResult};
use super::common_pathfinder::{build_adjacency_list, validate_nodes, reconstruct_path};

#[derive(Clone, PartialEq)]
struct State{
    cost : f64,
    node : NodeId,
}

impl Eq for State {}
impl Ord for State {
    fn cmp(&self, other : &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other : &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct DijkstraPathfinder;

impl<N, E: EdgeWeight> Pathfinder<N, E> for DijkstraPathfinder {
    fn find_path(
        &self,
        graph: &crate::graph::Graph<N, E>,
        start: NodeId,
        target: NodeId,
    ) -> Option<PathResult> {

        if !validate_nodes(graph, start, target){
            return None;
        }

        let adj = build_adjacency_list(graph);
        let nodes_len = graph.nodes().len();

        let mut dist = vec![f64::INFINITY; nodes_len];
        let mut came_from = HashMap::new();
        let mut heap = BinaryHeap::new();

        dist[start] = 0.0;
        heap.push(State {
            cost: 0.0,
            node: start,
        });

        while let Some(State { cost, node }) = heap.pop() {
            if node == target {
                break;
            }

            if cost > dist[node] {
                continue;
            }

            for &(next, edge_data) in &adj[node] {
                let weight = edge_data.weight();
                let next_cost = cost + weight;

                if next_cost < dist[next] {
                    dist[next] = next_cost;
                    came_from.insert(next, node);
                    heap.push(State {
                        cost: next_cost,
                        node: next,
                    });
                }
            }
        }

        if dist[target].is_infinite() {
            return None;
        }

        // Pfad rekonstruieren
        let path = reconstruct_path(start, target, &came_from)?;
        Some(PathResult {
            path,
            total_cost: dist[target],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::path_tests::run_standard_pathfinder_tests;

    #[test]
    fn run_dijkstra_standard_tests() {
        run_standard_pathfinder_tests(DijkstraPathfinder);
    }
}


