
#[path = "../src/model/mod.rs"]
mod model;
use model::graph::Graph;

#[test]
fn test_graph_instance_success() {
    let mut graph = Graph::new(10);
    assert_eq!(graph.get_count_edges(), 0);
    assert_eq!(graph.n_vertices, 10);
}