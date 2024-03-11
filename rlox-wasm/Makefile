.PHONY: build 

OUT_DIR ?= '../rlox-vsce/out/wasm'

build: 
	wasm-pack build --dev --target nodejs --out-dir $(OUT_DIR)
	rm -f $(OUT_DIR)/.gitignore $(OUT_DIR)/package.json

clean:
	cargo clean
	$(MAKE) -C ../rlox clean

clippy:
	cargo clippy
	$(MAKE) -C ../rlox clippy

test:
	cargo test
	$(MAKE) -C ../rlox test

init:
	cargo update

coverage:
	rm -f lox*profraw
	rm -rf target/debug/coverage
	export RUSTFLAGS="-Cinstrument-coverage" ; export LLVM_PROFILE_FILE="lox-%p-%Nm.profraw" ; cargo test
	grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
	# firefox target/debug/coverage/index.html
