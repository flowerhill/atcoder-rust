use std::fmt::Display;

/// 各要素を1行ずつ標準出力する（AtCoder の複数行出力テンプレ用）
pub fn print_lines<T: Display>(xs: &[T]) {
    println!(
        "{}",
        xs.iter().map(T::to_string).collect::<Vec<_>>().join("\n")
    );
}
