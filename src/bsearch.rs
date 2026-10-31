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
    /// 返る数は「`x` 未満」の要素の個数でもある(指しているのは要素ではなく境界)。
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
    /// 返る数は「`x` 以下」の要素の個数でもある(指しているのは要素ではなく境界)。
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

/// 昇順ソート済みスライスに対する「境界の要素」の取得。
///
/// `lower_bound` / `upper_bound` が返すのは「最初に条件を満たす位置 = 条件を満たす個数」で、
/// 「条件を満たす側の最後の要素」を指したいときは `- 1` が必要になる。その off-by-one を
/// 吸収し、`>= x` / `> x` / `< x` / `<= x` の 4 方向を同じ形で引けるようにする。
///
/// Haskell 版テンプレートの `lookupGEIdxA` / `lookupGTIdxA` / `lookupLTIdxA` / `lookupLEIdxA`
/// と同じ意味。Haskell の `boundGEA` 等は範囲外を `succ ub` / `pred lb` の番兵で返すが、
/// `usize` に `-1` が無いので `Option` で表す。
///
/// 実装は `lower_bound` / `upper_bound` の上に乗るので、両者を supertrait に置いて
/// 「`Bound` を実装する型は必ず両方を実装している」ことをコンパイラに検査させている。
///
/// ```
/// use atcoder_rust::bsearch::Bound;
///
/// let v = vec![1, 3, 5, 7, 9];
/// assert_eq!(v.bound_ge(&4), Some(2)); // 5 の位置
/// assert_eq!(v.bound_ge(&10), None);
/// assert_eq!(v.bound_gt(&5), Some(3)); // 7 の位置
/// assert_eq!(v.bound_gt(&9), None);
/// assert_eq!(v.bound_lt(&5), Some(1)); // 3 の位置
/// assert_eq!(v.bound_lt(&1), None);
/// assert_eq!(v.bound_le(&5), Some(2)); // 5 の位置
/// assert_eq!(v.bound_le(&0), None);
///
/// // 重複があるとき。ge/lt は重複の先頭側、le/gt は末尾側の外を指す
/// let d = vec![1, 2, 2, 2, 5];
/// assert_eq!(d.bound_lt(&2), Some(0)); // 重複の 1 つ手前の 1
/// assert_eq!(d.bound_ge(&2), Some(1)); // 重複する 2 の先頭
/// assert_eq!(d.bound_le(&2), Some(3)); // 重複する 2 の末尾
/// assert_eq!(d.bound_gt(&2), Some(4)); // 重複の 1 つ先の 5
///
/// // 存在しない値では lt/le と ge/gt が入れ替わる（区間に 1 つも入らない合図）
/// assert_eq!(d.bound_le(&3), Some(3)); // 3 以下の最後 = 2
/// assert_eq!(d.bound_ge(&3), Some(4)); // 3 以上の最初 = 5
///
/// // 値そのものが欲しいときは添字で引く
/// assert_eq!(v.bound_le(&6).map(|i| v[i]), Some(5));
///
/// // supertrait なので、`Bound` の境界だけで lower_bound / upper_bound も呼べる
/// fn count_between<T: Ord, S: Bound<T> + ?Sized>(s: &S, lo: &T, hi: &T) -> usize {
///     s.upper_bound(hi) - s.lower_bound(lo)
/// }
/// assert_eq!(count_between(&v[..], &3, &7), 3); // 3, 5, 7 の 3 個
/// ```
pub trait Bound<T>: LowerBound<T> + UpperBound<T> {
    /// 「`x` 以上」の値が現れる最小の添字（無ければ `None`）。
    /// 重複があれば `x` と等しい並びの先頭を指す。範囲内なら `lower_bound(x)` と同じ値。
    fn bound_ge(&self, x: &T) -> Option<usize>;
    /// 「`x` より大きい」値が現れる最小の添字（無ければ `None`）。
    /// `x` と等しい要素は飛ばすので、重複の 1 つ先を指す。範囲内なら `upper_bound(x)` と同じ値。
    fn bound_gt(&self, x: &T) -> Option<usize>;
    /// 「`x` より小さい」値が現れる最大の添字（無ければ `None`）。
    /// `x` と等しい要素は飛ばすので、重複の 1 つ手前を指す。`lower_bound(x) - 1` と同じ値。
    fn bound_lt(&self, x: &T) -> Option<usize>;
    /// 「`x` 以下」の値が現れる最大の添字（無ければ `None`）。
    /// 重複があれば `x` と等しい並びの末尾を指す。`upper_bound(x) - 1` と同じ値。
    fn bound_le(&self, x: &T) -> Option<usize>;
}

