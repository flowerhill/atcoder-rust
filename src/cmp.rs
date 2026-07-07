//! 比較・更新ヘルパ（chmax / chmin）。

/// `*x < v` なら `*x` を `v` に更新して true を返す（いわゆる chmax）。
///
/// ```
/// use atcoder_rust::cmp::chmax;
///
/// let mut best = 3;
/// assert!(chmax(&mut best, 5)); // 更新された
/// assert_eq!(best, 5);
/// assert!(!chmax(&mut best, 4)); // 更新されない
/// assert_eq!(best, 5);
/// ```
pub fn chmax<T: PartialOrd>(x: &mut T, v: T) -> bool {
    if *x < v {
        *x = v;
        true
    } else {
        false
    }
}

/// `v < *x` なら `*x` を `v` に更新して true を返す（いわゆる chmin）。
///
/// ```
/// use atcoder_rust::cmp::chmin;
///
/// let mut dist = i64::MAX;
/// assert!(chmin(&mut dist, 10));
/// assert_eq!(dist, 10);
/// assert!(!chmin(&mut dist, 10)); // 同値は更新しない
/// ```
pub fn chmin<T: PartialOrd>(x: &mut T, v: T) -> bool {
    if v < *x {
        *x = v;
        true
    } else {
        false
    }
}
