//! 部分集合の列挙（bit 全探索 / 半分全列挙の下ごしらえ）。

use std::ops::Add;

/// `xs` の全部分集合の和を、選んだ要素数ごとに分類して返す。
/// 返り値 `v[c]` は「ちょうど c 個選んだときの和」の一覧（長さ C(n, c)、昇順ソート済み）。
///
/// 半分全列挙（meet in the middle）の下ごしらえに使う。列を前半・後半に分けて
/// それぞれこれを呼び、片方の和を `bsearch::UpperBound` などで引くと、
/// 「ちょうど K 個選んで和が P 以下」のような条件を `O(2^(n/2) · n)` で数えられる。
///
/// 計算量は列挙・ソートとも `O(2^n · n)`、メモリ `O(2^n)`。n は 25 程度が上限
/// （2^25 ≈ 3.4×10^7）。`itertools` の `combinations(c)` でも同じ結果は作れるが、
/// 部分集合ごとに `Vec` を確保するぶん遅い（n = 20 で実測 2.6 倍）。
///
/// ```
/// use atcoder_rust::subset::subset_sums_by_count;
///
/// // 0 個 / 1 個 / 2 個 / 3 個 選んだときの和が個数ごとに分かれる
/// assert_eq!(
///     subset_sums_by_count(&[1u64, 2, 4]),
///     vec![vec![0], vec![1, 2, 4], vec![3, 5, 6], vec![7]]
/// );
///
/// // 同じ値でも要素は区別する（重複は潰さない）
/// assert_eq!(
///     subset_sums_by_count(&[5u64, 5]),
///     vec![vec![0], vec![5, 5], vec![10]]
/// );
///
/// // 空列でも「0 個選ぶ = 和 0」の 1 通りは返る
/// assert_eq!(subset_sums_by_count::<u64>(&[]), vec![vec![0]]);
///
/// // 負の値でもよい（各段は昇順に並ぶ）
/// assert_eq!(
///     subset_sums_by_count(&[-2i64, 3]),
///     vec![vec![0], vec![-2, 3], vec![1]]
/// );
/// ```
pub fn subset_sums_by_count<T>(xs: &[T]) -> Vec<Vec<T>>
where
    T: Copy + Default + Ord + Add<Output = T>,
{
    let n = xs.len();
    // `1u64 << n` は n >= 64 で壊れる（release では折り返して 1 通りだけ数えてしまう）。
    // 実際には 2^n の列挙自体が終わらないので、ここで落として気づけるようにする。
    assert!(n < 64, "subset_sums_by_count: 要素数が多すぎる (len = {n})");

    let mut sums: Vec<Vec<T>> = (0..=n).map(|_| Vec::new()).collect();
    for bits in 0u64..1 << n {
        let s = (0..n)
            .filter(|&i| bits >> i & 1 == 1)
            .fold(T::default(), |acc, i| acc + xs[i]);
        sums[bits.count_ones() as usize].push(s);
    }
    for v in sums.iter_mut() {
        v.sort_unstable();
    }
    sums
}
