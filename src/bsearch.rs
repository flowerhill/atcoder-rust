use crate::math::Integer;

/// 二分探索。左を `false`、右を `true` として、条件を満たす最小の値を探す。
///
/// `f` は `false → true` に単調変化する述語であること。
/// 返り値 `(ng, ok)` = `(条件を満たさない最大値, 条件を満たす最小値)`。
pub fn bisect<T: Integer>(l: T, r: T, mut f: impl FnMut(&T) -> bool) -> (T, T) {
    let (mut ng, mut ok) = (l, r);
    while ok > ng + T::ONE {
        let mid = ng + (ok - ng) / T::TWO;
        *if f(&mid) { &mut ok } else { &mut ng } = mid;
    }
    (ng, ok)
}

/// 二分探索(左 true 版)。左を `true`、右を `false` として、条件を満たす最大の値を探す。
///
/// `f` は `true → false` に単調変化する述語であること。
/// 返り値 `(ok, ng)` = `(条件を満たす最大値, 条件を満たさない最小値)`。
pub fn bisect_rev<T: Integer>(l: T, r: T, mut f: impl FnMut(&T) -> bool) -> (T, T) {
    let (mut ok, mut ng) = (l, r);
    while ng > ok + T::ONE {
        let mid = ok + (ng - ok) / T::TWO;
        *if f(&mid) { &mut ok } else { &mut ng } = mid;
    }
    (ok, ng)
}

/// 昇順ソート済みスライスに対する lower bound 操作。
pub trait LowerBound<T> {
    type Item: Ord;
    /// `x` 以上の値が最初に現れる位置を返す(無ければ `len` = 挿入位置)。
    fn lower_bound(&self, x: &T) -> usize;
}

impl<T: Ord> LowerBound<T> for [T] {
    type Item = T;
    fn lower_bound(&self, x: &T) -> usize {
        // bisect は f(l)=false を前提とするので、空 or 先頭が既に x 以上なら別扱い
        if self.first().is_none_or(|h| h >= x) {
            return 0;
        }
        // f(i) = self[i] >= x（false→true の単調述語）の最初の true を返す
        bisect(0, self.len(), |&i| &self[i] >= x).1
    }
}

/// 昇順ソート済みスライスに対する upper bound 操作。
pub trait UpperBound<T> {
    type Item: Ord;
    /// `x` より大きい値が最初に現れる位置を返す(無ければ `len` = 挿入位置)。
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> UpperBound<T> for [T] {
    type Item = T;
    fn upper_bound(&self, x: &T) -> usize {
        // 空 or 先頭が既に x より大きいなら別扱い
        if self.first().is_none_or(|h| h > x) {
            return 0;
        }
        // f(i) = self[i] > x（false→true の単調述語）の最初の true を返す
        bisect(0, self.len(), |&i| &self[i] > x).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // f(x) = x >= threshold を満たす最小の値が境界 (ng, ok) になる
    #[rstest]
    #[case(5, 4, 5)] // 通常の境界
    #[case(0, 0, 1)] // 全域 true → 左端
    #[case(10, 9, 10)] // 全域 false → 右端
    fn bisect_finds_boundary(#[case] threshold: i64, #[case] ng: i64, #[case] ok: i64) {
        assert_eq!(bisect(0i64, 10, |&x| x >= threshold), (ng, ok));
    }

    // 左 true 版: f(x) = x <= threshold を満たす最大の値が境界 (ok, ng) になる
    #[rstest]
    #[case(5, 5, 6)] // 通常の境界
    #[case(10, 9, 10)] // 全域 true → 右端
    #[case(0, 0, 1)] // 0 のみ true → 左端
    fn bisect_rev_finds_boundary(#[case] threshold: i64, #[case] ok: i64, #[case] ng: i64) {
        assert_eq!(bisect_rev(0i64, 10, |&x| x <= threshold), (ok, ng));
    }

    // bisect と bisect_rev は同じ単調列に対し同じ境界を指す
    // f(x)=x>=5 の「満たす最小」と f(x)=x<5 の「満たす最大」は隣接する
    #[test]
    fn bisect_and_rev_agree_on_boundary() {
        let (_, ok) = bisect(0i64, 10, |&x| x >= 5); // ok = 5
        let (max_true, _) = bisect_rev(0i64, 10, |&x| x < 5); // max_true = 4
        assert_eq!(ok, max_true + 1);
    }

    #[rstest]
    #[case(2, 1)]
    #[case(3, 4)]
    #[case(0, 0)] // 全要素より小さい
    #[case(6, 6)] // 全要素より大きい
    #[case(4, 5)] // 存在しない値は挿入位置
    fn lower_bound_basic(#[case] x: i32, #[case] expected: usize) {
        let v = vec![1, 2, 2, 2, 3, 5];
        assert_eq!(v.lower_bound(&x), expected);
    }

    #[rstest]
    #[case(2, 4)]
    #[case(3, 5)]
    #[case(0, 0)]
    #[case(5, 6)]
    #[case(4, 5)]
    fn upper_bound_basic(#[case] x: i32, #[case] expected: usize) {
        let v = vec![1, 2, 2, 2, 3, 5];
        assert_eq!(v.upper_bound(&x), expected);
    }

    #[rstest]
    #[case(&1)]
    #[case(&-5)]
    fn bounds_on_empty(#[case] x: &i32) {
        let v: Vec<i32> = vec![];
        assert_eq!(v.lower_bound(x), 0);
        assert_eq!(v.upper_bound(x), 0);
    }

    // [lower, upper) の幅 = その値の個数
    #[rstest]
    #[case(2, 3)]
    #[case(1, 1)]
    #[case(4, 0)] // 存在しない値
    fn lower_upper_difference_is_count(#[case] x: i32, #[case] count: usize) {
        let v = vec![1, 2, 2, 2, 3, 5];
        assert_eq!(v.upper_bound(&x) - v.lower_bound(&x), count);
    }
}
