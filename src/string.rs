pub fn contains_palindrome(s: String, k: usize) -> bool {
    for i in 0..=s.len() - k {
        if is_palindrome(&s[i..i + k]) {
            return true;
        }
    }
    false
}

pub fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}
