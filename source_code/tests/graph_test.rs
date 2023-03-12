use rstest::{fixture, rstest};

#[path = "../src/model/mod.rs"]
mod model;
use model::graph::Graph;
use model::matrix::Matrix;

impl Default for Graph {
    fn default() -> Self {
        Self {
            matrix: Matrix::new(10),
            n_vertices: 10,
        }
    }
}


#[test]
fn graph_instance_success() {
    let mut graph = Graph::default();
    assert_eq!(graph.get_count_edges(), 0);
    assert_eq!(graph.n_vertices, 10);
}

#[test]
fn graph_add_edge_success() {
    let mut graph = Graph::default();
    graph.add_edge(1,2);
    assert_eq!(graph.get_count_edges(), 1);
}

#[test]
fn ponder_edge_succes() {
    let mut graph = Graph::default();
    graph.add_edge(1,2);
    graph.ponder_edge(1,2, 10);
    assert_eq!(graph.get_count_edges(), 1);
}

#[test]
fn graph_rm_edge_success() {
    let mut graph = Graph::default();
    graph.add_edge(1,2);
    graph.rm_edge(1,2);
    assert_eq!(graph.get_count_edges(), 0);
}

#[test]
fn verify_vertice_existance_success(){
    let graph = Graph::default();
    assert_eq!(graph.exist_vertice(3), Ok(3));
}

#[test]
fn verify_vertice_existance_failure(){
    let graph = Graph::default();
    assert_eq!(graph.exist_vertice(12), Err(-1));
}

#[test]
fn ponder_vertice_success(){
    let mut graph = Graph::default();
    graph.ponder_vertice(9, 10);
    assert_eq!(graph.get_vertice_weight(9), 10);
}
