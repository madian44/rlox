.PHONY: build

build:
	cargo build

trace:
	cargo clippy --tests --features debug_trace_execution,debug_chunk
	cargo run --features debug_trace_execution,debug_chunk

clean:
	rm -f lox*profraw
	cargo clean

clippy:
	cargo clippy --tests --features debug_chunk

test:
	cargo test --features debug_chunk -- --nocapture

coverage:
	rm -f lox*profraw
	rm -rf target/debug/coverage
	export RUSTFLAGS="-Cinstrument-coverage" ; export LLVM_PROFILE_FILE="lox-%p-%Nm.profraw" ; cargo test
	grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
	# firefox target/debug/coverage/index.html
