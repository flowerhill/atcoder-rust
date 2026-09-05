use ac_library::ModInt1000000007;
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
    /// 絶対値。符号なし型では恒等写像（`gcd` などを符号の有無に依らず書くため）。
    fn abs(self) -> Self;
}
macro_rules! impl_integer {
    // 符号なし型: abs は恒等写像
    (unsigned: $($ty:ident),*) => {
        $( impl_integer!(@impl $ty, |x: $ty| x); )*
    };
    // 符号付き型: abs は標準の i*::abs
    (signed: $($ty:ident),*) => {
        $( impl_integer!(@impl $ty, |x: $ty| x.abs()); )*
    };
    (@impl $ty:ident, $abs:expr) => {
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
            fn abs(self) -> Self {
                ($abs)(self)
            }
        }
    };
}

impl_integer!(unsigned: u8, u16, u32, u64, u128, usize);
impl_integer!(signed: i8, i16, i32, i64, i128, isize);

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

/// 初項 `a`、公差 `d`、項数 `n` の等差数列 a, a+d, .., a+(n-1)d の総和 `n*a + d*n(n-1)/2`。
/// `n(n-1)` を作る前に偶数側を 2 で割るので、答えが `T` に収まる限り途中で溢れない
/// （符号なし型でもアンダーフローしない）。区間 [lo, hi] の総和は `sum_of_range`、
/// mod を取りたいときは `sum_of_arith_mod`（区間 [lo, hi] 版）を使う。
///
/// ```
/// use atcoder_rust::math::sum_of_arith;
/// assert_eq!(sum_of_arith(1u64, 1, 10), 55); // 1+2+..+10
/// assert_eq!(sum_of_arith(2u64, 3, 4), 26); // 2+5+8+11
/// assert_eq!(sum_of_arith(3i64, -2, 4), 0); // 3+1-1-3
/// assert_eq!(sum_of_arith(5u64, 0, 3), 15); // 公差 0
/// assert_eq!(sum_of_arith(7u64, 2, 0), 0); // 項数 0
/// // n*(n-1) は u64 で溢れるが、先に 2 で約分するので答えは正しい
/// assert_eq!(sum_of_arith(0u64, 1, 6_000_000_000), 17_999_999_997_000_000_000);
/// ```
pub fn sum_of_arith<T: Integer>(a: T, d: T, n: usize) -> T {
    if n == 0 {
        return T::ZERO;
    }
    let n = T::from_usize(n);
    // n(n-1)/2 = 0+1+..+(n-1)。n と n-1 の偶数側を先に 2 で割る。
    let steps = if n % T::TWO == T::ZERO {
        (n / T::TWO) * (n - T::ONE)
    } else {
        n * ((n - T::ONE) / T::TWO)
    };
    a * n + d * steps
}

/// 等差数列（公差 1）の和: 整数区間 [lo, hi] の総和。空区間 (lo > hi) なら 0。
/// `1+2+..+n` は `sum_of_range(1, n)`。mod を取るなら `sum_of_arith_mod`。
///
/// ```
/// use atcoder_rust::math::sum_of_range;
/// assert_eq!(sum_of_range(1u64, 10), 55); // 1+2+..+10
/// assert_eq!(sum_of_range(3u64, 5), 12); // 3+4+5
/// assert_eq!(sum_of_range(7u64, 7), 7); // 単点
/// assert_eq!(sum_of_range(5u64, 3), 0); // 空区間（符号なしでも安全）
/// assert_eq!(sum_of_range(-3i64, 3), 0); // 負を含む区間
/// ```
pub fn sum_of_range<T: Integer>(lo: T, hi: T) -> T {
    sum_of_arith(lo, T::ONE, range_size(lo, hi).as_usize())
}

