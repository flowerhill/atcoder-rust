use std::ops::{Add, Mul, Sub};

/// 2 成分の座標・ベクトルを表し、成分ごとの加減算ができる型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pair<T>(pub T, pub T);

impl<T: Add<Output = T>> Add for Pair<T> {
    type Output = Pair<T>;

    /// 成分ごとに加算する。
    fn add(self, rhs: Self) -> Self::Output {
        Pair(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Sub<Output = T>> Sub for Pair<T> {
    type Output = Pair<T>;

    /// 成分ごとに減算する。
    fn sub(self, rhs: Self) -> Self::Output {
        Pair(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Mul<Output = T> + Sub<Output = T>> Pair<T> {
    /// 2 次元ベクトルの外積（z 成分）。0 なら 2 ベクトルは平行。
    ///
    /// ```
    /// use atcoder_rust::pair::Pair;
    ///
    /// assert_eq!(Pair(1, 0).cross(Pair(0, 1)), 1);
    /// assert_eq!(Pair(2, 4).cross(Pair(1, 2)), 0);
    /// ```
    pub fn cross(self, rhs: Self) -> T {
        self.0 * rhs.1 - self.1 * rhs.0
    }
}

impl<T: Mul<Output = T> + Add<Output = T>> Pair<T> {
    /// 2 次元ベクトルの内積（スカラ積）。0 なら 2 ベクトルは直交。
    ///
    /// ```
    /// use atcoder_rust::pair::Pair;
    ///
    /// assert_eq!(Pair(1, 0).dot(Pair(0, 1)), 0);
    /// assert_eq!(Pair(2, 3).dot(Pair(4, 5)), 23);
    /// ```
    pub fn dot(self, rhs: Self) -> T {
        self.0 * rhs.0 + self.1 * rhs.1
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

    #[rstest]
    #[case(Pair(1, 0), Pair(0, 1), 1)]
    #[case(Pair(2, 4), Pair(1, 2), 0)]
    #[case(Pair(-1, 1), Pair(2, 2), -4)]
    fn cross_z_component(#[case] a: Pair<i32>, #[case] b: Pair<i32>, #[case] expected: i32) {
        assert_eq!(a.cross(b), expected);
    }

    #[rstest]
    #[case(Pair(1, 0), Pair(0, 1), 0)]
    #[case(Pair(2, 3), Pair(4, 5), 23)]
    #[case(Pair(-1, 1), Pair(2, 2), 0)]
    fn dot_scalar_product(#[case] a: Pair<i32>, #[case] b: Pair<i32>, #[case] expected: i32) {
        assert_eq!(a.dot(b), expected);
    }

    #[test]
    fn add_then_sub_roundtrips() {
        let a = Pair(7, -3);
        let b = Pair(2, 5);
        assert_eq!((a + b) - b, a);
    }
}
