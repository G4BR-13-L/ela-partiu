

mod model;
pub use crate::model::vertice::*;
pub use crate::model::graph::*;

fn main() {
    println!("Hello, world!");

    let mut ver = Vertice::new(1,1, vec![]);
    let mut graph = Graph::new(10);

    println!("{}", ver.label);
    println!("{}", graph.matrix.n_vertices);
    graph.matrix.get_count_edges();
    graph.add_edge(1,2);
    graph.matrix.get_count_edges();

}