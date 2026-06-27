use permutohedron::LexicalPermutation;

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
