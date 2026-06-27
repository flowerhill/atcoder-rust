use itertools::Itertools;

// union find
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let parent = (0..n).collect_vec();
        let size = vec![1; n];
        Self { parent, size }
    }

    pub fn root(&mut self, x: usize) -> usize {
        let idx = x;
        let px = self.parent[idx];

        if px == x {
            x
        } else {
            // 経路圧縮
            self.parent[idx] = self.root(px);
            self.parent[idx]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let px = self.root(x);
        let py = self.root(y);
        if px == py {
            return;
        }

        if self.size[px] < self.size[py] {
            self.parent[px] = py;
            self.size[py] += self.size[px];
        } else {
            self.parent[py] = px;
            self.size[px] += self.size[py];
        }
    }

    pub fn same_uf(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn size(&mut self, x: usize) -> usize {
        let px = self.root(x);
        self.size[px]
    }
}
