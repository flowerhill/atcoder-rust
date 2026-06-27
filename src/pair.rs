use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pair<T>(pub T, pub T);

impl<T: Add<Output = T>> Add for Pair<T> {
    type Output = Pair<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Pair(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Sub<Output = T>> Sub for Pair<T> {
    type Output = Pair<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Pair(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Pair(1, 2), Pair(3, 4), Pair(4, 6))]
    #[case(Pair(0, 0), Pair(5, -5), Pair(5, -5))]
    #[case(Pair(-1, -2), Pair(-3, -4), Pair(-4, -6))]
    fn add_componentwise(#[case] a: Pair<i32>, #[case] b: Pair<i32>, #[case] expected: Pair<i32>) {
        assert_eq!(a + b, expected);
    }

    #[rstest]
    #[case(Pair(3, 4), Pair(1, 2), Pair(2, 2))]
    #[case(Pair(0, 0), Pair(5, -5), Pair(-5, 5))]
    fn sub_componentwise(#[case] a: Pair<i32>, #[case] b: Pair<i32>, #[case] expected: Pair<i32>) {
        assert_eq!(a - b, expected);
    }

    #[test]
    fn add_then_sub_roundtrips() {
        let a = Pair(7, -3);
        let b = Pair(2, 5);
        assert_eq!((a + b) - b, a);
    }
}
