//! いもす法（区間・矩形への一括加算）。

/// 1 次元いもす法: 閉区間 `[l, r]` への加算クエリをまとめて処理し、
/// 長さ `n` の各点の合計値を返す。`O(クエリ数 + n)`。
///
/// クエリは `(l, r, v)`。`r < n` であること。
///
/// ```
/// use atcoder_rust::imos::imos_1d;
///
/// // [0, 2] に +1、[1, 3] に +2
/// assert_eq!(imos_1d(5, [(0, 2, 1), (1, 3, 2)]), vec![1, 3, 3, 2, 0]);
///
/// // r が末尾ぴったりでもよい
/// assert_eq!(imos_1d(3, [(1, 2, 10)]), vec![0, 10, 10]);
///
/// // クエリ 0 件なら全部 0
/// let empty: [(usize, usize, i64); 0] = [];
/// assert_eq!(imos_1d(4, empty), vec![0, 0, 0, 0]);
/// ```
pub fn imos_1d(n: usize, queries: impl IntoIterator<Item = (usize, usize, i64)>) -> Vec<i64> {
    // 閉区間なので r+1 に打ち消しを書ける余白を取る
    let mut acc = vec![0i64; n + 1];
    for (l, r, v) in queries {
        acc[l] += v;
        acc[r + 1] -= v;
    }

    let mut sum = 0;
    for x in acc.iter_mut() {
        sum += *x;
        *x = sum;
    }

    acc.truncate(n);
    acc
}

/// 2 次元いもす法: 閉矩形 `[r1, r2] × [c1, c2]` への加算クエリをまとめて処理し、
/// `h × w` の各セルの合計値を返す。`O(クエリ数 + h * w)`。
///
/// クエリは `((r1, c1), (r2, c2), v)` = (成分ごとに小さいほうの角, 大きいほうの角, 加算値)。
/// `r1 <= r2 < h`, `c1 <= c2 < w` であること。
///
/// ```
/// use atcoder_rust::imos::imos_2d;
///
/// // 左上 2x2 と右下 2x2 に +1 ずつ。重なる中央のセルだけ 2 になる
/// let grid = imos_2d(3, 3, [((0, 0), (1, 1), 1), ((1, 1), (2, 2), 1)]);
/// assert_eq!(grid, vec![vec![1, 1, 0], vec![1, 2, 1], vec![0, 1, 1]]);
///
/// // 角が格子の端ぴったりでもよい。v は負でもよい
/// let grid = imos_2d(2, 2, [((0, 0), (1, 1), 5), ((1, 1), (1, 1), -2)]);
/// assert_eq!(grid, vec![vec![5, 5], vec![5, 3]]);
/// ```
pub fn imos_2d(
    h: usize,
    w: usize,
    queries: impl IntoIterator<Item = ((usize, usize), (usize, usize), i64)>,
) -> Vec<Vec<i64>> {
    // 閉矩形なので (r2+1, c2+1) に打ち消しを書ける余白を取る
    let mut grid = vec![vec![0i64; w + 1]; h + 1];
    for ((r1, c1), (r2, c2), v) in queries {
        grid[r1][c1] += v;
        grid[r1][c2 + 1] -= v;
        grid[r2 + 1][c1] -= v;
        grid[r2 + 1][c2 + 1] += v;
    }

    // 横方向の累積和
    for row in grid.iter_mut() {
        let mut sum = 0;
        for x in row.iter_mut() {
            sum += *x;
            *x = sum;
        }
    }

    // 縦方向の累積和（1 つ上の行を足し込む）
    for r in 1..=h {
        let (upper, lower) = grid.split_at_mut(r);
        for (x, &p) in lower[0].iter_mut().zip(upper[r - 1].iter()) {
            *x += p;
        }
    }

    grid.truncate(h);
    for row in grid.iter_mut() {
        row.truncate(w);
    }
    grid
}
