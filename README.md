# atcoder-rust

AtCoder 用の Rust 競技プログラミング環境。自作ライブラリを `main.rs` から
`use` して解き、提出時には**使ったモジュールだけを 1 ファイルに束ねて**出力する。

## 必要環境

- Rust **1.89.0**（`rust-toolchain.toml` で固定。AtCoder の判定環境に合わせている）

## ディレクトリ構成

```
src/
├── main.rs          # 解答を書く場所（提出のエントリポイント）
├── lib.rs           # ライブラリのモジュール宣言
├── bin/
│   └── bundle.rs    # 提出用バンドラ
│
├── bsearch.rs       # 二分探索 / lower_bound / upper_bound
├── graph.rs         # dfs / bfs / dijkstra / 部分木サイズ
├── integer.rs       # Integer トレイト（整数の総称）
├── io.rs            # 出力ヘルパ（print_lines など）
├── pair.rs          # Pair<T>（成分ごとの加減算）
├── seq.rs           # 部分列判定 / 順列列挙
├── string.rs        # 回文判定など
├── unionfind.rs     # Union-Find
└── wunionfind.rs    # 重み付き Union-Find
```

## 解答の書き方

`main.rs` で、使いたい自作ライブラリを `atcoder_rust::<module>::<item>` で `use` する。

```rust
use atcoder_rust::unionfind::UnionFind;
use atcoder_rust::bsearch::{LowerBound, UpperBound};

#[fastout]
fn main() {
    input! { n: usize }
    let mut uf = UnionFind::new(n);
    // ...
}
```

外部 crate（`itertools` / `proconio` / `ndarray` など）はそのまま使える。これらは
AtCoder の判定環境にも存在するため、提出ファイルへは展開されない。

## 提出（1 ファイル化）

```sh
cargo run -q --bin bundle            # 提出コードを標準出力へ
cargo run -q --bin bundle | pbcopy   # そのままクリップボードへ
```

`main.rs` を解析し、使用している自作モジュールを（モジュール間の依存も辿って）
末尾に `mod <名> { .. }` として展開した 1 ファイルを出力する。出てきたものを
そのまま AtCoder に貼り付ければよい。`#[cfg(test)]` のテストは除去され、外部
crate の `use` はそのまま残る。

## テスト

```sh
cargo test
```

各モジュール／`main.rs` の `#[cfg(test)]` テストはローカルで普通に実行できる
（提出ファイルには含まれない）。

## acc テンプレートとして使う

このリポジトリは [atcoder-cli (acc)](https://github.com/Tatamo/atcoder-cli) の
テンプレートとしてそのまま使える（`template.json` / `Makefile` 同梱）。

```sh
# acc の config-dir に配置
rsync -a --exclude target --exclude .git --exclude node_modules \
  ./ "$(acc config-dir)/atcoder-rust/"

# タスク生成（テンプレ指定）
acc new abc300 --template atcoder-rust
cd abc300/a   # src/main.rs を編集
```

各タスクは独立した cargo プロジェクトになる。主な `make` ターゲット:

| コマンド | 動作 |
| --- | --- |
| `make test` | oj でサンプルテスト |
| `make bundle` | `submit.rs` を生成 |
| `make tb` | test → bundle |
| `make submit` | test → bundle → 提出（テスト失敗時は提出しない） |
| `make verify-bundle` | 提出ファイルが依存込みでコンパイルできるか検証 |
