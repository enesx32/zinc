#!/usr/bin/env bash
set -e

echo "==> Setting up .cargo/config.toml..."
mkdir -p .cargo
cat << 'EOF' > .cargo/config.toml
[build]
target = "x86_64-unknown-none"

[target.x86_64-unknown-none]
rustflags = [
    "-C", "link-arg=-Tkernel/linker.ld",
    "-C", "relocation-model=static",
]
EOF

echo "==> Building kernel..."
cargo build -p zinc-kernel --release

echo "==> Creating raw kernel binary..."
mkdir -p build
rust-objcopy -O binary target/x86_64-unknown-none/release/zinc-kernel build/kernel.bin

echo "==> Success! Output saved to build/kernel.bin"