/// 初項 `a`、公比 `r`、項数 `n` の等比数列 a, ar, .., ar^(n-1) の総和。O(n)。
/// `a(r^n - 1)/(r - 1)` の公式は `r == 1` で 0 除算になり、`r^n` も溢れやすいので、
/// ホーナー法 `s ← s*r + a` で 1 項ずつ畳み込む（途中の値は常に答え以下）。
/// mod を取るなら O(log n) の `sum_of_geom_mod` を使う。
///
/// ```
/// use atcoder_rust::math::sum_of_geom;
/// assert_eq!(sum_of_geom(1u64, 2, 10), 1023); // 2^10 - 1
/// assert_eq!(sum_of_geom(2u64, 3, 4), 80); // 2+6+18+54
/// assert_eq!(sum_of_geom(3i64, 1, 5), 15); // 公比 1 でも破綻しない
/// assert_eq!(sum_of_geom(1i64, -2, 4), -5); // 1-2+4-8
/// assert_eq!(sum_of_geom(7u64, 2, 0), 0); // 項数 0
/// ```
pub fn sum_of_geom<T: Integer>(a: T, r: T, n: usize) -> T {
    (0..n).fold(T::ZERO, |sum, _| sum * r + a)
}

/// 素数 mod 10^9+7 上の値。`+ - * /` がそのまま使えるので、mod を取る処理を手で書かない
/// （ac-library-rs の `StaticModInt`。判定環境にあるので提出ファイルへは展開されない）。
///
/// - 生成: `Mint::new(x)`（`x` は負でも 10^9+7 以上でもよい。自動で `[0, MOD)` に寄る）
/// - 取り出し: `m.val()` → `u32`
/// - 累乗 / 逆元: `m.pow(n)` / `m.inv()`
/// - `Display` があるので `println!("{}", m)` でそのまま出力できる
/// - 同じ `Vec` の 2 要素を足すときは `dp[i] += dp[j]` が借用検査に落ちるので
///   `dp[i] = dp[i] + dp[j]` と書く
///
/// 法が 10^9+7 以外なら `ac_library::{ModInt998244353, DynamicModInt}`、
/// 単発の累乗・逆元だけなら `ac_library::{pow_mod, inv_mod}`（`inv_mod` は合成数の法でも可）を使う。
///
/// ```
/// use atcoder_rust::math::Mint;
/// assert_eq!((Mint::new(3) + Mint::new(4)).val(), 7);
/// assert_eq!((Mint::new(0) - Mint::new(1)).val(), 1_000_000_006); // 負も自動で回る
/// assert_eq!((Mint::new(-1) * Mint::new(-1)).val(), 1);
/// assert_eq!((Mint::new(1) / Mint::new(2) * Mint::new(2)).val(), 1); // 除算は逆元倍
/// assert_eq!(Mint::new(2).pow(10).val(), 1024);
/// assert_eq!(Mint::new(1_000_000_008).val(), 1); // MOD 以上でもよい
/// ```
pub type Mint = ModInt1000000007;

/// 整数区間 [lo, hi] の総和 (lo+hi)(hi-lo+1)/2 を mod 10^9+7 で返す（`lo > hi` なら 0）。
/// 積を `i128` で厳密に計算してから `Mint` に落とすので、`lo`, `hi` が 10^18 規模でも
/// オーバーフローしない（積は最大 ~2*10^36 で i128 に収まる）。逆元は不要。
///
/// ```
/// use atcoder_rust::math::sum_of_arith_mod;
/// assert_eq!(sum_of_arith_mod(1, 10).val(), 55);
/// assert_eq!(sum_of_arith_mod(3, 5).val(), 12);
/// assert_eq!(sum_of_arith_mod(7, 7).val(), 7); // 単点
/// assert_eq!(sum_of_arith_mod(5, 3).val(), 0); // 空区間
/// assert_eq!(sum_of_arith_mod(1, 1_000_000_000_000_000_000).val(), 1225); // 10^18 でも溢れない
/// ```
pub fn sum_of_arith_mod(lo: i128, hi: i128) -> Mint {
    if lo > hi {
        Mint::new(0)
    } else {
        Mint::new((lo + hi) * (hi - lo + 1) / 2)
    }
}

