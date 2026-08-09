use permutohedron::LexicalPermutation;

/// `subseq` が `seq` の(連続とは限らない)部分列かどうかを判定する。
pub fn is_subsequence_of<T: PartialEq>(subseq: &[T], seq: &[T]) -> bool {
    let mut subseq_iter = subseq.iter();
    let mut current_subseq_item = subseq_iter.next();

    for seq_item in seq {
        if let Some(subseq_item) = current_subseq_item {
            if seq_item == subseq_item {
                current_subseq_item = subseq_iter.next();
            }
        } else {
            break;
        }
    }
    current_subseq_item.is_none()
}

/// 連長圧縮(ランレングス符号化)。連続して等しい要素を `(要素, 連続する個数)` にまとめる。
///
/// ```
/// use atcoder_rust::seq::run_length;
///
/// assert_eq!(
///     run_length(&['a', 'a', 'b', 'a']),
///     vec![(&'a', 2), (&'b', 1), (&'a', 1)]
/// );
/// assert!(run_length::<char>(&[]).is_empty());
/// ```
pub fn run_length<T: PartialEq>(seq: &[T]) -> Vec<(&T, usize)> {
    seq.iter().fold(vec![], |mut runs, x| {
        match runs.last_mut() {
            Some((y, count)) if *y == x => *count += 1,
            _ => runs.push((x, 1)),
        }
        runs
    })
}

/// `seq` の長さ `k` の部分列のうち、辞書順最小のものを返す。
///
/// 単調スタック法。左から要素を積みながら、「スタック末尾より小さい要素が来た」かつ
/// 「末尾を捨てても残りの要素数で長さ `k` を満たせる」あいだ末尾を捨てる。
/// `k <= seq.len()` であること(足りないと長さ `k` 未満の列が返る)。
///
/// ```
/// use atcoder_rust::seq::smallest_subsequence;
///
/// let s: Vec<char> = "atcoder".chars().collect();
/// assert_eq!(smallest_subsequence(&s, 3), vec!['a', 'c', 'd']);
/// assert_eq!(smallest_subsequence(&[3, 1, 2], 2), vec![1, 2]);
/// ```
pub fn smallest_subsequence<T: Ord + Copy>(seq: &[T], k: usize) -> Vec<T> {
    seq.iter()
        .enumerate()
        .fold(Vec::with_capacity(k), |mut stack, (i, &x)| {
            while stack.len() + (seq.len() - i) > k && stack.last().is_some_and(|&top| top > x) {
                stack.pop();
            }
            if stack.len() < k {
                stack.push(x);
            }
            stack
        })
}

/// 文字の多重集合 `cs` から、重複を除いた全順列を辞書順で生成する。
pub fn distinct_permutation(mut cs: Vec<char>) -> Vec<String> {
    cs.sort();
    let mut v = vec![];
    loop {
        v.push(cs.clone().into_iter().collect());
        if !cs.next_permutation() {
            break;
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&[1, 3], &[1, 2, 3, 4], true)]
    #[case(&[1, 4], &[1, 2, 3, 4], true)]
    #[case(&[2, 1], &[1, 2, 3, 4], false)] // 順序が逆
    #[case(&[], &[1, 2, 3], true)] // 空列は常に部分列
    #[case(&[1, 2, 3], &[1, 2], false)] // 長すぎる
    #[case(&[1, 1], &[1, 2, 1], true)] // 重複あり
    fn is_subsequence_cases(#[case] sub: &[i32], #[case] seq: &[i32], #[case] expected: bool) {
        assert_eq!(is_subsequence_of(sub, seq), expected);
    }

    #[rstest]
    #[case(&[], vec![])] // 空列
    #[case(&[1], vec![(1, 1)])] // 1 要素
    #[case(&[1, 1, 1], vec![(1, 3)])] // 全て同じ
    #[case(&[1, 2, 3], vec![(1, 1), (2, 1), (3, 1)])] // 全て異なる
    #[case(&[1, 1, 2, 1], vec![(1, 2), (2, 1), (1, 1)])] // 同じ値が離れて再登場
    fn run_length_cases(#[case] seq: &[i32], #[case] expected: Vec<(i32, usize)>) {
        let expected: Vec<(&i32, usize)> = expected.iter().map(|(x, c)| (x, *c)).collect();
        assert_eq!(run_length(seq), expected);
    }

    #[test]
    fn run_length_preserves_total_len() {
        let seq = ['o', 'o', 'x', 'o', 'x', 'x', 'x'];
        assert_eq!(run_length(&seq).iter().map(|&(_, c)| c).sum::<usize>(), seq.len());
    }

    #[rstest]
    #[case("atcoder", 3, "acd")] // サンプル1
    #[case("kittyonyourlap", 5, "inlap")] // サンプル2
    #[case("abc", 3, "abc")] // k = n なら元の列そのまま
    #[case("cba", 3, "cba")] // k = n は降順でも並べ替えない
    #[case("cba", 1, "a")] // 降順なら最後の 1 文字
    #[case("abc", 1, "a")] // 昇順なら先頭の 1 文字
    #[case("aaa", 2, "aa")] // 全て同じ(等しい要素は捨てない)
    #[case("bab", 2, "ab")] // 末尾の b は残り個数の制約で必ず採用
    fn smallest_subsequence_cases(#[case] s: &str, #[case] k: usize, #[case] expected: &str) {
        let cs: Vec<char> = s.chars().collect();
        assert_eq!(
            smallest_subsequence(&cs, k).into_iter().collect::<String>(),
            expected
        );
    }

    #[test]
    fn distinct_permutation_no_dup() {
        let perms = distinct_permutation(vec!['a', 'b']);
        assert_eq!(perms, vec!["ab".to_string(), "ba".to_string()]);
    }

    #[test]
    fn distinct_permutation_dedups_repeats() {
        // "aab" の異なる順列は 3 通りのみ（3! ではない）
        let perms = distinct_permutation(vec!['a', 'a', 'b']);
        assert_eq!(perms, vec!["aab", "aba", "baa"]);
    }

    #[rstest]
    #[case(vec!['a'], 1)]
    #[case(vec!['a', 'b'], 2)]
    #[case(vec!['a', 'b', 'c'], 6)]
    #[case(vec!['a', 'a'], 1)]
    fn distinct_permutation_count(#[case] cs: Vec<char>, #[case] expected: usize) {
        assert_eq!(distinct_permutation(cs).len(), expected);
    }
}
