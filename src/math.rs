use std::ops::{
    Add, AddAssign, BitAnd, BitOr, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, Shr, Sub,
    SubAssign,
};

/// 各種整数型(`i64` / `usize` など)を共通に扱うためのトレイト。
/// 四則演算・ビット演算・定数・`usize` 変換をまとめて要求する。
pub trait Integer:
    Sized
    + Copy
    + Ord
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + RemAssign
    + Shr<usize, Output = Self>
    + Shl<usize, Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const MAX: Self;
    const MIN: Self;
    /// `self` を `usize` にキャストして返す。
    fn as_usize(&self) -> usize;
    /// `usize` 値を `Self` 型にキャストして生成する。
    fn from_usize(n: usize) -> Self;
}
macro_rules! impl_integer {
    ($($ty:ident),*) => {
        $(
            impl Integer for $ty {
                const ZERO: Self = 0;
                const ONE: Self = 1;
                const TWO: Self = 2;
                const MAX: Self = Self::MAX;
                const MIN: Self = Self::MIN;
                fn as_usize(&self) -> usize {
                    *self as usize
                }
                fn from_usize(n: usize) -> Self {
                    n as $ty
                }
            }
        )*
    };
}

impl_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

/// 整数区間 [lo, hi] の要素数。空区間 (lo > hi) なら 0 を返す。
/// `Integer` を実装する任意の整数型で使える（`usize` など）。
/// `lo > hi` を先に弾くので符号なし型でもアンダーフローしない。
pub fn range_size<T: Integer>(lo: T, hi: T) -> T {
    if lo > hi {
        T::ZERO
    } else {
        hi - lo + T::ONE
    }
}

/// 非負整数 `n` を `base` 進法で桁分解する（最上位桁が先頭）。
/// `n == 0` のときは `[0]` を返す。
///
/// ```
/// use atcoder_rust::math::to_digits;
/// assert_eq!(to_digits(21u64, 10), vec![2u64, 1]);
/// assert_eq!(to_digits(17u64, 9), vec![1, 8]);
/// assert_eq!(to_digits(0u64, 9), vec![0]);
/// ```
pub fn to_digits<T>(mut n: T, base: T) -> Vec<T>
where
    T: Copy + PartialEq + Default + std::ops::Rem<Output = T> + std::ops::Div<Output = T>,
{
    let zero = T::default();
    let mut ds = Vec::new();
    loop {
        ds.push(n % base);
        n = n / base;
        if n == zero {
            break;
        }
    }
    ds.reverse();
    ds
}

/// `base` 進法の桁列（最上位桁が先頭）を整数に戻す。
///
/// ```
/// use atcoder_rust::math::from_digits;
/// assert_eq!(from_digits(&[2u64, 1], 8), 17);
/// assert_eq!(from_digits(&[1u64, 5], 8), 13);
/// assert_eq!(from_digits::<u64>(&[], 8), 0);
/// ```
pub fn from_digits<T>(digits: &[T], base: T) -> T
where
    T: Copy + Default + std::ops::Mul<Output = T> + std::ops::Add<Output = T>,
{
    digits.iter().fold(T::default(), |acc, &d| acc * base + d)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn check_consts<T: Integer>() {
        assert!(T::ZERO + T::ONE == T::ONE);
        assert!(T::ONE + T::ONE == T::TWO);
        assert!(T::MAX >= T::MIN);
    }

    #[test]
    fn constants_consistent() {
        check_consts::<i64>();
        check_consts::<u32>();
        check_consts::<usize>();
        check_consts::<i8>();
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(42)]
    #[case(1000)]
    fn usize_roundtrip(#[case] n: usize) {
        assert_eq!(<i64 as Integer>::from_usize(n).as_usize(), n);
        assert_eq!(<u32 as Integer>::from_usize(n).as_usize(), n);
    }

    #[rstest]
    #[case(21i64, 42)]
    #[case(0i64, 0)]
    #[case(-3i64, -6)]
    fn generic_double(#[case] x: i64, #[case] expected: i64) {
        fn double<T: Integer>(x: T) -> T {
            x * T::TWO
        }
        assert_eq!(double(x), expected);
    }

    #[test]
    fn signed_min_max_match_inherent() {
        assert_eq!(<i32 as Integer>::MAX, i32::MAX);
        assert_eq!(<i32 as Integer>::MIN, i32::MIN);
        assert_eq!(<u8 as Integer>::MAX, u8::MAX);
        assert_eq!(<u8 as Integer>::MIN, u8::MIN);
    }

    // 区間 [lo, hi] の要素数。空区間は 0
    #[rstest]
    #[case(1, 5, 5)]
    #[case(3, 3, 1)] // 1 点
    #[case(5, 1, 0)] // 空区間
    #[case(0, -1, 0)] // 空区間
    fn range_size_signed(#[case] lo: i64, #[case] hi: i64, #[case] expected: i64) {
        assert_eq!(range_size(lo, hi), expected);
    }

    // 符号なし型でも lo > hi でアンダーフローしない
    #[rstest]
    #[case(2, 5, 4)]
    #[case(5, 2, 0)]
    fn range_size_unsigned(#[case] lo: usize, #[case] hi: usize, #[case] expected: usize) {
        assert_eq!(range_size(lo, hi), expected);
    }

    // 桁分解（最上位桁が先頭）。n == 0 のときは [0]
    #[rstest]
    #[case(21, 10, vec![2, 1])]
    #[case(17, 9, vec![1, 8])]
    #[case(0, 9, vec![0])]
    #[case(255, 16, vec![15, 15])]
    #[case(5, 2, vec![1, 0, 1])]
    #[case(7, 10, vec![7])]
    fn to_digits_works(#[case] n: u64, #[case] base: u64, #[case] expected: Vec<u64>) {
        assert_eq!(to_digits(n, base), expected);
    }

    // 桁列（最上位桁が先頭）を整数に戻す。空列は 0
    #[rstest]
    #[case(vec![2, 1], 8, 17)]
    #[case(vec![1, 5], 8, 13)]
    #[case(vec![], 8, 0)]
    #[case(vec![15, 15], 16, 255)]
    #[case(vec![0], 9, 0)]
    fn from_digits_works(#[case] digits: Vec<u64>, #[case] base: u64, #[case] expected: u64) {
        assert_eq!(from_digits(&digits, base), expected);
    }

    // to_digits と from_digits は互いに逆変換
    #[rstest]
    #[case(0, 10)]
    #[case(1, 10)]
    #[case(21, 10)]
    #[case(255, 16)]
    #[case(1_000_000, 7)]
    #[case(123_456_789, 2)]
    fn digits_roundtrip(#[case] n: u64, #[case] base: u64) {
        assert_eq!(from_digits(&to_digits(n, base), base), n);
    }
}
