build:
	cargo build
build_release:
	cargo build --release
aarch64:
	mkdir bin
	cross build --release --target aarch64-unknown-linux-gnu
	cp target/aarch64-unknown-linux-gnu/release/alfred-idroid01 bin/
	rm -rf bin