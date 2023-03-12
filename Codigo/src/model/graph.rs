use std::string;

use crate::model::matrix::Matrix;

pub struct Graph{
    pub matrix: Matrix,
}

impl Graph{
    pub fn new( n_vertices: usize ) -> Self {
        Graph { matrix: Matrix::new(n_vertices) }
    }
}