/// 初項 `a`、公比 `r`、項数 `n` の等比数列の総和を mod 10^9+7 で返す。O(log n)。
/// 公式 `a(r^n - 1)/(r - 1)` を使うが、分母が 0 になる `r ≡ 1` は総和が `a * n` に
/// なるので先に分岐する（法が素数なのでそれ以外は逆元が存在する）。
/// `a`, `r` は負や 10^9+7 以上でもよい（`Mint::new` が `[0, MOD)` に寄せるので、
/// `r == Mint::new(1)` の判定がそのまま `r ≡ 1 (mod MOD)` の判定になる）。
///
/// ```
/// use atcoder_rust::math::sum_of_geom_mod;
/// assert_eq!(sum_of_geom_mod(1, 2, 10).val(), 1023); // 2^10 - 1
/// assert_eq!(sum_of_geom_mod(2, 3, 4).val(), 80); // 2+6+18+54
/// assert_eq!(sum_of_geom_mod(3, 1, 5).val(), 15); // 公比 1（分母が 0 になるので分岐で回避）
/// assert_eq!(sum_of_geom_mod(3, 1_000_000_008, 5).val(), 15); // r ≡ 1 (mod MOD) でも同じ
/// assert_eq!(sum_of_geom_mod(1, 2, 0).val(), 0); // 項数 0
/// assert_eq!(sum_of_geom_mod(1, 2, 100).val(), 976_371_284); // (2^100 - 1) mod MOD
/// ```
pub fn sum_of_geom_mod(a: i64, r: i64, n: usize) -> Mint {
    let (a, r) = (Mint::new(a), Mint::new(r));
    let one = Mint::new(1);
    if r == one {
        a * Mint::new(n)
    } else {
        a * (r.pow(n as u64) - one) / (r - one)
    }
}

/// 階乗・逆階乗を前計算し、mod 10^9+7 上で二項係数 `nCr` / 順列 `nPr` を O(1) で返す。
/// `new(n_max)` で 0..=n_max のテーブルを作る（前計算 O(n_max)）。
///
/// ```
/// use atcoder_rust::math::Comb;
/// let c = Comb::new(1000);
/// assert_eq!(c.comb(5, 2).val(), 10);
/// assert_eq!(c.perm(5, 2).val(), 20);
/// assert_eq!(c.comb(5, 0).val(), 1);
/// assert_eq!(c.comb(2, 5).val(), 0); // r > n は 0
/// assert_eq!(c.comb(1000, 500), c.comb(1000, 500)); // 大きい n でも O(1)
/// ```
pub struct Comb {
    fact: Vec<Mint>,
    inv_fact: Vec<Mint>,
}

impl Comb {
    /// 0..=n_max の階乗・逆階乗を前計算する。O(n_max)。
    /// 逆元は n_max のぶんだけ 1 回求め、あとは i を掛けて降順に伝播させる。
    pub fn new(n_max: usize) -> Self {
        let mut fact = vec![Mint::new(1); n_max + 1];
        for i in 1..=n_max {
            fact[i] = fact[i - 1] * Mint::new(i);
        }
        let mut inv_fact = vec![Mint::new(1); n_max + 1];
        inv_fact[n_max] = fact[n_max].inv();
        for i in (1..=n_max).rev() {
            inv_fact[i - 1] = inv_fact[i] * Mint::new(i);
        }
        Self { fact, inv_fact }
    }

    /// 二項係数 nCr（mod 10^9+7）。`r > n` なら 0。`n` は前計算範囲内であること。
    pub fn comb(&self, n: usize, r: usize) -> Mint {
        if r > n {
            return Mint::new(0);
        }
        assert!(
            n < self.fact.len(),
            "Comb::comb: n={} が前計算範囲 {} を超えています",
            n,
            self.fact.len() - 1
        );
        self.fact[n] * self.inv_fact[r] * self.inv_fact[n - r]
    }

    /// 順列 nPr（mod 10^9+7）。`r > n` なら 0。`n` は前計算範囲内であること。
    pub fn perm(&self, n: usize, r: usize) -> Mint {
        if r > n {
            return Mint::new(0);
        }
        assert!(
            n < self.fact.len(),
            "Comb::perm: n={} が前計算範囲 {} を超えています",
            n,
            self.fact.len() - 1
        );
        self.fact[n] * self.inv_fact[n - r]
    }
}

