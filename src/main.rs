mod graph;
use graph::GraphBuilder;

mod algorithms;
use algorithms::{Pathfinder, BfsPathfinder};

fn main() {
    let mut builder = GraphBuilder::new();

    builder.add_node("A", "Start");
    builder.add_node("B", "Mitte");
    builder.add_node("C", "Ziel");
    builder.add_node("D", "T");
    builder.add_node("E", "R");

    builder.add_edge("A", "B", 10.0);
    builder.add_edge("B", "C", 5.0);
    builder.add_edge("A", "C", 20.0);
    builder.add_edge("B", "D", 4.0);
    builder.add_edge("D", "C", 4.0);
    builder.add_edge("D", "E", 2.0);

    let graph = builder.build().unwrap();

    println!("Graph erfolgreich gebaut mit {} Knoten und {} Kanten!", 
        graph.nodes().len(), graph.edges().len());

    let bfs = BfsPathfinder;
    if let Some(res) = bfs.find_path(&graph, 0, 4) {
        println!("BFS Pfad: {:?}, Kosten: {}", res.path, res.total_cost);
    }
}
