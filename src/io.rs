use std::fmt::Display;

/// Bool を AtCoder の "Yes"/"No" 文字列に変換する
///
/// `#[fastout]` は付けた関数内の `println!` だけを高速化バッファに置き換えるため、
/// ライブラリ側で print すると出力順が壊れうる。print は main に残し、変換だけを提供する。
/// 使い方: `println!("{}", yn(solve(a)));`
///
/// ```
/// use atcoder_rust::io::yn;
/// assert_eq!(yn(true), "Yes");
/// assert_eq!(yn(false), "No");
/// ```
pub fn yn(f: bool) -> &'static str {
    if f {
        "Yes"
    } else {
        "No"
    }
}

/// 各要素を1行ずつ標準出力する（AtCoder の複数行出力テンプレ用）
pub fn print_lines<T: Display>(xs: &[T]) {
    println!(
        "{}",
        xs.iter().map(T::to_string).collect::<Vec<_>>().join("\n")
    );
}
