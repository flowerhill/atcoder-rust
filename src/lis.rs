/// 各 `i` について「`seq[i]` で終わる**狭義増加**部分列の最大長」を返す（O(N log N)）。
///
/// 列全体の LIS 長は戻り値の最大値。向きや狭義/広義は要素の包み方で切り替える:
/// - 狭義減少: `Reverse(x)` で包む
/// - 広義増加: `(x, i)` で包む（添字が同値の並びを狭義にする）
/// - 各位置から**始まる**列: 逆順の列に適用して結果を反転する
///
/// ```
/// use atcoder_rust::lis::lis_lengths;
/// use std::cmp::Reverse;
///
/// let a = [3, 1, 4, 1, 5, 9, 2, 6];
/// assert_eq!(lis_lengths(&a), vec![1, 1, 2, 1, 3, 4, 2, 4]);
/// assert_eq!(lis_lengths(&a).into_iter().max(), Some(4)); // 1, 4, 5, 9 など
///
/// // 狭義減少
/// let rev: Vec<_> = a.iter().map(|&x| Reverse(x)).collect();
/// assert_eq!(lis_lengths(&rev), vec![1, 2, 1, 2, 1, 1, 2, 2]);
///
/// // 広義増加（同値も続けて取れる）
/// let b = [2, 2, 1, 2];
/// let pairs: Vec<_> = b.iter().enumerate().map(|(i, &x)| (x, i)).collect();
/// assert_eq!(lis_lengths(&pairs), vec![1, 2, 1, 3]);
/// ```
pub fn lis_lengths<T: Ord>(seq: &[T]) -> Vec<usize> {
    // tails[p] = 長さ p+1 の増加部分列の末尾としてありうる最小の要素（常に狭義増加）
    let mut tails: Vec<&T> = Vec::new();
    let mut lens = Vec::with_capacity(seq.len());
    for x in seq {
        // x 未満の要素数 p: 長さ p の列の後ろに x を継げる（同値には継げない = 狭義）
        let p = tails.partition_point(|&t| t < x);
        if p == tails.len() {
            tails.push(x);
        } else {
            tails[p] = x;
        }
        lens.push(p + 1);
    }
    lens
}

/// 最長の狭義増加部分列を 1 本復元し、その添字を昇順で返す（O(N log N)）。
///
/// 値が欲しければ `seq[i]` で引く。狭義/広義・減少の切り替えは [`lis_lengths`] と同じく
/// 要素の包み方で行い、返る添字は包む前の列の添字としてそのまま使える。
///
/// ```
/// use atcoder_rust::lis::lis_indices;
///
/// let a = [3, 1, 4, 1, 5, 9, 2, 6];
/// let idx = lis_indices(&a);
/// assert_eq!(idx, vec![1, 2, 4, 7]);
/// assert_eq!(idx.iter().map(|&i| a[i]).collect::<Vec<_>>(), vec![1, 4, 5, 6]);
/// assert!(lis_indices::<i32>(&[]).is_empty());
/// ```
pub fn lis_indices<T: Ord>(seq: &[T]) -> Vec<usize> {
    let lens = lis_lengths(seq);
    let mut need = lens.iter().copied().max().unwrap_or(0);
    let mut picked = Vec::with_capacity(need);
    // 右から、長さ need で終わる位置のうち最も右のものを拾い、need を 1 ずつ減らす。
    // 同じ長さの位置どうしは右ほど値が小さい（大きければ長さが伸びている）ので、
    // 最も右の「長さ need-1」は必ず直前に拾った値より小さく、値の比較は要らない。
    for i in (0..seq.len()).rev() {
        if lens[i] == need {
            picked.push(i);
            need -= 1;
        }
    }
    picked.reverse();
    picked
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&[], vec![])] // 空列
    #[case(&[7], vec![1])] // 1 要素
    #[case(&[1, 2, 3], vec![1, 2, 3])] // 全体が増加
    #[case(&[3, 2, 1], vec![1, 1, 1])] // 全体が減少
    #[case(&[2, 2, 2], vec![1, 1, 1])] // 同値は狭義なので継げない
    #[case(&[1, 3, 2, 3], vec![1, 2, 2, 3])] // 3 を 2 で置き換えてから 3 が継げる
    #[case(&[5, 1, 2, 3], vec![1, 1, 2, 3])] // 先頭の大きい値を 1 で置き換える
    fn lis_lengths_cases(#[case] seq: &[i32], #[case] expected: Vec<usize>) {
        assert_eq!(lis_lengths(seq), expected);
    }

    /// 長さ 0..=6・値 0..3 の全ての列
    fn all_small_seqs() -> impl Iterator<Item = Vec<usize>> {
        (0..=6u32).flat_map(|n| {
            (0..3usize.pow(n)).map(move |code| (0..n).map(|k| code / 3usize.pow(k) % 3).collect())
        })
    }

    /// O(N^2) の素朴な DP: dp[i] = a[i] で終わる狭義増加部分列の最大長
    fn naive_dp(seq: &[usize]) -> Vec<usize> {
        let mut dp = vec![1; seq.len()];
        for i in 0..seq.len() {
            for j in 0..i {
                if seq[j] < seq[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        dp
    }

    #[test]
    fn lis_lengths_matches_naive_dp() {
        for seq in all_small_seqs() {
            assert_eq!(lis_lengths(&seq), naive_dp(&seq), "seq = {:?}", seq);
        }
    }

    #[rstest]
    #[case(&[], vec![])] // 空列
    #[case(&[7], vec![0])] // 1 要素
    #[case(&[1, 2, 3], vec![0, 1, 2])] // 全体が増加 → 全部
    #[case(&[3, 2, 1], vec![2])] // 全体が減少 → 最も右の 1 個
    #[case(&[2, 2, 2], vec![2])] // 同値は 1 個しか取れない
    #[case(&[3, 1, 4, 1, 5, 9, 2, 6], vec![1, 2, 4, 7])] // tails の最終形 [1,2,5,6] とは別物
    #[case(&[1, 5, 2, 3], vec![0, 2, 3])] // 長さ 2 の候補 5 と 2 のうち、3 に継げる 2 を拾う
    fn lis_indices_cases(#[case] seq: &[i32], #[case] expected: Vec<usize>) {
        assert_eq!(lis_indices(seq), expected);
    }

    /// 全ての小さい列で、復元結果が「添字も値も狭義増加」かつ「長さが最大」であるか
    #[test]
    fn lis_indices_is_valid_and_longest() {
        for seq in all_small_seqs() {
            let idx = lis_indices(&seq);
            assert!(
                idx.windows(2).all(|w| w[0] < w[1] && seq[w[0]] < seq[w[1]]),
                "seq = {:?}, idx = {:?}",
                seq,
                idx
            );
            assert_eq!(
                idx.len(),
                naive_dp(&seq).into_iter().max().unwrap_or(0),
                "seq = {:?}",
                seq
            );
        }
    }
}
