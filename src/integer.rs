use std::ops::{
    Add, AddAssign, BitAnd, BitOr, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, Shr, Sub,
    SubAssign,
};

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
    fn as_usize(&self) -> usize;
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
}
