use num::traits::{Signed, ToPrimitive};
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

impl<T: Signed> Pair<T> {
    /// マンハッタン距離 |x1 - x2| + |y1 - y2|。
    ///
    /// ```
    /// use atcoder_rust::pair::Pair;
    ///
    /// assert_eq!(Pair(0, 0).manhattan_dist(Pair(3, 4)), 7);
    /// assert_eq!(Pair(-1, 2).manhattan_dist(Pair(2, -2)), 7);
    /// ```
    pub fn manhattan_dist(self, rhs: Self) -> T {
        (self.0 - rhs.0).abs() + (self.1 - rhs.1).abs()
    }
}

impl<T: Copy + Sub<Output = T> + Mul<Output = T> + Add<Output = T>> Pair<T> {
    /// ユークリッド距離の 2 乗（sqrt なし）。距離の等値比較にはこちらを使う。
    /// i64 でも座標が 2×10^9 を超えると 2 乗が溢れるので、その場合は i128 で使う。
    ///
    /// ```
    /// use atcoder_rust::pair::Pair;
    ///
    /// assert_eq!(Pair(0, 0).euclid_dist2(Pair(3, 4)), 25);
    /// ```
    pub fn euclid_dist2(self, rhs: Self) -> T {
        let d = self - rhs;
        d.dot(d)
    }
}

impl<T: ToPrimitive> Pair<T> {
    /// ユークリッド距離を f64 で返す。丸め誤差が出るので等値比較には euclid_dist2 を使うこと。
    ///
    /// ```
    /// use atcoder_rust::pair::Pair;
    ///
    /// assert_eq!(Pair(0, 0).euclid_dist(Pair(3, 4)), 5.0);
    /// assert_eq!(Pair(0.5, 0.0).euclid_dist(Pair(2.0, 2.0)), 2.5);
    /// ```
    pub fn euclid_dist(self, rhs: Self) -> f64 {
        let to = |v: T| v.to_f64().expect("Pair::euclid_dist: to_f64 failed");
        let dx = to(self.0) - to(rhs.0);
        let dy = to(self.1) - to(rhs.1);
        dx.hypot(dy)
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

    #[rstest]
    #[case(Pair(0, 0), Pair(3, 4), 7)]
    #[case(Pair(-1, 2), Pair(2, -2), 7)]
    #[case(Pair(5, 5), Pair(5, 5), 0)]
    fn manhattan_dist_abs_sum(#[case] a: Pair<i32>, #[case] b: Pair<i32>, #[case] expected: i32) {
        assert_eq!(a.manhattan_dist(b), expected);
    }

    #[rstest]
    #[case(Pair(0, 0), Pair(3, 4), 25)]
    #[case(Pair(-1, -1), Pair(2, 3), 25)]
    #[case(Pair(5, 5), Pair(5, 5), 0)]
    fn euclid_dist2_squared(#[case] a: Pair<i64>, #[case] b: Pair<i64>, #[case] expected: i64) {
        assert_eq!(a.euclid_dist2(b), expected);
    }

    #[test]
    fn euclid_dist_matches_sqrt_of_dist2() {
        let (a, b) = (Pair(0, 0), Pair(3, 4));
        assert_eq!(a.euclid_dist(b), (a.euclid_dist2(b) as f64).sqrt());
    }

    #[test]
    fn add_then_sub_roundtrips() {
        let a = Pair(7, -3);
        let b = Pair(2, 5);
        assert_eq!((a + b) - b, a);
    }
}
