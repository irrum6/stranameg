build:
	cargo build
release:
	cargo build --release
movebin:
	cp ./target/release/stranameg ./binary
remo: release movebin