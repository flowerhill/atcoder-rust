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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn initially_disjoint() {
        let mut uf = UnionFind::new(5);
        for i in 0..5 {
            assert_eq!(uf.size(i), 1);
        }
        assert!(!uf.same_uf(0, 1));
        assert!(uf.same_uf(2, 2));
    }

    #[test]
    fn unite_merges_components() {
        let mut uf = UnionFind::new(5);
        uf.unite(0, 1);
        uf.unite(1, 2);
        assert!(uf.same_uf(0, 2));
        assert_eq!(uf.size(0), 3);
        assert_eq!(uf.size(2), 3);
        // 別グループは未連結
        assert!(!uf.same_uf(0, 3));
        assert_eq!(uf.size(3), 1);
    }

    #[test]
    fn unite_is_idempotent() {
        let mut uf = UnionFind::new(3);
        uf.unite(0, 1);
        uf.unite(0, 1); // 既に同一でもサイズは増えない
        assert_eq!(uf.size(0), 2);
    }

    // 辺を貼った後に期待される連結成分の数
    #[rstest]
    #[case(&[], 4)] // 連結なし → 4成分
    #[case(&[(0, 1)], 3)]
    #[case(&[(0, 1), (2, 3)], 2)]
    #[case(&[(0, 1), (1, 2), (2, 3)], 1)] // 全連結
    #[case(&[(0, 1), (0, 1)], 3)] // 重複辺は無視
    fn connected_component_count(#[case] edges: &[(usize, usize)], #[case] expected: usize) {
        let mut uf = UnionFind::new(4);
        for &(a, b) in edges {
            uf.unite(a, b);
        }
        let roots: std::collections::HashSet<usize> = (0..4).map(|i| uf.root(i)).collect();
        assert_eq!(roots.len(), expected);
    }
}
