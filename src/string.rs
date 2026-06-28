/// 長さ `k` の連続部分文字列に回文が 1 つでも含まれるか判定する。
///
/// `k` は `s.len()` 以下であること(超えると添字計算がアンダーフローする)。
pub fn contains_palindrome(s: String, k: usize) -> bool {
    for i in 0..=s.len() - k {
        if is_palindrome(&s[i..i + k]) {
            return true;
        }
    }
    false
}

/// `s` が回文かどうかを判定する。
pub fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("", true)]
    #[case("a", true)]
    #[case("aba", true)]
    #[case("abba", true)]
    #[case("abc", false)]
    #[case("ab", false)]
    fn is_palindrome_cases(#[case] s: &str, #[case] expected: bool) {
        assert_eq!(is_palindrome(s), expected);
    }

    // 長さ k の回文部分文字列を含むか
    #[rstest]
    #[case("abacaba", 3, true)] // "aba" など
    #[case("abc", 3, false)]
    #[case("abc", 1, true)] // 長さ1は常に回文
    #[case("abccba", 2, true)] // "cc"
    #[case("abcdef", 2, false)]
    fn contains_palindrome_cases(#[case] s: &str, #[case] k: usize, #[case] expected: bool) {
        assert_eq!(contains_palindrome(s.to_string(), k), expected);
    }
}
