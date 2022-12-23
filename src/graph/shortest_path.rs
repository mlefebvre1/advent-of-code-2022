pub trait ShortestPath {
    fn disjktra_shortest_path(&self, src: usize, dst: usize) -> isize;
}
