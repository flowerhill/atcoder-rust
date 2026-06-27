# AtCoder Rust テンプレート用タスク Makefile
BIN := atcoder-rust

# --- 単体ターゲット ---

# oj でサンプルテスト（release ビルドした実行ファイルで実行）
test:
	cargo build -q --release --bin $(BIN)
	oj test -c "./target/release/$(BIN)" -d test

# 提出用の 1 ファイルを生成（main.rs が使う自作モジュールを束ねる）
bundle:
	cargo run -q --bin bundle > submit.rs

# 提出用ファイルが（外部 crate 込みで）コンパイルできるか検証。
# submit.rs を一時的に cargo の bin として置き、依存込みでビルドする。
verify-bundle: bundle
	@cp submit.rs src/bin/_verify_submit.rs
	@cargo build -q --bin _verify_submit; st=$$?; rm -f src/bin/_verify_submit.rs; \
	  if [ $$st -eq 0 ]; then echo "verify-bundle: OK"; else echo "verify-bundle: FAILED"; fi; exit $$st

# --- 連結ターゲット（make は前提を左から順に実行し、失敗で中断する）---

# テスト → バンドル
tb test-bundle: test bundle

# テスト → バンドル → acc で提出（テストが落ちたら提出しない）
submit: test bundle
	acc submit -- -y

# テスト無しでバンドル → 提出
submit-force: bundle
	acc submit -- -y

# --- その他 ---

clean:
	rm -f submit.rs
	cargo clean

help:
	@echo "make test          - サンプルテスト (oj)"
	@echo "make bundle        - submit.rs を生成"
	@echo "make tb            - test → bundle"
	@echo "make submit        - test → bundle → 提出 (テスト失敗時は提出しない)"
	@echo "make submit-force  - test 無しで bundle → 提出"
	@echo "make verify-bundle - submit.rs が単体でコンパイルできるか確認"
	@echo "make clean         - submit.rs と target を削除"

.PHONY: test bundle verify-bundle tb test-bundle submit submit-force clean help
