use std::string;

pub struct Matrix {
    pub matrix: Vec<Vec<u32>>,
    pub vertice_weights: Vec<u32>,
    pub n_vertices: u32,
}

impl Matrix {
    pub fn new(n_vertices: u32) -> Self {
        Matrix {
            matrix: vec![vec![0u32; n_vertices as usize]; n_vertices as usize],
            vertice_weights: vec![n_vertices as u32],
            n_vertices: n_vertices as u32,
        }
    }

    pub fn get_count_edges(&self) -> u32{
        let mut count: u32 = 0;
        for i in self.matrix.iter() {
            for &j in i.iter() {
                if j != 0 {
                    count += 1;
                }
            }
        }
        return count;
    }

    pub fn add_edge(&mut self, v: u32, w: u32) {
        self.matrix[v as usize][w as usize] = 1;
    }

    pub fn ponder_edge(&mut self, v: u32, w: u32, weight: u32) {
        self.matrix[v as usize][w as usize] = weight;
    }

    pub fn rm_edge(&mut self, v: u32, w: u32) {
        self.matrix[v as usize][w as usize] = 0;
    }
}
