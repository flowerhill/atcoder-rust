use crate::integer::Integer;

// 二分探索
// 左をfalse, 右をtrueとして、条件を満たす最小の値を探す
pub fn bisect<T: Integer>(l: T, r: T, mut f: impl FnMut(&T) -> bool) -> (T, T) {
    let (mut ng, mut ok) = (l, r);
    while ok > ng + T::ONE {
        let mid = ng + (ok - ng) / T::TWO;
        *if f(&mid) { &mut ok } else { &mut ng } = mid;
    }
    (ng, ok)
}

pub trait LowerBound<T> {
    type Item: Ord;
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

pub trait UpperBound<T> {
    type Item: Ord;
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
