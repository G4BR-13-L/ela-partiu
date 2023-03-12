use std::string;

use crate::model::matrix::Matrix;

pub struct Graph {
    pub matrix: Matrix,
    pub n_vertices: u32,
}

impl Graph {
    pub fn new(n_vertices: u32) -> Self {
        Graph {
            matrix: Matrix::new(n_vertices),
            n_vertices: n_vertices,
        }
    }

    pub fn get_count_edges(&self) -> u32 {
        return self.matrix.get_count_edges();
    }

    pub fn add_edge(&mut self, v: u32, w: u32) {
        self.matrix.add_edge(v, w);
    }

    pub fn ponder_edge(&mut self, v: u32, w: u32, weight: u32) {
        self.matrix.ponder_edge(v, w, weight);
    }

    pub fn rm_edge(&mut self, v: u32, w: u32) {
        self.matrix.rm_edge(v, w);
    }

    pub fn ponder_vertice(&mut self, v: u32, weight: u32) {
        self.matrix.ponder_vertice(v, weight);
    }

    pub fn exist_vertice(&self, v: u32) -> Result<u32, i32> {
        return self.matrix.exist_vertice(v);
    }

    pub fn get_vertice_weight(&self, v: u32) -> u32 {
        return self.matrix.get_vertice_weight(v);
    }
}
