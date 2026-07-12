# atcoder-rust

AtCoder 用の Rust 競技プログラミング環境。自作ライブラリを `main.rs` から
`use` して解き、提出時には**使ったモジュールだけを 1 ファイルに束ねて**出力する。

## 必要環境

- Rust **1.89.0**（AtCoder の判定環境に合わせている）
  - rustup 利用時: `rust-toolchain.toml` で自動的に固定される
  - Nix 利用時: 同梱の `flake.nix` で rustc / cargo / rust-analyzer 1.89.0 と oj が入る（下記）

### Nix + direnv で環境構築

`flake.nix` / `flake.lock` / `.envrc` をコンテスト用ルートディレクトリ（各タスクを作る場所）にコピーして direnv を許可する:

```sh
cp flake.nix flake.lock .envrc ~/dev/atcoder/atcoder-rust/   # ルートは例
direnv allow ~/dev/atcoder/atcoder-rust
```

direnv は親ディレクトリの `.envrc` を遡って見つけるため、ルート 1 か所に置けば配下の全タスクディレクトリで環境が自動で有効になる（`template.json` の `static` には意図的に含めていない）。個人用の環境変数は同じ場所の `.envrc.local` に書く（git には含めない）。ジャッジの Rust バージョンが上がったときは `flake.nix` の `"1.89.0"` と `rust-toolchain.toml` を揃えて更新する。

## ディレクトリ構成

```
src/
├── main.rs          # 解答を書く場所（提出のエントリポイント）
├── lib.rs           # ライブラリのモジュール宣言
├── bin/
│   └── bundle.rs    # 提出用バンドラ
│
├── bsearch.rs       # 二分探索 / lower_bound / upper_bound
├── cmp.rs           # chmax / chmin
├── graph.rs         # 隣接リスト構築（build_undirected_graph）/ dfs / bfs / dijkstra / 木 DP（tree_order・tree_dp）
├── grid.rs          # グリッド用の方向定数（DIRS4 / DIRS8）
├── io.rs            # 出力ヘルパ（yn / print_lines など）
├── math.rs          # Integer トレイト / mod 演算（add_mod・modpow・Comb など）/ 桁変換
├── pair.rs          # Pair<T>（成分ごとの加減算）
├── seq.rs           # 部分列判定 / 順列列挙
├── string.rs        # 回文判定など
├── sumunionfind.rs  # 和の制約 x_a + x_b = c 用の符号付き Union-Find（SumUnionFind）
├── unionfind.rs     # Union-Find
└── wunionfind.rs    # 重み付き Union-Find（差 x_j - x_i = w）
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

# コンテスト作成後にタスクを個別追加する場合も同様
acc add a --template atcoder-rust
```

`acc add` / `acc new` で Rust を使うときは `--template atcoder-rust` を付ける。
毎回指定したくなければデフォルトテンプレートを切り替えておけば `--template` を
省略できる:

```sh
acc config default-template atcoder-rust
```

各タスクは独立した cargo プロジェクトになる。主な `make` ターゲット:

| コマンド | 動作 |
| --- | --- |
| `make test` | oj でサンプルテスト |
| `make bundle` | `submit.rs` を生成 |
| `make tb` | test → bundle |
| `make submit` | test → bundle → 提出（テスト失敗時は提出しない） |
| `make submit-force` | test 無しで bundle → 提出 |
| `make verify-bundle` | 提出ファイルが依存込みでコンパイルできるか検証 |
| `make clean` | `submit.rs` と `target` を削除 |
