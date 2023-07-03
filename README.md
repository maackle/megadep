# Setup

```
rustup override set nightly
rustup component add rustc-dev llvm-tools
cargo run --example alphabeta
```

May also need to create `.cargo/config.toml` with the following (adjust for your system):

```
[env]
CFG_COMPILER_HOST_TRIPLE = "x86_64-unknown-linux-gnu"
CFG_RELEASE = "1.72.0"
CFG_RELEASE_CHANNEL = "nightly"
RUSTC_INSTALL_BINDIR = "/tmp"
```