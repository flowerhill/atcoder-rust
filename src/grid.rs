//! グリッド(2 次元盤面)用ユーティリティ。

/// 上下左右の 4 方向。座標が `usize` なら `wrapping_add_signed` と組で使い、
/// 負のはみ出しは巨大値へのラップを上限チェックで弾く。
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