/// 最大公約数（ユークリッドの互除法）。負値は絶対値で扱う。`gcd(0, 0) == 0`。
/// `Integer` を実装する任意の整数型で使える（`usize` など）。
///
/// ```
/// use atcoder_rust::math::gcd;
/// assert_eq!(gcd(12, 18), 6);
/// assert_eq!(gcd(0, 5), 5);
/// assert_eq!(gcd(-12, 18), 6);
/// assert_eq!(gcd(12usize, 18), 6); // 符号なしでもそのまま呼べる
/// ```
pub fn gcd<T: Integer>(a: T, b: T) -> T {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != T::ZERO {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

/// 最小公倍数。`a` か `b` が 0 なら 0。先に割ってから掛けてオーバーフローを抑える。
/// `Integer` を実装する任意の整数型で使える。
///
/// ```
/// use atcoder_rust::math::lcm;
/// assert_eq!(lcm(4, 6), 12);
/// assert_eq!(lcm(0, 5), 0);
/// assert_eq!(lcm(4usize, 6), 12);
/// ```
pub fn lcm<T: Integer>(a: T, b: T) -> T {
    if a == T::ZERO || b == T::ZERO {
        T::ZERO
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

/// 篩で 0..=n の各数の「異なる素因数の個数」（ω 関数）をまとめて求める。
/// `v[i]` は i の素因数の種類数で、`v[0] = v[1] = 0`。O(n log log n)。
/// 素数 p を見つけるたび p の倍数すべてに 1 を足す（`counts[p] == 0` なら
/// より小さい素数で割り切れない = p は素数）。種類数は n <= 10^18 でも 15 以下に
/// 収まるので `u8` で持ち、n = 10^7 でも 10 MB に抑える。
///
/// ```
/// use atcoder_rust::math::distinct_prime_factor_counts;
/// let w = distinct_prime_factor_counts(12);
/// assert_eq!(w[1], 0); // 1 は素因数なし
/// assert_eq!(w[7], 1); // 素数
/// assert_eq!(w[8], 1); // 2^3 は種類としては 1
/// assert_eq!(w[12], 2); // 2^2 * 3
/// assert_eq!(distinct_prime_factor_counts(0), vec![0]);
/// ```
pub fn distinct_prime_factor_counts(n: usize) -> Vec<u8> {
    let mut counts = vec![0u8; n + 1];
    for p in 2..=n {
        if counts[p] == 0 {
            for m in (p..=n).step_by(p) {
                counts[m] += 1;
            }
        }
    }
    counts
}

/// 試し割りによる素因数分解。`(素因数, 指数)` を昇順で返す。`n >= 1`。O(√n)。
/// `Integer` を実装する任意の整数型で使える。
///
/// ```
/// use atcoder_rust::math::factorize;
/// assert_eq!(factorize(12), vec![(2, 2), (3, 1)]);
/// assert_eq!(factorize(1), vec![]);
/// assert_eq!(factorize(97), vec![(97, 1)]);
/// assert_eq!(factorize(360u64), vec![(2, 3), (3, 2), (5, 1)]); // 符号なしでも可
/// ```
pub fn factorize<T: Integer>(mut n: T) -> Vec<(T, u32)> {
    let mut factors = Vec::new();
    let mut d = T::TWO;
    while d * d <= n {
        if n % d == T::ZERO {
            let mut e = 0;
            while n % d == T::ZERO {
                n /= d;
                e += 1;
            }
            factors.push((d, e));
        }
        d += T::ONE;
    }
    if n > T::ONE {
        factors.push((n, 1));
    }
    factors
}

/// `n` の約数を昇順で全列挙する。`n >= 1`。O(√n)。
/// `Integer` を実装する任意の整数型で使える。約数は i と n/i のペアで現れ
/// 片方は必ず √n 以下なので、i を √n まで回して両側を集める。
/// 答えの計算に n^3 などが要る場合は `i128` で呼べばキャストなしで書ける
/// （ただしループ内が 128bit 除算になり i64 の 2〜3 倍かかる）。
///
/// ```
/// use atcoder_rust::math::divisors;
/// assert_eq!(divisors(12), vec![1, 2, 3, 4, 6, 12]);
/// assert_eq!(divisors(1), vec![1]);
/// assert_eq!(divisors(7), vec![1, 7]);
/// assert_eq!(divisors(36i128), vec![1, 2, 3, 4, 6, 9, 12, 18, 36]);
/// assert_eq!(divisors(16u64), vec![1, 2, 4, 8, 16]); // 平方数でも √n を重複させない
/// ```
pub fn divisors<T: Integer>(n: T) -> Vec<T> {
    let mut small = Vec::new();
    let mut large = Vec::new();
    let mut i = T::ONE;
    while i * i <= n {
        if n % i == T::ZERO {
            small.push(i);
            if i != n / i {
                large.push(n / i);
            }
        }
        i += T::ONE;
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

    // 等差数列和（初項・公差・項数）。素朴なループと一致すること
    #[rstest]
    #[case(1, 1, 10, 55)] // 1+2+..+10
    #[case(2, 3, 4, 26)] // 2+5+8+11
    #[case(5, 0, 3, 15)] // 公差 0
    #[case(7, 2, 0, 0)] // 項数 0
    #[case(4, 5, 1, 4)] // 1 項なら初項そのもの
    #[case(3, -2, 4, 0)] // 公差が負で総和 0
    fn sum_of_arith_works(#[case] a: i64, #[case] d: i64, #[case] n: usize, #[case] expected: i64) {
        assert_eq!(sum_of_arith(a, d, n), expected);
        assert_eq!(sum_of_arith(a, d, n), (0..n as i64).map(|i| a + d * i).sum());
    }

    // n(n-1) が u64 を溢れる大きさでも、先に 2 で約分するので答えは正しい
    #[test]
    fn sum_of_arith_no_overflow_on_huge_n() {
        let n = 6_000_000_000usize; // n*(n-1) ≈ 3.6*10^19 > u64::MAX
        assert_eq!(sum_of_arith(0u64, 1, n), 17_999_999_997_000_000_000);
        // 項数が奇数側の経路（n が奇数）も通す
        assert_eq!(sum_of_arith(0u64, 1, n + 1), 18_000_000_003_000_000_000);
    }

    // 区間 [lo, hi] の総和。空区間は 0、負を含む区間でも正しい
    #[rstest]
    #[case(1, 10, 55)] // 1+2+..+10
    #[case(3, 5, 12)]
    #[case(7, 7, 7)] // 単点
    #[case(5, 3, 0)] // 空区間
    #[case(-3, 3, 0)] // 負を含み総和 0
    #[case(-5, -3, -12)] // 負のみ
    fn sum_of_range_works(#[case] lo: i64, #[case] hi: i64, #[case] expected: i64) {
        assert_eq!(sum_of_range(lo, hi), expected);
        assert_eq!(sum_of_range(lo, hi), (lo..=hi).sum());
    }

    // 符号なし型でも lo > hi でアンダーフローしない
    #[rstest]
    #[case(2, 5, 14)]
    #[case(5, 2, 0)]
    fn sum_of_range_unsigned(#[case] lo: usize, #[case] hi: usize, #[case] expected: usize) {
        assert_eq!(sum_of_range(lo, hi), expected);
    }

    // sum_of_arith_mod（区間 mod 版）と法を取る前の値が一致する
    #[test]
    fn sum_of_range_agrees_with_mod_version() {
        for (lo, hi) in [(1i64, 10), (3, 5), (7, 7), (5, 3), (1, 100_000)] {
            assert_eq!(
                Mint::new(sum_of_range(lo, hi)),
                sum_of_arith_mod(lo as i128, hi as i128)
            );
        }
    }

    // 等比数列和（非 mod）。公比 1 や負の公比でも破綻しない
    #[rstest]
    #[case(1, 2, 10, 1023)] // 2^10 - 1
    #[case(2, 3, 4, 80)] // 2+6+18+54
    #[case(3, 1, 5, 15)] // 公比 1（除算公式なら 0 除算になるケース）
    #[case(1, -2, 4, -5)] // 1-2+4-8
    #[case(7, 2, 0, 0)] // 項数 0
    #[case(7, 5, 1, 7)] // 1 項なら初項そのもの
    #[case(0, 3, 5, 0)] // 初項 0
    fn sum_of_geom_works(#[case] a: i64, #[case] r: i64, #[case] n: usize, #[case] expected: i64) {
        assert_eq!(sum_of_geom(a, r, n), expected);
    }

    // 等比数列和 mod。素朴な O(n) 累積と一致し、r ≡ 1 でも逆元不要で正しい
    #[rstest]
    #[case(1, 2, 10, 1023)]
    #[case(2, 3, 4, 80)]
    #[case(3, 1, 5, 15)] // 公比 1
    #[case(3, 1_000_000_008, 5, 15)] // r ≡ 1 (mod MOD)：r == 1 だけ弾く実装が壊れる境界
    #[case(1, 2, 0, 0)] // 項数 0
    #[case(1, 2, 100, 976_371_284)] // (2^100 - 1) mod MOD：pow が効く長さ
    #[case(-1, 2, 3, 1_000_000_000)] // 初項が負でも [0, MOD) に寄る（= MOD - 7）
    fn sum_of_geom_mod_works(#[case] a: i64, #[case] r: i64, #[case] n: usize, #[case] expected: u32) {
        assert_eq!(sum_of_geom_mod(a, r, n).val(), expected);
    }

    // 閉じた式（逆元を使う経路）が素朴な 1 項ずつの累積と一致する
    #[test]
    fn sum_of_geom_mod_agrees_with_naive() {
        let (a, r) = (5i64, 7i64);
        let mut naive = Mint::new(0);
        let mut term = Mint::new(a);
        for n in 0..50 {
            assert_eq!(sum_of_geom_mod(a, r, n), naive, "n={n}");
            naive = naive + term;
            term = term * Mint::new(r);
        }
    }

    // 等差数列和 mod。空区間は 0、10^18 規模でも i128 でオーバーフローしない
    #[rstest]
    #[case(1, 10, 55)]
    #[case(3, 5, 12)]
    #[case(7, 7, 7)] // 単点
    #[case(5, 3, 0)] // 空区間
    #[case(1, 100_000, 49965)] // (1+10^5)*10^5/2 = 5000050000 mod (10^9+7)
    #[case(1, 1_000_000_000_000_000_000, 1225)] // lo=1, hi=10^18: 積が i64 を溢れる領域
    fn sum_of_arith_mod_works(#[case] lo: i128, #[case] hi: i128, #[case] expected: u32) {
        assert_eq!(sum_of_arith_mod(lo, hi).val(), expected);
    }

    // Comb（ModInt 版）がパスカルの三角形と一致する
    #[test]
    fn comb_agrees_with_pascal() {
        let n_max = 30;
        let c = Comb::new(n_max);
        let mut pascal = vec![vec![Mint::new(0); n_max + 1]; n_max + 1];
        for n in 0..=n_max {
            pascal[n][0] = Mint::new(1);
            for r in 1..=n {
                pascal[n][r] = pascal[n - 1][r - 1] + pascal[n - 1][r];
            }
        }
        for n in 0..=n_max {
            for r in 0..=n_max {
                let expected = if r <= n { pascal[n][r] } else { Mint::new(0) };
                assert_eq!(c.comb(n, r), expected, "n={n} r={r}");
            }
        }
    }

    // nPr = nCr * r!。逆階乗テーブルの伝播が正しいことの確認
    #[test]
    fn perm_agrees_with_comb_times_factorial() {
        let c = Comb::new(100);
        for (n, r) in [(5usize, 2usize), (10, 0), (10, 10), (100, 37), (3, 5)] {
            let fact_r = (1..=r.min(n)).fold(Mint::new(1), |acc, i| acc * Mint::new(i));
            let expected = if r > n { Mint::new(0) } else { c.comb(n, r) * fact_r };
            assert_eq!(c.perm(n, r), expected, "n={n} r={r}");
        }
    }

    // gcd / lcm / factorize が符号なし型でもそのまま呼べる（Integer 化の確認）
    #[test]
    fn number_theory_is_generic() {
        assert_eq!(gcd(12usize, 18), 6);
        assert_eq!(gcd(-12i64, 18), 6);
        assert_eq!(lcm(4u32, 6), 12);
        assert_eq!(factorize(360u64), vec![(2, 3), (3, 2), (5, 1)]);
        assert_eq!(factorize(1i128), vec![]);
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
