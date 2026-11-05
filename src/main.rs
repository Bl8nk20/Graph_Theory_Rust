mod graph;
use graph::GraphBuilder;

fn main() {
    let mut builder = GraphBuilder::new();

    // Knoten hinzufügen (z. B. mit Städtenamen als Key und Gewichten)
    builder.add_node("Berlin", "Hauptstadt");
    builder.add_node("München", "Süd-Metropole");

    // Kanten hinzufügen
    builder.add_edge("Berlin", "München", 585); // z.B. Entfernung in km

    let graph = builder.build().unwrap();

    println!("Graph erfolgreich gebaut mit {} Knoten und {} Kanten!", 
        graph.nodes().len(), graph.edges().len());
}
