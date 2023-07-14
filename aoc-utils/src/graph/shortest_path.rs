use ndarray::Array2;

pub trait ShortestPath {
    fn disjktra_shortest_path(&self, src: usize, dst: usize) -> isize;
}

pub fn floyd_warshal(w: &mut Array2<usize>) {
    let n = w.shape()[0];

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if w[[i, j]] > w[[i, k]].saturating_add(w[[k, j]]) {
                    w[[i, j]] = w[[i, k]].saturating_add(w[[k, j]]);
                }
            }
        }
    }
}
