use std::ops::{Add, AddAssign, Neg, Sub};

/// 重み付き Union-Find。各要素に「根からの相対重み」を持たせ、
/// 同じ集合に属する 2 要素間の重みの差を管理する。
pub struct WeightedUnionFind<T: Clone> {
    parent: Vec<usize>,
    rank: Vec<usize>,
    diff_weight: Vec<T>,
}

impl<T> WeightedUnionFind<T>
where
    T: Copy + Clone + Add<Output = T> + AddAssign + Sub<Output = T> + Neg<Output = T>,
{
    /// `size` 個の要素を、重み 0(= `zero`)の独立した集合として初期化する。
    pub fn new(size: usize, zero: T) -> Self {
        WeightedUnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
            diff_weight: vec![zero; size],
        }
    }

    /// `x` の根を返す。経路圧縮しつつ累積重みを根からの相対重みへ更新する。
    pub fn find(&mut self, x: usize) -> usize {
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

    /// `weight(y) - weight(x) == w` となるように `x` と `y` の集合を併合する(union by rank)。
    pub fn unite(&mut self, x: usize, y: usize, w: T) {
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

    /// `x` と `y` が同じ集合に属するか判定する。
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// 同じ集合内での `weight(y) - weight(x)` を返す。
    pub fn diff(&mut self, x: usize, y: usize) -> T {
        self.diff_weight[y] - self.diff_weight[x]
    }

    /// `x` の根からの相対重みを返す。
    pub fn weight(&mut self, x: usize) -> T {
        self.find(x);
        self.diff_weight[x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn diff_within_same_group() {
        // weight(y) - weight(x) = w となるように unite(x, y, w)
        let mut uf = WeightedUnionFind::new(5, 0i64);
        uf.unite(0, 1, 3); // 1 は 0 より 3 大きい
        uf.unite(1, 2, 5); // 2 は 1 より 5 大きい
        assert!(uf.is_same(0, 2));
        assert_eq!(uf.diff(0, 1), 3);
        assert_eq!(uf.diff(1, 2), 5);
        assert_eq!(uf.diff(0, 2), 8); // 推移的に 3 + 5
        assert_eq!(uf.diff(2, 0), -8); // 逆向きは符号反転
    }

    #[test]
    fn separate_groups_not_same() {
        let mut uf = WeightedUnionFind::new(5, 0i64);
        uf.unite(0, 1, 2);
        uf.unite(2, 3, 4);
        assert!(!uf.is_same(0, 2));
        assert!(uf.is_same(2, 3));
    }

    // 連鎖 0->1->2->3 の重みから任意ペアの diff を検証
    #[rstest]
    #[case(0, 3, 1 + 2 + 3)]
    #[case(1, 3, 2 + 3)]
    #[case(0, 2, 1 + 2)]
    #[case(3, 0, -(1 + 2 + 3))]
    fn diff_along_chain(#[case] x: usize, #[case] y: usize, #[case] expected: i64) {
        let mut uf = WeightedUnionFind::new(4, 0i64);
        uf.unite(0, 1, 1);
        uf.unite(1, 2, 2);
        uf.unite(2, 3, 3);
        assert_eq!(uf.diff(x, y), expected);
    }

    #[test]
    fn unite_already_connected_is_noop() {
        let mut uf = WeightedUnionFind::new(3, 0i64);
        uf.unite(0, 1, 5);
        uf.unite(0, 1, 999); // 既に同一グループなので無視される
        assert_eq!(uf.diff(0, 1), 5);
    }
}
