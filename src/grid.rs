//! グリッド(2 次元盤面)用ユーティリティ。

use std::ops::{Index, IndexMut};

/// 上下左右の 4 方向。座標が `usize` なら `wrapping_add_signed` と組で使い、
/// 負のはみ出しは巨大値へのラップを上限チェックで弾く。
/// 盤面を `Grid` で持っているなら `Grid::neighbors4` が同じことをしてくれる。
///
/// ```
/// use atcoder_rust::grid::DIRS4;
///
/// // 2x2 盤面の (0, 0) の隣接マスは (1, 0) と (0, 1)
/// let (y, x) = (0usize, 0usize);
/// let neighbors: Vec<_> = DIRS4
///     .into_iter()
///     .map(|(dy, dx)| (y.wrapping_add_signed(dy), x.wrapping_add_signed(dx)))
///     .filter(|&(ny, nx)| ny < 2 && nx < 2)
///     .collect();
/// assert_eq!(neighbors, vec![(1, 0), (0, 1)]);
/// ```
pub const DIRS4: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

/// 斜めを含む 8 方向。
///
/// ```
/// use atcoder_rust::grid::DIRS8;
///
/// // 3x3 盤面の中央 (1, 1) からは 8 マスすべてに行ける
/// let (y, x) = (1usize, 1usize);
/// let count = DIRS8
///     .into_iter()
///     .map(|(dy, dx)| (y.wrapping_add_signed(dy), x.wrapping_add_signed(dx)))
///     .filter(|&(ny, nx)| ny < 3 && nx < 3)
///     .count();
/// assert_eq!(count, 8);
/// ```
pub const DIRS8: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

/// h 行 × w 列の盤面。値は行優先で 1 本の `Vec` に持つ。
///
/// `Vec<Vec<T>>` と違って実体が平坦なので、`index` の通し番号をそのまま
/// Union-Find の頂点番号などに使える。マスの読み書きは `(r, c)` の添字で行う。
///
/// ```
/// use atcoder_rust::grid::Grid;
///
/// let mut dist = Grid::new(2, 3, usize::MAX);
/// assert_eq!(dist.len(), 6);
///
/// dist[(0, 0)] = 0;
/// // 盤内に収まる 4 近傍だけが返る（角のマスなら 2 方向）
/// for (nr, nc) in dist.neighbors4(0, 0) {
///     dist[(nr, nc)] = 1;
/// }
/// assert_eq!(dist[(1, 0)], 1);
/// assert_eq!(dist[(0, 1)], 1);
/// assert_eq!(dist[(1, 1)], usize::MAX);
///
/// // (r, c) と通し番号は index / coord で行き来する
/// assert_eq!(dist.index(1, 2), 5);
/// assert_eq!(dist.coord(5), (1, 2));
/// ```
pub struct Grid<T> {
    pub h: usize,
    pub w: usize,
    values: Vec<T>,
}

impl<T: Clone> Grid<T> {
    /// 全マスを `value` で埋めた h 行 × w 列の盤面を作る。
    pub fn new(h: usize, w: usize, value: T) -> Self {
        Self {
            h,
            w,
            values: vec![value; h * w],
        }
    }
}

impl<T> Grid<T> {
    /// マス数 h * w。Union-Find の要素数などに使う。
    ///
    /// ```
    /// use atcoder_rust::grid::Grid;
    /// assert_eq!(Grid::new(3, 4, 0).len(), 12);
    /// ```
    pub fn len(&self) -> usize {
        self.h * self.w
    }

    /// マスが 1 つも無い（h か w が 0）か。
    ///
    /// ```
    /// use atcoder_rust::grid::Grid;
    /// assert!(Grid::new(0, 5, 0).is_empty());
    /// assert!(!Grid::new(1, 1, 0).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// (r, c) を行優先の通し番号 r * w + c にする。
    ///
    /// ```
    /// use atcoder_rust::grid::Grid;
    ///
    /// let grid = Grid::new(2, 3, 0);
    /// // 行末と次の行頭は番号が連続するが、盤面上は隣接していない
    /// assert_eq!(grid.index(0, 2), 2);
    /// assert_eq!(grid.index(1, 0), 3);
    /// ```
    pub fn index(&self, r: usize, c: usize) -> usize {
        debug_assert!(
            r < self.h && c < self.w,
            "Grid::index: 盤外のマス (r, c) = ({r}, {c}), 盤面 = {}x{}",
            self.h,
            self.w
        );
        r * self.w + c
    }