impl<T: Ord> Bound<T> for [T] {
    fn bound_ge(&self, x: &T) -> Option<usize> {
        let i = self.lower_bound(x);
        (i < self.len()).then_some(i)
    }

    fn bound_gt(&self, x: &T) -> Option<usize> {
        let i = self.upper_bound(x);
        (i < self.len()).then_some(i)
    }

    fn bound_lt(&self, x: &T) -> Option<usize> {
        // lower_bound = 「x 未満の個数」なので、その 1 つ手前が最後の x 未満
        self.lower_bound(x).checked_sub(1)
    }

    fn bound_le(&self, x: &T) -> Option<usize> {
        // upper_bound = 「x 以下の個数」なので、その 1 つ手前が最後の x 以下
        self.upper_bound(x).checked_sub(1)
    }
}

/// `neighbors` が返す片側の要素 `(添字, 値)`。端をはみ出す側は `None`。
pub type Neighbor<'a, T> = Option<(usize, &'a T)>;

/// 昇順ソート済みスライスから、`x` を挟む前後の要素を取り出す操作。
pub trait Neighbors<T>: Bound<T> {
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
        // 左は「x 未満で最後」、右は「x 以上で最初」= Bound の 2 方向そのもの
        (
            self.bound_lt(x).map(|i| (i, &self[i])),
            self.bound_ge(x).map(|i| (i, &self[i])),
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

    // 重複がある列。ge/lt は重複の先頭側、gt/le は末尾側を境界にする
    #[rstest]
    #[case(2, Some(1), Some(4), Some(0), Some(3))]
    #[case(1, Some(0), Some(1), None, Some(0))] // 先頭と一致 → lt は範囲外
    #[case(5, Some(5), None, Some(4), Some(5))] // 末尾と一致 → gt は範囲外
    #[case(4, Some(5), Some(5), Some(4), Some(4))] // 存在しない値は前後で挟む
    #[case(0, Some(0), Some(0), None, None)] // 全要素より小さい
    #[case(9, None, None, Some(5), Some(5))] // 全要素より大きい
    fn bound_cases(
        #[case] x: i32,
        #[case] ge: Option<usize>,
        #[case] gt: Option<usize>,
        #[case] lt: Option<usize>,
        #[case] le: Option<usize>,
    ) {
        let v = vec![1, 2, 2, 2, 3, 5];
        assert_eq!(v.bound_ge(&x), ge);
        assert_eq!(v.bound_gt(&x), gt);
        assert_eq!(v.bound_lt(&x), lt);
        assert_eq!(v.bound_le(&x), le);
    }

    // 全要素が同じ値 → ge/le は両端を指し、gt/lt は範囲外
    #[test]
    fn bound_on_all_equal() {
        let v = vec![7, 7, 7];
        assert_eq!(v.bound_ge(&7), Some(0));
        assert_eq!(v.bound_le(&7), Some(2));
        assert_eq!(v.bound_gt(&7), None);
        assert_eq!(v.bound_lt(&7), None);
    }

    // 空スライスでは 4 方向すべて None（checked_sub が 0 - 1 で落ちないこと）
    #[test]
    fn bound_on_empty() {
        let v: Vec<i32> = vec![];
        assert_eq!(v.bound_ge(&1), None);
        assert_eq!(v.bound_gt(&1), None);
        assert_eq!(v.bound_lt(&1), None);
        assert_eq!(v.bound_le(&1), None);
    }

    // 累積和に対する用途: 先頭の番兵 0 があるので bound_le は必ず Some を返す
    #[test]
    fn bound_le_on_cumsum() {
        let sa = vec![0u64, 22, 52]; // 安い順 0/1/2 個の合計
        assert_eq!(sa.bound_le(&110), Some(2));
        assert_eq!(sa.bound_le(&51), Some(1));
        assert_eq!(sa.bound_le(&0), Some(0)); // 予算 0 でも「0 個」は買える
    }

    // 既存 API との関係: lt/le は lower/upper の 1 つ手前、ge/gt は範囲内のときだけ一致
    #[rstest]
    #[case(0)]
    #[case(2)]
    #[case(4)]
    #[case(9)]
    fn bound_agrees_with_lower_upper(#[case] x: i32) {
        let v = vec![1, 2, 2, 2, 3, 5];
        assert_eq!(v.bound_lt(&x), v.lower_bound(&x).checked_sub(1));
        assert_eq!(v.bound_le(&x), v.upper_bound(&x).checked_sub(1));
        assert_eq!(
            v.bound_ge(&x),
            (v.lower_bound(&x) < v.len()).then(|| v.lower_bound(&x))
        );
        assert_eq!(
            v.bound_gt(&x),
            (v.upper_bound(&x) < v.len()).then(|| v.upper_bound(&x))
        );
    }

    // neighbors は (bound_lt, bound_ge) と同じ境界を指す
    #[rstest]
    #[case(0)]
    #[case(2)]
    #[case(4)]
    #[case(9)]
    fn bound_agrees_with_neighbors(#[case] x: i32) {
        let v = vec![1, 2, 2, 2, 3, 5];
        let (lower, upper) = v.neighbors(&x);
        assert_eq!(lower.map(|(i, _)| i), v.bound_lt(&x));
        assert_eq!(upper.map(|(i, _)| i), v.bound_ge(&x));
    }

    // 全探索との照合。`Bound` は lower_bound / upper_bound の上に実装されているので、
    // このテストが 6 関数すべての正しさをまとめて担保する。
    // 空列・1 要素・重複・全要素同値・負値・連番を、範囲外の x まで含めて総当たりする。
    #[rstest]
    #[case(vec![])]
    #[case(vec![0])]
    #[case(vec![1, 1])]
    #[case(vec![1, 2, 2, 2, 3, 5])]
    #[case(vec![-3, -1, 0, 0, 4])]
    #[case(vec![7, 7, 7, 7, 7])]
    #[case(vec![0, 1, 2, 3, 4, 5, 6, 7])]
    fn bounds_agree_with_linear_scan(#[case] v: Vec<i32>) {
        for x in -6..=10 {
            // lower_bound / upper_bound は「条件を満たす個数」
            assert_eq!(
                v.lower_bound(&x),
                v.iter().filter(|&&e| e < x).count(),
                "lower_bound: v={v:?}, x={x}"
            );
            assert_eq!(
                v.upper_bound(&x),
                v.iter().filter(|&&e| e <= x).count(),
                "upper_bound: v={v:?}, x={x}"
            );
            // bound_* は「条件を満たす側の端の位置」
            assert_eq!(
                v.bound_ge(&x),
                v.iter().position(|&e| e >= x),
                "bound_ge: v={v:?}, x={x}"
            );
            assert_eq!(
                v.bound_gt(&x),
                v.iter().position(|&e| e > x),
                "bound_gt: v={v:?}, x={x}"
            );
            assert_eq!(
                v.bound_lt(&x),
                v.iter().rposition(|&e| e < x),
                "bound_lt: v={v:?}, x={x}"
            );
            assert_eq!(
                v.bound_le(&x),
                v.iter().rposition(|&e| e <= x),
                "bound_le: v={v:?}, x={x}"
            );
        }
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
