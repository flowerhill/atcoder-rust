use std::ops::{Add, AddAssign, Neg, Sub};

struct WeightedUnionFind<T: Clone> {
    parent: Vec<usize>,
    rank: Vec<usize>,
    diff_weight: Vec<T>,
}

impl<T> WeightedUnionFind<T>
where
    T: Copy + Clone + Add<Output = T> + AddAssign + Sub<Output = T> + Neg<Output = T>,
{
    fn new(size: usize, zero: T) -> Self {
        WeightedUnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
            diff_weight: vec![zero; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let root = self.find(self.parent[x]);
            let par_diff_weight = self.diff_weight[self.parent[x]];
            self.diff_weight[x] += par_diff_weight;
            self.parent[x] = root;
            root
        }
    }

    fn unite(&mut self, x: usize, y: usize, w: T) {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);
        if root_x == root_y {
            return;
        }

        let mut weight = w + self.diff_weight[x] - self.diff_weight[y];

        if self.rank[root_x] < self.rank[root_y] {
            (root_y, root_x) = (root_x, root_y);
            weight = -weight;
        }

        if self.rank[root_x] == self.rank[root_y] {
            self.rank[root_x] += 1;
        }

        self.parent[root_y] = root_x;
        self.diff_weight[root_y] = weight;
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn diff(&mut self, x: usize, y: usize) -> T {
        self.diff_weight[y] - self.diff_weight[x]
    }

    fn weight(&mut self, x: usize) -> T {
        self.find(x);
        self.diff_weight[x]
    }
}
