//! 累積和（静的な列に対する区間和クエリ）。

use std::ops::{Add, Sub};

/// 列 `xs` の累積和を返す。返り値 `s` は長さ `xs.len() + 1` で `s[i] = xs[..i]` の総和。
/// 先頭に 0 が入る形なので、半開区間 `[l, r)` の和は `s[r] - s[l]`（`range_sum`）で取れる。
/// 構築 `O(n)`、以降のクエリ 1 回 `O(1)`。
///
/// ```
/// use atcoder_rust::cumsum::cumsum;
///
/// assert_eq!(cumsum([1u64, 2, 3, 4]), vec![0, 1, 3, 6, 10]);
///
/// // 空列でも先頭の 0 だけは入る
/// assert_eq!(cumsum(Vec::<i64>::new()), vec![0]);
///
/// // 負の値でもよい
/// assert_eq!(cumsum([3i64, -5, 2]), vec![0, 3, -2, 0]);
/// ```
pub fn cumsum<T>(xs: impl IntoIterator<Item = T>) -> Vec<T>
where
    T: Copy + Default + Add<Output = T>,
{
    std::iter::once(T::default())
        .chain(xs.into_iter().scan(T::default(), |acc, x| {
            *acc = *acc + x;
            Some(*acc)
        }))
        .collect()
}

/// `cumsum` が返した累積和 `s` から、半開区間 `[l, r)` の和を取り出す。`O(1)`。
///
/// 閉区間 `[l, r]` が欲しいときは `range_sum(s, l, r + 1)` と呼ぶ。
///
/// ```
/// use atcoder_rust::cumsum::{cumsum, range_sum};
///
/// let s = cumsum([1u64, 2, 3, 4]);
/// assert_eq!(range_sum(&s, 1, 3), 5); // 2 + 3
/// assert_eq!(range_sum(&s, 0, 4), 10); // 全体
/// assert_eq!(range_sum(&s, 2, 2), 0); // 空区間
/// ```
pub fn range_sum<T>(s: &[T], l: usize, r: usize) -> T
where
    T: Copy + Sub<Output = T>,
{
    s[r] - s[l]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![], vec![0])] // 空列
    #[case(vec![7], vec![0, 7])] // 1 要素
    #[case(vec![0, 0, 0], vec![0, 0, 0, 0])] // 全部 0
    #[case(vec![1, 2, 3, 4], vec![0, 1, 3, 6, 10])] // 典型
    fn cumsum_cases(#[case] xs: Vec<u64>, #[case] expected: Vec<u64>) {
        assert_eq!(cumsum(xs), expected);
    }

    #[test]
    fn cumsum_does_not_overflow_u32_range() {
        // u64 なら 10^9 を 10^5 個足しても溢れない（u32 なら溢れる規模）
        let s = cumsum(vec![1_000_000_000u64; 100_000]);
        assert_eq!(s[100_000], 100_000_000_000_000);
    }

    #[test]
    fn range_sum_covers_every_subinterval() {
        let xs = vec![5i64, -3, 8, 0, 2];
        let s = cumsum(xs.clone());
        for l in 0..=xs.len() {
            for r in l..=xs.len() {
                assert_eq!(range_sum(&s, l, r), xs[l..r].iter().sum::<i64>());
            }
        }
    }
}
