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

/// 2 次元いもす法の本体。矩形加算を差分として溜め、`build` で累積和を取ってグリッドにする。
///
/// 加算クエリを 1 つのイテレータにまとめにくい場面（条件分岐しながら足す、
/// 複数の入力を読みながら足す等）向け。単にクエリ列を渡すだけなら関数版の
/// [`imos_2d`]（閉矩形）/ [`imos_2d_half_open`]（半開矩形）を使う。
///
/// ```
/// use atcoder_rust::imos::Imos2D;
///
/// let mut imos = Imos2D::new(3, 3);
/// imos.add_rect((0, 0), (2, 2), 1); // 半開矩形 [0, 2) x [0, 2)
/// imos.add_rect_inclusive((1, 1), (2, 2), 1); // 閉矩形 [1, 2] x [1, 2]（同じ範囲）
/// assert_eq!(imos.build(), vec![vec![1, 1, 0], vec![1, 2, 1], vec![0, 1, 1]]);
/// ```
pub struct Imos2D {
    h: usize,
    w: usize,
    diff: Vec<Vec<i64>>,
}

impl Imos2D {
    /// `h × w` のグリッドへの矩形加算を受け付ける状態を作る。
    pub fn new(h: usize, w: usize) -> Self {
        // 右端・下端の 1 つ外側に打ち消しを書ける余白を取る
        Self {
            h,
            w,
            diff: vec![vec![0i64; w + 1]; h + 1],
        }
    }

    /// 半開矩形 `[r1, r2) × [c1, c2)` に `v` を加算する。`O(1)`。
    ///
    /// `r1 <= r2 <= h`, `c1 <= c2 <= w` であること。
    /// `r1 == r2` や `c1 == c2`（幅・高さ 0）は「何も加算しない」として扱う。
    pub fn add_rect(&mut self, (r1, c1): (usize, usize), (r2, c2): (usize, usize), v: i64) {
        debug_assert!(
            r1 <= r2 && r2 <= self.h && c1 <= c2 && c2 <= self.w,
            "Imos2D::add_rect: 矩形が範囲外 (r1, c1) = {:?}, (r2, c2) = {:?}, h = {}, w = {}",
            (r1, c1),
            (r2, c2),
            self.h,
            self.w
        );

        self.diff[r1][c1] += v;
        self.diff[r1][c2] -= v;
        self.diff[r2][c1] -= v;
        self.diff[r2][c2] += v;
    }

    /// 閉矩形 `[r1, r2] × [c1, c2]` に `v` を加算する。`O(1)`。
    ///
    /// `r1 <= r2 < h`, `c1 <= c2 < w` であること。
    pub fn add_rect_inclusive(
        &mut self,
        (r1, c1): (usize, usize),
        (r2, c2): (usize, usize),
        v: i64,
    ) {
        self.add_rect((r1, c1), (r2 + 1, c2 + 1), v);
    }

    /// 累積和を取り、`h × w` の各セルの合計値を返す。`O(h * w)`。
    pub fn build(self) -> Vec<Vec<i64>> {
        let Self { h, w, mut diff } = self;

        // 横方向の累積和
        for row in diff.iter_mut() {
            let mut sum = 0;
            for x in row.iter_mut() {
                sum += *x;
                *x = sum;
            }
        }

        // 縦方向の累積和（1 つ上の行を足し込む）
        for r in 1..=h {
            let (upper, lower) = diff.split_at_mut(r);
            for (x, &p) in lower[0].iter_mut().zip(upper[r - 1].iter()) {
                *x += p;
            }
        }

        diff.truncate(h);
        for row in diff.iter_mut() {
            row.truncate(w);
        }
        diff
    }
}

/// 2 次元いもす法（閉矩形版）: 閉矩形 `[r1, r2] × [c1, c2]` への加算クエリをまとめて処理し、
/// `h × w` の各セルの合計値を返す。`O(クエリ数 + h * w)`。
///
/// クエリは `((r1, c1), (r2, c2), v)` = (成分ごとに小さいほうの角, 大きいほうの角, 加算値)。
/// `r1 <= r2 < h`, `c1 <= c2 < w` であること。
/// 入力が `[l, r)` の半開区間で与えられるなら [`imos_2d_half_open`] を使う。
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
    let mut imos = Imos2D::new(h, w);
    for (top_left, bottom_right, v) in queries {
        imos.add_rect_inclusive(top_left, bottom_right, v);
    }
    imos.build()
}

