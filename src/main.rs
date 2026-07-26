#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::marker::Usize1;
#[allow(unused_imports)]
use proconio::{fastout, input};
#[allow(unused_imports)]
use superslice::Ext;

// 自作ライブラリを使うときは use する。
// `cargo run -q --bin bundle` で、使ったモジュールだけが提出ファイルに展開される。
// 例:
//   use atcoder_rust::unionfind::UnionFind;
//   use atcoder_rust::bsearch::{LowerBound, UpperBound};

/// 解法本体。入力の読み取りは main に任せ、ここは純粋な計算に徹する
/// (テストから直接呼べるようにするため)。
fn solve() -> u64 {
    todo!()
}

#[fastout]
fn main() {
    input! {}

    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn samples() {}
}