    /// 通し番号を (r, c) に戻す（`index` の逆変換）。
    ///
    /// ```
    /// use atcoder_rust::grid::Grid;
    ///
    /// let grid = Grid::new(2, 3, 0);
    /// assert_eq!((0..grid.len()).map(|i| grid.coord(i)).collect::<Vec<_>>(),
    ///            vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)]);
    /// ```
    pub fn coord(&self, i: usize) -> (usize, usize) {
        debug_assert!(
            i < self.len(),
            "Grid::coord: 範囲外の通し番号 i = {i}, マス数 = {}",
            self.len()
        );
        (i / self.w, i % self.w)
    }

    /// 通し番号順に並んだ値のスライス。行ごとに見たいときは `chunks(grid.w)` と組む。
    ///
    /// ```
    /// use atcoder_rust::grid::Grid;
    ///
    /// let mut grid = Grid::new(2, 2, 0);
    /// grid[(1, 1)] = 7;
    /// assert_eq!(grid.values(), &[0, 0, 0, 7]);
    /// assert_eq!(grid.values().chunks(grid.w).collect::<Vec<_>>(), vec![&[0, 0], &[0, 7]]);
    /// ```
    pub fn values(&self) -> &[T] {
        &self.values
    }

    /// 上下左右のうち盤内に収まるマスを列挙する。負のはみ出しは
    /// `checked_add_signed` が None を返すので `?` で落ちる。
    ///
    /// 返すイテレータは h と w のコピーしか持たないため、**列挙しながら同じ盤面を
    /// 書き換えられる**（`use<T>` は edition 2024 でもそれを保つための指定）。
    ///
    /// ```
    /// use atcoder_rust::grid::Grid;
    ///
    /// // 1x1 盤面には隣が無い / 3x3 の中央からは 4 方向すべて
    /// assert_eq!(Grid::new(1, 1, 0).neighbors4(0, 0).count(), 0);
    /// assert_eq!(Grid::new(3, 3, 0).neighbors4(1, 1).count(), 4);
    /// ```
    pub fn neighbors4(&self, r: usize, c: usize) -> impl Iterator<Item = (usize, usize)> + use<T> {
        let (h, w) = (self.h, self.w);
        DIRS4.into_iter().filter_map(move |(dr, dc)| {
            let (nr, nc) = (r.checked_add_signed(dr)?, c.checked_add_signed(dc)?);
            (nr < h && nc < w).then_some((nr, nc))
        })
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (r, c): (usize, usize)) -> &T {
        &self.values[self.index(r, c)]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut T {
        let i = self.index(r, c);
        &mut self.values[i]
    }
}

/// 2 次元配列の行と列を入れ替える。元の `a[y][x]` が戻り値の `[x][y]` になる。
///
/// 「列ごとの処理」を「行ごとの処理」に読み替えたいときに使う。要素は move するので
/// `T: Clone` を要求しない代わりに元の配列を消費する（両方要るなら `transpose(a.clone())`）。
/// なお列の集計だけが目的なら、転置を作らず 1 行ずつ足し込むほうが速くメモリも食わない。
///
/// 行の長さが揃っていない入力は panic する（短いほうに黙って切り詰めない）。
///
/// ```
/// use atcoder_rust::grid::transpose;
///
/// // 2x3 → 3x2
/// assert_eq!(
///     transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
///     vec![vec![1, 4], vec![2, 5], vec![3, 6]]
/// );
///
/// // 文字グリッドを列ごとに見る
/// assert_eq!(
///     transpose(vec![vec!['a', 'b'], vec!['c', 'd']]),
///     vec![vec!['a', 'c'], vec!['b', 'd']]
/// );
///
/// assert!(transpose::<i32>(vec![]).is_empty());
/// ```
pub fn transpose<T>(a: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let (h, w) = (a.len(), a.first().map_or(0, Vec::len));
    a.into_iter().fold(
        (0..w).map(|_| Vec::with_capacity(h)).collect::<Vec<_>>(),
        |mut cols, row| {
            assert_eq!(
                row.len(),
                w,
                "transpose: 行の長さが揃っていない (先頭行の長さ = {w}, 行数 = {h})"
            );
            cols.iter_mut().zip(row).for_each(|(col, x)| col.push(x));
            cols
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![], vec![])] // 空
    #[case(vec![vec![1, 2, 3]], vec![vec![1], vec![2], vec![3]])] // 1xN → Nx1
    #[case(vec![vec![1], vec![2], vec![3]], vec![vec![1, 2, 3]])] // Nx1 → 1xN
    #[case(vec![vec![1, 2], vec![3, 4]], vec![vec![1, 3], vec![2, 4]])] // 正方
    #[case(vec![vec![1, 2, 3], vec![4, 5, 6]], vec![vec![1, 4], vec![2, 5], vec![3, 6]])] // 非正方（h と w の取り違え検出）
    #[case(vec![vec![], vec![], vec![]], vec![])] // 幅 0 の行だけ → 0 行
    fn transpose_cases(#[case] a: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        assert_eq!(transpose(a), expected);
    }

    #[rstest]
    // 角: 2 方向だけ
    #[case(3, 3, 0, 0, vec![(1, 0), (0, 1)])]
    #[case(3, 3, 2, 2, vec![(1, 2), (2, 1)])]
    // 辺の途中: 3 方向
    #[case(3, 3, 0, 1, vec![(1, 1), (0, 0), (0, 2)])]
    // 中央: 4 方向
    #[case(3, 3, 1, 1, vec![(0, 1), (2, 1), (1, 0), (1, 2)])]
    // 1x1: 隣なし
    #[case(1, 1, 0, 0, vec![])]
    // 横 1 列 / 縦 1 列（h と w の取り違え検出）
    #[case(1, 3, 0, 1, vec![(0, 0), (0, 2)])]
    #[case(3, 1, 1, 0, vec![(0, 0), (2, 0)])]
    fn neighbors4_cases(
        #[case] h: usize,
        #[case] w: usize,
        #[case] r: usize,
        #[case] c: usize,
        #[case] expected: Vec<(usize, usize)>,
    ) {
        let grid = Grid::new(h, w, 0);
        assert_eq!(grid.neighbors4(r, c).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn index_and_coord_are_inverse() {
        let grid = Grid::new(3, 4, 0);
        for r in 0..grid.h {
            for c in 0..grid.w {
                assert_eq!(grid.coord(grid.index(r, c)), (r, c));
            }
        }
        // 全マスの番号が 0..h*w を過不足なく覆う
        let ids: Vec<usize> = (0..grid.h)
            .flat_map(|r| (0..grid.w).map(move |c| (r, c)))
            .map(|(r, c)| grid.index(r, c))
            .collect();
        assert_eq!(ids, (0..grid.len()).collect::<Vec<_>>());
    }

    #[test]
    fn index_mut_writes_through_to_values() {
        // (r, c) 添字での書き込みが通し番号どおりの位置に入る
        let mut grid = Grid::new(2, 3, 0);
        grid[(0, 0)] = 1;
        grid[(1, 2)] = 9;
        assert_eq!(grid.values(), &[1, 0, 0, 0, 0, 9]);
        assert_eq!(grid[(1, 2)], 9);
    }

    #[test]
    fn neighbors4_allows_mutating_the_same_grid() {
        // 近傍を列挙しながら同じ盤面を書き換えられる（BFS の基本形）
        let mut grid = Grid::new(3, 3, 0);
        for (nr, nc) in grid.neighbors4(1, 1) {
            grid[(nr, nc)] = 1;
        }
        assert_eq!(grid.values(), &[0, 1, 0, 1, 0, 1, 0, 1, 0]);
    }

    #[test]
    #[should_panic(expected = "Grid::index")]
    fn index_rejects_out_of_board() {
        // debug ビルドでは盤外を黙って別のマスに丸めない
        Grid::new(2, 3, 0).index(0, 3);
    }

    #[test]
    fn transpose_twice_is_identity() {
        let a = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(transpose(transpose(a.clone())), a);
    }

    #[test]
    fn transpose_moves_non_clone_values() {
        // Clone を要求しないので、Copy でない値もそのまま移せる
        let a = vec![vec!["a".to_string(), "b".to_string()]];
        assert_eq!(
            transpose(a),
            vec![vec!["a".to_string()], vec!["b".to_string()]]
        );
    }

    #[test]
    #[should_panic(expected = "transpose")]
    fn transpose_rejects_jagged() {
        transpose(vec![vec![1, 2], vec![3]]);
    }
}
