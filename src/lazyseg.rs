//! 遅延セグメント木（ac-library-rs の `LazySegtree`）に載せる作用（`MapMonoid`）の定義集。
//!
//! ac-library-rs はモノイドと作用の実装を利用者側に任せているので、よく使う組み合わせをここに置く。

use std::marker::PhantomData;

use ac_library::{MapMonoid, Max, Monoid};

/// 区間 chmax（各要素を `max(x, f)` に更新）+ 区間 max 取得。
///
/// `f` が区間内のどの値よりも大きいと分かっている場合は「区間代入」と同じ意味になるので、
/// 「区間をいまの最大値より大きい値で塗りつぶす」用途にも使える。
///
/// ```
/// use ac_library::LazySegtree;
/// use atcoder_rust::lazyseg::ChmaxMax;
///
/// let mut seg = LazySegtree::<ChmaxMax<u64>>::from(vec![3, 1, 4, 1, 5]);
/// assert_eq!(seg.prod(1..4), 4);
///
/// seg.apply_range(0..3, 4); // [4, 4, 4, 1, 5]
/// assert_eq!(seg.prod(0..3), 4);
/// assert_eq!(seg.prod(3..5), 5);
/// ```
pub struct ChmaxMax<T>(PhantomData<fn() -> T>);

impl<T> MapMonoid for ChmaxMax<T>
where
    T: Copy + Ord,
    Max<T>: Monoid<S = T>,
{
    type M = Max<T>;
    type F = T;

    /// 単位作用は max の単位元（型の最小値）。chmax しても値が変わらない。
    fn identity_map() -> T {
        Max::<T>::identity()
    }

    fn mapping(f: &T, x: &T) -> T {
        (*f).max(*x)
    }

    fn composition(f: &T, g: &T) -> T {
        (*f).max(*g)
    }
}
