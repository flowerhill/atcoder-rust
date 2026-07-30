//! 値渡しの演算子 impl から、参照版（`&T op U` / `T op &U` / `&T op &U`）を生やすマクロ。
//!
//! `i64` などのプリミティブには std が同等のマクロ（core の `forward_ref_binop!` 等）で
//! 参照版を用意しているが、自作型には無い。そのため `Vec<Pair<i64>>` を `iter()` で
//! 舐めると `Item = &Pair<i64>` になり、`acc + p` が「`&Pair` を足せない」で落ちる。
//! `*p` と書けば済むものの、演算のたびにデシリアライズが要るのは見通しが悪いので、
//! 値渡しの impl を 1 つ書いたらこのマクロで参照版 3 種を生やしておく。
//!
//! 元の型が `Copy` であることが前提（参照から `*self` で値を取り出すため）。
//! ジェネリック型に使うときは境界を `where` 節で渡す。
//!
//! ```ignore
//! use crate::refops::forward_ref_binop;
//!
//! impl<T: Add<Output = T>> Add for Pair<T> { .. }
//! forward_ref_binop!(impl<T> Add, add for Pair<T>, Pair<T> where T: Copy + Add<Output = T>);
//!
//! // 非ジェネリックな型なら core と同じ呼び出し形
//! forward_ref_binop!(impl Add, add for Money, Money);
//! ```

/// 値渡しの二項演算子 `T op U` から `&T op U` / `T op &U` / `&T op &U` を生やす。
#[macro_export]
macro_rules! forward_ref_binop {
    (impl $(<$($g:ident),*>)? $imp:ident, $method:ident for $t:ty, $u:ty $(where $($w:tt)*)?) => {
        impl $(<$($g),*>)? $imp<$u> for &$t $(where $($w)*)? {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            #[track_caller]
            fn $method(self, rhs: $u) -> Self::Output {
                $imp::$method(*self, rhs)
            }
        }

        impl $(<$($g),*>)? $imp<&$u> for $t $(where $($w)*)? {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            #[track_caller]
            fn $method(self, rhs: &$u) -> Self::Output {
                $imp::$method(self, *rhs)
            }
        }

        impl $(<$($g),*>)? $imp<&$u> for &$t $(where $($w)*)? {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            #[track_caller]
            fn $method(self, rhs: &$u) -> Self::Output {
                $imp::$method(*self, *rhs)
            }
        }
    };
}

/// 値渡しの単項演算子 `op T` から `op &T` を生やす。
#[macro_export]
macro_rules! forward_ref_unop {
    (impl $(<$($g:ident),*>)? $imp:ident, $method:ident for $t:ty $(where $($w:tt)*)?) => {
        impl $(<$($g),*>)? $imp for &$t $(where $($w)*)? {
            type Output = <$t as $imp>::Output;

            #[inline]
            #[track_caller]
            fn $method(self) -> Self::Output {
                $imp::$method(*self)
            }
        }
    };
}

/// 値渡しの複合代入 `T op= U` から `T op= &U` を生やす。
#[macro_export]
macro_rules! forward_ref_op_assign {
    (impl $(<$($g:ident),*>)? $imp:ident, $method:ident for $t:ty, $u:ty $(where $($w:tt)*)?) => {
        impl $(<$($g),*>)? $imp<&$u> for $t $(where $($w)*)? {
            #[inline]
            #[track_caller]
            fn $method(&mut self, rhs: &$u) {
                $imp::$method(self, *rhs);
            }
        }
    };
}

// `#[macro_export]` はマクロをクレート直下に置くだけなので、モジュールパス
// (`crate::refops::forward_ref_binop`) でも呼べるよう re-export する。バンドラは
// `crate::<モジュール名>` を見て依存を辿るため、この形でないと refops が提出ファイルに
// 載らない。提出後は bin クレートになり、使わなかったマクロの re-export に
// unused_imports が付くので許可しておく。
#[allow(unused_imports)]
pub use {forward_ref_binop, forward_ref_op_assign, forward_ref_unop};

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::{Add, AddAssign, Neg};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Money(i64);

    impl Add for Money {
        type Output = Money;
        fn add(self, rhs: Self) -> Money {
            Money(self.0 + rhs.0)
        }
    }
    forward_ref_binop!(impl Add, add for Money, Money);

    impl Neg for Money {
        type Output = Money;
        fn neg(self) -> Money {
            Money(-self.0)
        }
    }
    forward_ref_unop!(impl Neg, neg for Money);

    impl AddAssign for Money {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0;
        }
    }
    forward_ref_op_assign!(impl AddAssign, add_assign for Money, Money);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Wrap<T>(T);

    impl<T: Add<Output = T>> Add for Wrap<T> {
        type Output = Wrap<T>;
        fn add(self, rhs: Self) -> Wrap<T> {
            Wrap(self.0 + rhs.0)
        }
    }
    forward_ref_binop!(impl<T> Add, add for Wrap<T>, Wrap<T> where T: Copy + Add<Output = T>);

    #[test]
    fn binop_accepts_every_reference_combination() {
        let (a, b) = (Money(1), Money(2));
        assert_eq!(a + b, Money(3));
        assert_eq!(&a + b, Money(3));
        assert_eq!(a + &b, Money(3));
        assert_eq!(&a + &b, Money(3));
    }

    #[test]
    fn unop_accepts_a_reference() {
        assert_eq!(-&Money(5), Money(-5));
    }

    #[test]
    fn op_assign_accepts_a_reference() {
        let mut a = Money(1);
        a += &Money(2);
        assert_eq!(a, Money(3));
    }

    /// ジェネリック型でも境界を where 節で渡せば同じように生える。
    #[test]
    fn generic_type_gets_reference_impls() {
        let (a, b) = (Wrap(1), Wrap(2));
        assert_eq!(&a + &b, Wrap(3));
    }

    /// 実際の動機: Item が参照になるイテレータ越しの畳み込み。
    #[test]
    fn folds_over_an_iterator_of_references() {
        let vs = vec![Money(1), Money(2), Money(3)];
        assert_eq!(vs.iter().fold(Money(0), |acc, m| acc + m), Money(6));
    }
}