/// 2 次元いもす法（半開矩形版）: 半開矩形 `[r1, r2) × [c1, c2)` への加算クエリをまとめて処理し、
/// `h × w` の各セルの合計値を返す。`O(クエリ数 + h * w)`。
///
/// クエリは `((r1, c1), (r2, c2), v)` = (含む側の角, 含まない側の角, 加算値)。
/// `r1 <= r2 <= h`, `c1 <= c2 <= w` であること。
/// 「左下 (lx, ly)・右上 (rx, ry) の長方形」のように端が半開で与えられる入力を、
/// `-1` の補正なしにそのまま渡せる。
///
/// ```
/// use atcoder_rust::imos::imos_2d_half_open;
///
/// // 左上 2x2 と右下 2x2 に +1 ずつ。重なる中央のセルだけ 2 になる
/// let grid = imos_2d_half_open(3, 3, [((0, 0), (2, 2), 1), ((1, 1), (3, 3), 1)]);
/// assert_eq!(grid, vec![vec![1, 1, 0], vec![1, 2, 1], vec![0, 1, 1]]);
///
/// // 幅 0 の矩形は何も加算しない
/// let grid = imos_2d_half_open(2, 2, [((0, 0), (2, 0), 1)]);
/// assert_eq!(grid, vec![vec![0, 0], vec![0, 0]]);
/// ```
pub fn imos_2d_half_open(
    h: usize,
    w: usize,
    queries: impl IntoIterator<Item = ((usize, usize), (usize, usize), i64)>,
) -> Vec<Vec<i64>> {
    let mut imos = Imos2D::new(h, w);
    for (start, end, v) in queries {
        imos.add_rect(start, end, v);
    }
    imos.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    /// 矩形加算クエリ ((r1, c1), (r2, c2), v)。
    type Query = ((usize, usize), (usize, usize), i64);

    #[rstest]
    // 全体を覆う 1 クエリ（端ぴったり）
    #[case(2, 3, vec![((0, 0), (2, 3), 1)], vec![vec![1, 1, 1], vec![1, 1, 1]])]
    // 高さ 0 / 幅 0 の空矩形は何も足さない
    #[case(2, 2, vec![((1, 0), (1, 2), 5), ((0, 1), (2, 1), 5)], vec![vec![0, 0], vec![0, 0]])]
    // 行と列の取り違え検出（非正方・横帯と縦帯が 1 セルで交差）
    #[case(2, 3, vec![((0, 0), (1, 3), 1), ((0, 2), (2, 3), 1)],
           vec![vec![1, 1, 2], vec![0, 0, 1]])]
    // 負の加算で打ち消せる
    #[case(2, 2, vec![((0, 0), (2, 2), 3), ((0, 0), (1, 1), -3)],
           vec![vec![0, 3], vec![3, 3]])]
    fn imos_2d_half_open_cases(
        #[case] h: usize,
        #[case] w: usize,
        #[case] queries: Vec<Query>,
        #[case] expected: Vec<Vec<i64>>,
    ) {
        assert_eq!(imos_2d_half_open(h, w, queries), expected);
    }

    #[test]
    fn imos_2d_agrees_with_half_open() {
        // 閉矩形 [1, 2] x [0, 1] と半開矩形 [1, 3) x [0, 2) は同じ範囲
        let closed = imos_2d(4, 4, [((1, 0), (2, 1), 1)]);
        let half_open = imos_2d_half_open(4, 4, [((1, 0), (3, 2), 1)]);
        assert_eq!(closed, half_open);
    }

    #[test]
    fn struct_accumulates_across_calls() {
        let mut imos = Imos2D::new(2, 2);
        for r in 0..2 {
            imos.add_rect((r, 0), (r + 1, 2), r as i64 + 1);
        }
        assert_eq!(imos.build(), vec![vec![1, 1], vec![2, 2]]);
    }

    #[test]
    #[should_panic(expected = "Imos2D::add_rect")]
    fn add_rect_rejects_reversed_corners() {
        // 角を逆順に渡すと（debug ビルドでは）気づける
        Imos2D::new(3, 3).add_rect((2, 2), (1, 1), 1);
    }
}
