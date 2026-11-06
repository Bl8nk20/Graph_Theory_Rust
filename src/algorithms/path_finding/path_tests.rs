use super::Pathfinder;
use crate::graph::GraphBuilder;

/// Zentrale Test-Suite für alle Pathfinder-Algorithmen
pub fn run_standard_pathfinder_tests<P>(pathfinder: P)
where
    P: Pathfinder<&'static str, ()>,
{
    test_start_equals_target(&pathfinder);
    test_no_path_exists(&pathfinder);
    test_cyclic_graph(&pathfinder);
    test_diamond_graph_shortest_path(&pathfinder);
}

fn test_start_equals_target<P>(pf: &P)
where
    P: Pathfinder<&'static str, ()>,
{
    let mut builder: GraphBuilder<&str, ()> = GraphBuilder::new();
    builder.add_node("A", "Start"); // Zweiter Parameter ist Typ N (&str)
    let graph = builder.build().unwrap();

    let result = pf.find_path(&graph, 0, 0);
    assert!(result.is_some(), "Start gleich Ziel muss einen Pfad liefern");
    let res = result.unwrap();
    assert_eq!(res.path, vec![0]);
    assert_eq!(res.total_cost, 0.0);
}

fn test_no_path_exists<P>(pf: &P)
where
    P: Pathfinder<&'static str, ()>,
{
    let mut builder: GraphBuilder<&str, ()> = GraphBuilder::new();
    builder.add_node("A", "Node A");
    builder.add_node("B", "Node B");
    let graph = builder.build().unwrap();

    let result = pf.find_path(&graph, 0, 1);
    assert!(result.is_none(), "Isolierte Knoten dürfen keinen Pfad haben");
}

fn test_cyclic_graph<P>(pf: &P)
where
    P: Pathfinder<&'static str, ()>,
{
    let mut builder: GraphBuilder<&str, ()> = GraphBuilder::new();
    builder.add_node("A", "A");
    builder.add_node("B", "B");
    builder.add_node("C", "C");
    builder.add_node("D", "D");

    builder.add_edge("A", "B", ());
    builder.add_edge("B", "C", ());
    builder.add_edge("C", "A", ());
    builder.add_edge("B", "D", ());

    let graph = builder.build().unwrap();
    let result = pf.find_path(&graph, 0, 3);

    assert!(result.is_some());
    let res = result.unwrap();
    assert_eq!(res.path, vec![0, 1, 3]);
}

fn test_diamond_graph_shortest_path<P>(pf: &P)
where
    P: Pathfinder<&'static str, ()>,
{
    let mut builder: GraphBuilder<&str, ()> = GraphBuilder::new();
    builder.add_node("A", "A");
    builder.add_node("B", "B");
    builder.add_node("C", "C");
    builder.add_node("D", "D");

    builder.add_edge("A", "B", ());
    builder.add_edge("B", "D", ());
    builder.add_edge("A", "C", ());
    builder.add_edge("C", "D", ());
    builder.add_edge("A", "D", ());

    let graph = builder.build().unwrap();
    let result = pf.find_path(&graph, 0, 3);

    assert!(result.is_some());
    let res = result.unwrap();
    assert_eq!(res.path, vec![0, 3]);
}
