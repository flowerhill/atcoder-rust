use itertools::Itertools;
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

/// 2 次元配列を「1 行 1 行、要素を `sep` 区切り」で並べた 1 つの文字列にする（末尾に改行は付けない）。
///
/// 数値グリッドなら `sep` は `" "`、文字グリッドなら `""` を使う。
///
/// ```
/// use atcoder_rust::io::format_grid;
///
/// assert_eq!(format_grid(&[vec![1, 2], vec![3, 4]], " "), "1 2\n3 4");
/// assert_eq!(format_grid(&[vec!['#', '.'], vec!['.', '#']], ""), "#.\n.#");
/// assert_eq!(format_grid::<i32>(&[], " "), "");
/// ```
pub fn format_grid<T: Display>(grid: &[Vec<T>], sep: &str) -> String {
    grid.iter().map(|row| row.iter().join(sep)).join("\n")
}

/// 2 次元配列を 1 行ずつ標準出力する。`println!("{}", format_grid(grid, sep))` と同じ。
pub fn print_grid<T: Display>(grid: &[Vec<T>], sep: &str) {
    println!("{}", format_grid(grid, sep));
}

/// 1 行ぶんの出力にできるタプル。要素を `sep` で連結して 1 行にする。
///
/// 「1 行に複数の値を出す」解答で、`solve` 側に `format!` を書かずに
/// `Vec<(u64, u64)>` のまま `print_tuples` へ渡すためのトレイト。2〜4 要素のタプルに実装済み。
pub trait Tuple {
    fn format_tuple(&self, sep: &str) -> String;
}

macro_rules! impl_tuple {
    ($($name:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($name: Display),+> Tuple for ($($name,)+) {
            fn format_tuple(&self, sep: &str) -> String {
                let ($($name,)+) = self;
                [$($name.to_string()),+].join(sep)
            }
        }
    };
}

impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);

/// タプルの列を「1 行 1 タプル、要素を `sep` 区切り」で並べた 1 つの文字列にする（末尾に改行は付けない）。
///
/// ```
/// use atcoder_rust::io::format_tuples;
///
/// assert_eq!(format_tuples(&[(1, 2), (3, 4)], " "), "1 2\n3 4");
/// assert_eq!(format_tuples(&[(1, 'a', "x")], " "), "1 a x"); // 型が混ざってもよい
/// assert_eq!(format_tuples::<(i32, i32)>(&[], " "), "");
/// ```
pub fn format_tuples<T: Tuple>(xs: &[T], sep: &str) -> String {
    xs.iter().map(|x| x.format_tuple(sep)).join("\n")
}

/// タプルの列を 1 行ずつ標準出力する。`println!("{}", format_tuples(xs, sep))` と同じ。
pub fn print_tuples<T: Tuple>(xs: &[T], sep: &str) {
    println!("{}", format_tuples(xs, sep));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1, 2, 3], vec![4, 5, 6]], " ", "1 2 3\n4 5 6")] // 非正方（行と列の取り違え検出）
    #[case(vec![vec![1]], " ", "1")] // 1x1 は区切りも改行も出ない
    #[case(vec![vec![1], vec![2]], " ", "1\n2")] // 1 列でも区切りは出ない
    #[case(vec![], " ", "")] // 空グリッド
    #[case(vec![vec![], vec![]], " ", "\n")] // 幅 0 の行だけ → 空行が行数ぶん
    #[case(vec![vec![1, 2]], ", ", "1, 2")] // 複数文字の区切り
    fn format_grid_cases(#[case] grid: Vec<Vec<i32>>, #[case] sep: &str, #[case] expected: &str) {
        assert_eq!(format_grid(&grid, sep), expected);
    }

    #[test]
    fn format_grid_joins_chars_without_sep() {
        let grid = vec![vec!['a', 'b'], vec!['c', 'd']];
        assert_eq!(format_grid(&grid, ""), "ab\ncd");
    }

    #[rstest]
    #[case(vec![(63, 261)], "63 261")] // 1 行だけなら改行は入らない
    #[case(vec![(1, 2), (3, 4)], "1 2\n3 4")] // 複数行
    #[case(vec![(0, 0)], "0 0")] // 0 も欠落しない
    #[case(vec![], "")] // 空
    fn format_rows_cases(#[case] xs: Vec<(u64, u64)>, #[case] expected: &str) {
        assert_eq!(format_tuples(&xs, " "), expected);
    }

    #[test]
    fn format_rows_supports_arity_3_and_4() {
        assert_eq!(format_tuples(&[(1, 2, 3)], " "), "1 2 3");
        assert_eq!(format_tuples(&[(1, 2, 3, 4)], ","), "1,2,3,4");
    }

    #[test]
    fn format_rows_allows_mixed_display_types() {
        // 数値と文字・文字列が混ざったタプルでも 1 行になる
        assert_eq!(format_tuples(&[(1u64, 'x', "abc")], " "), "1 x abc");
    }
}
