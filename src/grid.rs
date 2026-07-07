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
