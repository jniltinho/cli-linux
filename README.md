# Install RustLang on Debian 10

```bash
sudo apt-get update
sudo apt-get install curl git gcc xz-utils
cd $HOME
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> $HOME/.profile
source $HOME/.profile
```

## Run cli-linux

```bash
git clone https://github.com/jniltinho/cli-linux.git
cd cli-linux
cargo build --release
target/release/cli-linux --name "Teste"
target/release/cli-linux -h
target/release/cli-linux -V
```

## Run UPX on Rust binary

```bash
curl -LO https://github.com/upx/upx/releases/download/v3.96/upx-3.96-amd64_linux.tar.xz
tar -xf upx-3.96-amd64_linux.tar.xz
cp upx-3.96-amd64_linux/upx /usr/local/bin/
rm -rf upx-3.96-*
ls -sh target/release/cli-linux
upx target/release/cli-linux
ls -sh target/release/cli-linux
target/release/cli-linux --help
```

## Rust Lang Cross Compilation

```bash
## Install Docker Ubuntu/Debian
apt-get update
apt-get install -y apt-transport-https ca-certificates curl software-properties-common docker.io socat

cargo install cross
cross build --release --target x86_64-pc-windows-gnu
```

## Links

* https://tech.fpcomplete.com/rust/command-line-parsing-clap
* https://www.rust-lang.org/tools/install
* https://crates.io/crates/cmd_lib
* https://github.com/rust-shell-script/rust_cmd_lib
* https://github.com/rust-embedded/cross
