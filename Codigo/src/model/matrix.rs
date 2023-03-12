use std::string;

pub struct Matrix {
    pub matrix: Vec<Vec<usize>>,
    pub vertice_weights: Vec<usize>,
    pub n_vertices: usize,
}

impl Matrix {
    pub fn new(n_vertices: usize) -> Self {
        Matrix {
            matrix: vec![vec![0; n_vertices]; n_vertices],
            vertice_weights: vec![n_vertices],
            n_vertices,
        }


    }
}
