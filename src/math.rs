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

/// 素数 mod（AtCoder 頻出の 10^9+7）。
pub const MOD: i64 = 1_000_000_007;

/// `base^exp mod m` を繰り返し二乗法（バイナリ法）で計算する。O(log exp)。
/// `m == 1` でも `1 % m == 0` となり破綻しない。`0^0 == 1` と定義する。
/// `base` が負でも `base %= m` 後に `+ m` で非負へ寄せて扱う。`exp >= 0` が前提。
///
/// ```
/// use atcoder_rust::math::{modpow, MOD};
/// assert_eq!(modpow(2, 10, MOD), 1024);
/// assert_eq!(modpow(3, 0, 7), 1);
/// assert_eq!(modpow(0, 0, 7), 1);
/// assert_eq!(modpow(123, 456, MOD), 565291922);
/// assert_eq!(modpow(-1, 3, 7), 6); // (-1)^3 = -1 ≡ 6 (mod 7)
/// ```
pub fn modpow(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1 % m;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

/// 素数 `m` を法とする `a` の逆元（フェルマーの小定理）。`m` は素数、`gcd(a, m) == 1` が前提。
/// `a` が負でも `modpow` 側で非負へ寄せて扱う。
///
/// ```
/// use atcoder_rust::math::{modinv, MOD};
/// assert_eq!(modinv(2, MOD) * 2 % MOD, 1);
/// assert_eq!(modinv(3, 7), 5); // 3*5 = 15 ≡ 1 (mod 7)
/// ```
pub fn modinv(a: i64, m: i64) -> i64 {
    modpow(a, m - 2, m)
}

/// 階乗・逆階乗を前計算し、mod 素数 `MOD` 上で二項係数 `nCr` / 順列 `nPr` を O(1) で返す。
/// `new(n_max)` で 0..=n_max のテーブルを作る（前計算 O(n_max)）。
///
/// ```
/// use atcoder_rust::math::Comb;
/// let c = Comb::new(1000);
/// assert_eq!(c.comb(5, 2), 10);
/// assert_eq!(c.perm(5, 2), 20);
/// assert_eq!(c.comb(5, 0), 1);
/// assert_eq!(c.comb(2, 5), 0); // r > n は 0
/// ```
pub struct Comb {
    fact: Vec<i64>,
    inv_fact: Vec<i64>,
}

impl Comb {
    /// 0..=n_max の階乗・逆階乗を前計算する。O(n_max)。
    pub fn new(n_max: usize) -> Self {
        let mut fact = vec![1i64; n_max + 1];
        for i in 1..=n_max {
            fact[i] = fact[i - 1] * i as i64 % MOD;
        }
        let mut inv_fact = vec![1i64; n_max + 1];
        inv_fact[n_max] = modinv(fact[n_max], MOD);
        for i in (1..=n_max).rev() {
            inv_fact[i - 1] = inv_fact[i] * i as i64 % MOD;
        }
        Self { fact, inv_fact }
    }

    /// 二項係数 nCr mod MOD。`r > n` なら 0。`n` は前計算範囲内であること。
    pub fn comb(&self, n: usize, r: usize) -> i64 {
        if r > n {
            return 0;
        }
        assert!(
            n < self.fact.len(),
            "Comb::comb: n={} が前計算範囲 {} を超えています",
            n,
            self.fact.len() - 1
        );
        self.fact[n] * self.inv_fact[r] % MOD * self.inv_fact[n - r] % MOD
    }

    /// 順列 nPr mod MOD。`r > n` なら 0。`n` は前計算範囲内であること。
    pub fn perm(&self, n: usize, r: usize) -> i64 {
        if r > n {
            return 0;
        }
        assert!(
            n < self.fact.len(),
            "Comb::perm: n={} が前計算範囲 {} を超えています",
            n,
            self.fact.len() - 1
        );
        self.fact[n] * self.inv_fact[n - r] % MOD
    }
}

/// 最大公約数（ユークリッドの互除法）。負値は絶対値で扱う。`gcd(0, 0) == 0`。
///
/// ```
/// use atcoder_rust::math::gcd;
/// assert_eq!(gcd(12, 18), 6);
/// assert_eq!(gcd(0, 5), 5);
/// assert_eq!(gcd(-12, 18), 6);
/// ```
pub fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

/// 最小公倍数。`a` か `b` が 0 なら 0。先に割ってから掛けてオーバーフローを抑える。
///
/// ```
/// use atcoder_rust::math::lcm;
/// assert_eq!(lcm(4, 6), 12);
/// assert_eq!(lcm(0, 5), 0);
/// ```
pub fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        a.abs() / gcd(a, b) * b.abs()
    }
}

/// エラトステネスの篩。長さ `n+1` の `Vec<bool>` を返し、`v[i]` は i が素数か（0,1 は false）。
///
/// ```
/// use atcoder_rust::math::sieve;
/// let p = sieve(10);
/// assert!(p[2] && p[3] && p[5] && p[7]);
/// assert!(!p[0] && !p[1] && !p[9]);
/// ```
pub fn sieve(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n >= 1 {
        is_prime[1] = false;
    }
    let mut i = 2;
    while i * i <= n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    is_prime
}

/// 試し割りによる素因数分解。`(素因数, 指数)` を昇順で返す。`n >= 1`。O(√n)。
///
/// ```
/// use atcoder_rust::math::factorize;
/// assert_eq!(factorize(12), vec![(2, 2), (3, 1)]);
/// assert_eq!(factorize(1), vec![]);
/// assert_eq!(factorize(97), vec![(97, 1)]);
/// ```
pub fn factorize(mut n: i64) -> Vec<(i64, u32)> {
    let mut factors = Vec::new();
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            let mut e = 0;
            while n % d == 0 {
                n /= d;
                e += 1;
            }
            factors.push((d, e));
        }
        d += 1;
    }
    if n > 1 {
        factors.push((n, 1));
    }
    factors
}

/// `n` の約数を昇順で全列挙する。`n >= 1`。O(√n)。
///
/// ```
/// use atcoder_rust::math::divisors;
/// assert_eq!(divisors(12), vec![1, 2, 3, 4, 6, 12]);
/// assert_eq!(divisors(1), vec![1]);
/// assert_eq!(divisors(7), vec![1, 7]);
/// ```
pub fn divisors(n: i64) -> Vec<i64> {
    let mut small = Vec::new();
    let mut large = Vec::new();
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            small.push(i);
            if i != n / i {
                large.push(n / i);
            }
        }
        i += 1;
    }
    large.reverse();
    small.extend(large);
    small
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
