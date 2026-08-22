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
    /// `x` 以上の値が最初に現れる位置を返す(無ければ `len` = 挿入位置)。
    fn lower_bound(&self, x: &T) -> usize;
}

impl<T: Ord> LowerBound<T> for [T] {
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
    /// `x` より大きい値が最初に現れる位置を返す(無ければ `len` = 挿入位置)。
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> UpperBound<T> for [T] {
    fn upper_bound(&self, x: &T) -> usize {
        // 空 or 先頭が既に x より大きいなら別扱い
        if self.first().is_none_or(|h| h > x) {
            return 0;
        }
        // f(i) = self[i] > x（false→true の単調述語）の最初の true を返す
        bisect(0, self.len(), |&i| &self[i] > x).1
    }
}

/// `neighbors` が返す片側の要素 `(添字, 値)`。端をはみ出す側は `None`。
pub type Neighbor<'a, T> = Option<(usize, &'a T)>;

/// 昇順ソート済みスライスから、`x` を挟む前後の要素を取り出す操作。
pub trait Neighbors<T> {
    /// `(x 未満で最大の要素, x 以上で最小の要素)` を `(添字, 値)` の組で返す。
    /// 端をはみ出す側は `None`（`x` が全要素より小さい / 大きい場合）。
    ///
    /// `x` と等しい要素があるときは、右側にその最初の 1 つが入る。
    ///
    /// ```
    /// use atcoder_rust::bsearch::Neighbors;
    ///
    /// let v = vec![10, 20, 20, 30];
    /// assert_eq!(v.neighbors(&25), (Some((2, &20)), Some((3, &30))));
    /// assert_eq!(v.neighbors(&20), (Some((0, &10)), Some((1, &20)))); // 一致は右側
    /// assert_eq!(v.neighbors(&5), (None, Some((0, &10)))); // 左端の外
    /// assert_eq!(v.neighbors(&99), (Some((3, &30)), None)); // 右端の外
    /// ```
    fn neighbors(&self, x: &T) -> (Neighbor<'_, T>, Neighbor<'_, T>);
}

impl<T: Ord> Neighbors<T> for [T] {
    fn neighbors(&self, x: &T) -> (Neighbor<'_, T>, Neighbor<'_, T>) {
        let i = self.lower_bound(x);
        (
            i.checked_sub(1).map(|j| (j, &self[j])),
            self.get(i).map(|v| (i, v)),
        )
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

    #[rstest]
    #[case(0, None, Some((0, 1)))] // 全要素より小さい → 左は無し
    #[case(6, Some((5, 5)), None)] // 全要素より大きい → 右は無し
    #[case(4, Some((4, 3)), Some((5, 5)))] // 存在しない値は前後で挟む
    #[case(2, Some((0, 1)), Some((1, 2)))] // 一致する値は右側（重複の先頭）
    #[case(1, None, Some((0, 1)))] // 先頭と一致 → 左は無し
    fn neighbors_cases(
        #[case] x: i32,
        #[case] lower: Option<(usize, i32)>,
        #[case] upper: Option<(usize, i32)>,
    ) {
        let v = vec![1, 2, 2, 2, 3, 5];
        let to_ref = |e: Option<(usize, i32)>| e.map(|(i, _)| (i, &v[i]));
        assert_eq!(v.neighbors(&x), (to_ref(lower), to_ref(upper)));
        // 値そのものも期待通りか（添字だけ合っていて値が違う取り違えを弾く）
        assert_eq!(v.neighbors(&x).0.map(|(_, &e)| e), lower.map(|(_, e)| e));
        assert_eq!(v.neighbors(&x).1.map(|(_, &e)| e), upper.map(|(_, e)| e));
    }

    // 空スライスでは前後どちらも存在しない
    #[test]
    fn neighbors_on_empty() {
        let v: Vec<i32> = vec![];
        assert_eq!(v.neighbors(&1), (None, None));
    }

    // 右側は lower_bound、左側はその 1 つ手前という関係が常に成り立つ
    #[rstest]
    #[case(0)]
    #[case(2)]
    #[case(4)]
    #[case(9)]
    fn neighbors_agree_with_lower_bound(#[case] x: i32) {
        let v = vec![1, 2, 2, 2, 3, 5];
        let i = v.lower_bound(&x);
        let (lower, upper) = v.neighbors(&x);
        assert_eq!(upper.map(|(j, _)| j), (i < v.len()).then_some(i));
        assert_eq!(lower.map(|(j, _)| j), i.checked_sub(1));
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
