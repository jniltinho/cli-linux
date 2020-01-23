

.PHONY: all

all: build upx


build:
	cargo build --release
	cargo fmt --all
	cargo fmt --all -- --check
	ls -sh target/release/cli-linux

build-musl:
	apt-get install -y musl-tools
	rustup target add x86_64-unknown-linux-musl
	cargo fmt --all
	cargo fmt --all -- --check
	cargo build --release --target x86_64-unknown-linux-musl
	upx target/x86_64-unknown-linux-musl/release/cli-linux
	ls -sh target/x86_64-unknown-linux-musl/release/cli-linux

upx:
	ls -sh target/release/cli-linux
	upx target/release/cli-linux
	ls -sh target/release/cli-linux


install-upx:
	curl -skLO https://github.com/upx/upx/releases/download/v3.95/upx-3.95-amd64_linux.tar.xz
	tar -xf upx-3.95-amd64_linux.tar.xz
	cp upx-3.95-amd64_linux/upx /usr/local/bin/
	rm -rf upx-3.95*


.PHONY: install-golang


install-rust: 
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	source $HOME/.profile
