use std::ops::{Add, Neg, Sub};

/// 和の制約 `x_a + x_b = c` を管理する符号付き Union-Find。
///
/// 各要素を「根の値 `x_root` の一次式」
/// `x_i = offset_i + sign_i * x_root`（`sign_i ∈ {+1, -1}`）として保持する。
/// 差 `x_j - x_i = w` を扱う [`crate::wunionfind::WeightedUnionFind`] と対になる型で、
/// 和の辺 `x_a + x_b = c` は `x_b = c - x_a` と符号が反転するため、各要素に
/// 符号 `sign_i` を持たせている。
///
/// 同一集合内では、片方の値を固定すればもう片方は必ず一意に定まる（[`resolve`]）。
/// 別集合どうしの関係は定まらない。
///
/// [`resolve`]: Self::resolve
pub struct SumUnionFind<T: Clone> {
    parent: Vec<usize>,
    size: Vec<usize>,
    /// 根に対する定数項 `offset_i`
    offset: Vec<T>,
    /// 根に対する符号が `-1` かどうか（`true` なら `x_i = offset_i - x_root`）
    neg: Vec<bool>,
}

impl<T> SumUnionFind<T>
where
    T: Copy + Clone + Add<Output = T> + Sub<Output = T> + Neg<Output = T>,
{
    /// `size` 個の要素を、それぞれ独立した集合として初期化する。
    /// `zero` は `offset` の初期値（加法単位元）。
    pub fn new(size: usize, zero: T) -> Self {
        SumUnionFind {
            parent: (0..size).collect(),
            size: vec![1; size],
            offset: vec![zero; size],
            neg: vec![false; size],
        }
    }

    /// `neg` が真なら符号反転（`sign * v` に相当）。
    fn signed(neg: bool, v: T) -> T {
        if neg {
            -v
        } else {
            v
        }
    }

    /// `x` の根を返す。経路圧縮しつつ `offset`/`neg` を根からの相対値へ畳み込む。
    /// 再帰せず反復で行うため、深い連結でもスタックを消費しない。
    pub fn find(&mut self, x: usize) -> usize {
        let mut path = Vec::new();
        let mut cur = x;
        while self.parent[cur] != cur {
            path.push(cur);
            cur = self.parent[cur];
        }
        let root = cur;
        // 根に近い順（親が先に圧縮済みになる順）に畳み込む。
        // x_u = offset_u + sign_u * x_p, x_p = offset_p + sign_p * x_root
        for &u in path.iter().rev() {
            let p = self.parent[u];
            self.offset[u] = self.offset[u] + Self::signed(self.neg[u], self.offset[p]);
            self.neg[u] ^= self.neg[p];
            self.parent[u] = root;
        }
        root
    }

    /// 和の制約 `x_a + x_b == c` を追加する（union by size）。
    /// 既に同一集合なら何もしない（入力が無矛盾である前提）。
    pub fn unite(&mut self, a: usize, b: usize, c: T) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return;
        }
        let (oa, na) = (self.offset[a], self.neg[a]);
        let (ob, nb) = (self.offset[b], self.neg[b]);
        // x_a + x_b = c と x_a = oa + sa*x_ra, x_b = ob + sb*x_rb より
        //   x_rb = sb*(c - oa - ob) + (-sa*sb)*x_ra       （1/sb = sb）
        // 符号 -sa*sb は na と nb が一致するとき -1 なので neg = (na == nb)。
        let base = c - oa - ob;
        let flipped = na == nb;
        if self.size[ra] >= self.size[rb] {
            self.parent[rb] = ra;
            self.offset[rb] = Self::signed(nb, base);
            self.neg[rb] = flipped;
            self.size[ra] += self.size[rb];
        } else {
            self.parent[ra] = rb;
            self.offset[ra] = Self::signed(na, base);
            self.neg[ra] = flipped;
            self.size[rb] += self.size[ra];
        }
    }

    /// `x` と `y` が同じ集合に属するか判定する。
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// `x_a = val` と仮定したときの `x_b` を返す。
    /// 同一集合なら必ず一意に定まり、別集合なら `None`。
    ///
    /// ```
    /// use atcoder_rust::sumunionfind::SumUnionFind;
    /// let mut uf = SumUnionFind::new(4, 0i64);
    /// uf.unite(0, 1, 3); // x0 + x1 = 3
    /// assert_eq!(uf.resolve(0, 1, 1), Some(2)); // x0=1 なら x1=2
    /// assert_eq!(uf.resolve(2, 3, 5), None);    // 別集合
    /// uf.unite(2, 3, 6); // x2 + x3 = 6
    /// assert_eq!(uf.resolve(2, 3, 5), Some(1)); // x2=5 なら x3=1
    /// assert_eq!(uf.resolve(0, 0, 33), Some(33)); // a == b は val をそのまま返す
    /// ```
    pub fn resolve(&mut self, a: usize, b: usize, val: T) -> Option<T> {
        if self.find(a) != self.find(b) {
            return None;
        }
        // x_a = oa + sa*x_root = val  =>  x_root = sa*(val - oa)
        let x_root = Self::signed(self.neg[a], val - self.offset[a]);
        Some(self.offset[b] + Self::signed(self.neg[b], x_root))
    }

    /// `x_a + x_b` が値として確定するなら返す（同一集合かつ符号が逆のとき）。
    /// 別集合、または符号が揃って和が `x_root` に依存する場合は `None`。
    ///
    /// ```
    /// use atcoder_rust::sumunionfind::SumUnionFind;
    /// let mut uf = SumUnionFind::new(3, 0i64);
    /// uf.unite(0, 1, 3);  // x0 + x1 = 3
    /// assert_eq!(uf.sum(0, 1), Some(3));
    /// uf.unite(1, 2, 10); // x1 + x2 = 10 -> x0 と x2 は符号が揃う
    /// assert_eq!(uf.sum(0, 2), None);     // x0 + x2 は x_root が残り不定
    /// assert_eq!(uf.sum(1, 2), Some(10));
    /// ```
    pub fn sum(&mut self, a: usize, b: usize) -> Option<T> {
        if self.find(a) != self.find(b) || self.neg[a] == self.neg[b] {
            return None;
        }
        Some(self.offset[a] + self.offset[b])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // typical90_068 サンプル1 の流れをそのまま検証する。
    #[test]
    fn sample1() {
        let mut uf = SumUnionFind::new(4, 0i64);
        uf.unite(0, 1, 3); // 0 1 2 3
        assert_eq!(uf.resolve(0, 1, 1), Some(2)); // 1 1 2 1 -> 2
        assert_eq!(uf.resolve(2, 3, 5), None); // 1 3 4 5 -> Ambiguous
        uf.unite(2, 3, 6); // 0 3 4 6
        assert_eq!(uf.resolve(2, 3, 5), Some(1)); // 1 3 4 5 -> 1
        uf.unite(1, 2, 6); // 0 2 3 6
        assert_eq!(uf.resolve(2, 0, 5), Some(2)); // 1 3 1 5 -> 2
    }

    // 和の鎖 x0+x1=10, x1+x2=3, x2+x3=8 => x1=10-x0, x2=x0-7, x3=15-x0
    #[rstest]
    #[case(0, 1, 4, 6)] // x0=4 => x1=6
    #[case(0, 2, 4, -3)] // x0=4 => x2=-3
    #[case(0, 3, 4, 11)] // x0=4 => x3=11
    #[case(3, 0, 11, 4)] // 逆向き x3=11 => x0=4
    fn chain(#[case] a: usize, #[case] b: usize, #[case] val: i64, #[case] expected: i64) {
        let mut uf = SumUnionFind::new(4, 0i64);
        uf.unite(0, 1, 10);
        uf.unite(1, 2, 3);
        uf.unite(2, 3, 8);
        assert_eq!(uf.resolve(a, b, val), Some(expected));
    }

    // 符号が逆になるペアだけ和が確定する。
    #[test]
    fn sum_defined_only_for_opposite_sign() {
        let mut uf = SumUnionFind::new(3, 0i64);
        uf.unite(0, 1, 10); // x0 と x1 は符号逆 -> 和確定
        uf.unite(1, 2, 3); // x0 と x2 は符号一致 -> 和不定
        assert_eq!(uf.sum(0, 1), Some(10));
        assert_eq!(uf.sum(1, 2), Some(3));
        assert_eq!(uf.sum(0, 2), None);
    }

    #[test]
    fn separate_groups() {
        let mut uf = SumUnionFind::new(4, 0i64);
        uf.unite(0, 1, 5);
        uf.unite(2, 3, 7);
        assert!(!uf.is_same(0, 2));
        assert_eq!(uf.resolve(0, 2, 1), None);
        assert_eq!(uf.sum(0, 2), None);
    }

    #[test]
    fn unite_already_connected_is_noop() {
        let mut uf = SumUnionFind::new(3, 0i64);
        uf.unite(0, 1, 5);
        uf.unite(0, 1, 999); // 既に同一集合なので無視される
        assert_eq!(uf.sum(0, 1), Some(5));
    }
}
