pub mod parse;
pub mod print;

#[derive(Debug, Eq, PartialEq)]
pub struct GraphList {
    list: Vec<Vec<usize>>,
    gtype: GraphType,
    n: usize,
    m: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GraphType {
    Directed,
    Undirected,
} 

impl GraphList {
    pub fn new(n: usize, gtype: GraphType) -> Self {
        let mut list = Vec::new();
        list.resize_with(n + 1, Vec::new);
        Self { gtype, list, n , m : 0}
    }

    pub fn add_edge(&mut self, v: usize, u: usize) {
        match self.gtype {
            GraphType::Directed => self.add_directed(v, u),
            GraphType::Undirected => self.add_undirected(v, u),
        }
    }

    fn add_undirected(&mut self, v: usize, u: usize) {
        self.list[v].push(u);
        self.list[u].push(v);
        self.m += 1;
    }

    fn add_directed(&mut self, v: usize, u: usize) {
        self.list[v].push(u);
        self.m += 1;
    }

    pub fn neigh(&self, n: usize) -> &[usize] {
        &self.list[n]
    }

    pub fn list(&self) -> impl Iterator<Item = (usize, &Vec<usize>)> {
        self.list.iter().enumerate().skip(1)
    }

    pub fn list_mut(&mut self) -> impl Iterator<Item = (usize, &mut Vec<usize>)> {
        self.list.iter_mut().enumerate().skip(1)
    }

    pub fn v_size(&self) -> usize {
        self.n
    }

    pub fn e_size(&self) -> usize {
        self.m
    }

    pub fn g_type(&self) -> GraphType {
        self.gtype
    }
}