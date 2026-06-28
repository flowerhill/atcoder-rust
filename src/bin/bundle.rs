//! AtCoder 提出用に `main.rs` と、そこから使っている自作ライブラリモジュールを
//! 1 ファイルに束ねて標準出力へ書き出す。
//!
//! 使い方:
//!   cargo run -q --bin bundle            # 標準出力に表示
//!   cargo run -q --bin bundle | pbcopy   # クリップボードへ
//!
//! 仕組み:
//!   - `main.rs` 内の `atcoder_rust::<mod>` 参照を集める
//!   - 参照されたモジュール(`src/<mod>.rs`)を、モジュール間依存(`crate::<mod>`)も
//!     辿って収集する
//!   - 各モジュールを `mod <名> { ... }` として `main` の末尾に展開し、
//!     `atcoder_rust::` を `crate::` に置換、`#[cfg(test)]` 項目を除去する
//!   - `use itertools::..` などの外部 crate はそのまま残す(AtCoder 環境に存在する)

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

/// クレートのルートディレクトリ(コンパイル時に確定)。
const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");

/// クレートの `src/` ディレクトリの絶対パスを返す。
fn src_dir() -> PathBuf {
    Path::new(CRATE_DIR).join("src")
}

/// `src/` 直下のライブラリモジュール名一覧(main.rs / lib.rs / bin/ を除く)。
fn available_modules() -> BTreeSet<String> {
    let mut mods = BTreeSet::new();
    for entry in fs::read_dir(src_dir()).expect("read src/") {
        let path = entry.expect("dir entry").path();
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        let stem = path.file_stem().unwrap().to_str().unwrap().to_string();
        if stem == "main" || stem == "lib" {
            continue;
        }
        mods.insert(stem);
    }
    mods
}

/// `src` 中の `crate::<ident>` を走査し、`available` に含まれるものを `out` に集める。
fn collect_crate_refs(src: &str, available: &BTreeSet<String>, out: &mut BTreeSet<String>) {
    const PAT: &str = "crate::";
    let mut rest = src;
    while let Some(pos) = rest.find(PAT) {
        let after = &rest[pos + PAT.len()..];
        let ident: String = after
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if available.contains(&ident) {
            out.insert(ident);
        }
        rest = after;
    }
}

/// `#[cfg(test)]` が付いた項目(`mod tests { .. }` や `fn` など)を波括弧対応で除去する。
fn strip_cfg_test(src: &str) -> String {
    const ATTR: &str = "#[cfg(test)]";
    let mut out = String::with_capacity(src.len());
    let mut rest = src;
    while let Some(pos) = rest.find(ATTR) {
        out.push_str(&rest[..pos]);
        let after = &rest[pos + ATTR.len()..];
        // 次の `{` を探し、対応する `}` までを丸ごと飛ばす。
        match after.find('{') {
            Some(brace) => {
                let mut depth = 0usize;
                let mut end = None;
                for (i, c) in after[brace..].char_indices() {
                    match c {
                        '{' => depth += 1,
                        '}' => {
                            depth -= 1;
                            if depth == 0 {
                                end = Some(brace + i + c.len_utf8());
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                match end {
                    Some(e) => rest = &after[e..],
                    None => {
                        // 対応する括弧が見つからない場合は諦めてそのまま出力。
                        out.push_str(ATTR);
                        rest = after;
                    }
                }
            }
            None => {
                out.push_str(ATTR);
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

fn main() {
    let available = available_modules();

    let main_src = fs::read_to_string(src_dir().join("main.rs")).expect("read src/main.rs");
    // `atcoder_rust::` を `crate::` に置換してから依存解決する。
    let main_src = main_src.replace("atcoder_rust::", "crate::");
    let main_src = strip_cfg_test(&main_src);

    // main から参照されるモジュールを起点に、依存を推移的に収集する。
    let mut needed = BTreeSet::new();
    collect_crate_refs(&main_src, &available, &mut needed);

    let mut worklist: Vec<String> = needed.iter().cloned().collect();
    let mut module_src: std::collections::BTreeMap<String, String> = Default::default();
    while let Some(m) = worklist.pop() {
        let path = src_dir().join(format!("{m}.rs"));
        let body = fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {}", path.display()));
        let body = strip_cfg_test(&body);
        let mut deps = BTreeSet::new();
        collect_crate_refs(&body, &available, &mut deps);
        for d in deps {
            if needed.insert(d.clone()) {
                worklist.push(d);
            }
        }
        module_src.insert(m, body);
    }

    // 出力を組み立てる。
    let mut out = String::new();
    out.push_str(main_src.trim_end());
    out.push('\n');
    for (name, body) in &module_src {
        out.push_str(&format!("\n// ===== src/{name}.rs =====\n"));
        out.push_str(&format!("mod {name} {{\n"));
        out.push_str(body.trim_matches('\n'));
        out.push_str("\n}\n");
    }

    print!("{out}");
}
