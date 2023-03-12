pub struct Vertice {
    pub label: u32,
    pub weight: u32,
    pub edges: Vec<*mut Vertice>,
}
 
impl Vertice {
    pub fn new(label: u32, weight: u32, edges: Vec<*mut Vertice>) -> Self {
        Vertice { label, weight, edges }
    }
}