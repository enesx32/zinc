# zinc-os

![Version](https://img.shields.io/badge/version-v4.0.2-blue)

<a href="./docs/showcase.md">
  <img src="https://img.shields.io/badge/%20View%20Showcase-FF9900?style=for-the-badge" />
</a>

A tiny hobby operating system written in Rust and x86-64 assembly.

Zinc boots from a custom BIOS bootloader, switches the CPU into 64-bit long mode, and runs a Rust kernel with its own native API, memory allocator, drivers, types, and shell.

## Features

* Custom BIOS bootloader
* Rust kernel
* x86-64 long mode
* VGA text mode output
* PS/2 keyboard input
* PIT-based timing
* Custom global memory allocator
* Dynamic heap-backed `String`
* Custom Native API
* Built-in shell
* Bootable ISO built with a small Python script
* No `xorriso` or `mkisofs` required

## Native API - Zcore

Zcore is Zinc's native API and provides the low-level components used by the kernel.

It currently includes:

* VGA text output
* Keyboard input
* PIT timing
* Memory allocation
* Custom `String` type
* System constants

The Zcore code is split into separate crates:

```text
zcore/
├── constants/
├── drivers/
├── memory/
└── types/
```

### Memory

Zinc now has its own global heap allocator.

The allocator provides dynamic memory for Rust types such as:

```rust
String
Vec
Box
```

This allows Zinc to move away from fixed-size buffers for data such as shell commands.

The current allocator is a simple bump allocator and is intended for the early development stages of Zinc.

## Default shell - Brass

Brass is Zinc's built-in command shell.

Current commands:

* `echo <text>`
* `help`
* `version`
* `clear`

Example:

```text
+- enesx32[/]
|
+--- $ echo hello
hello
```

## How to install

Download `zinc-os.iso` from the [Releases](https://github.com/enesx32/zinc/releases) page.

### QEMU

```bash
qemu-system-x86_64 -cdrom zinc-os.iso
```

### VirtualBox

1. Click **New**.
2. Set **Type** to `Other`.
3. Set **Version** to `Other/Unknown (64-bit)`.
4. Do not add a virtual hard disk.
5. In **Settings > System**, make sure **Enable EFI** is off.
6. In **Settings > Storage**, put `zinc-os.iso` in the optical drive.
7. Start the VM.

### VMware

1. Create a new VM.
2. Choose **I will install the operating system later**.
3. Pick `Other` and `Other 64-bit`.
4. Set the CD/DVD drive to use `zinc-os.iso`.
5. Keep the firmware set to BIOS, not UEFI.

Zinc currently requires BIOS firmware and x86-64 support.

## Build from source

### Requirements

* Rust 1.85 or newer
* `x86_64-unknown-none` target
* LLVM tools
* NASM
* QEMU
* Python 3

Install the Rust components:

```bash
rustup target add x86_64-unknown-none
rustup component add llvm-tools
cargo install cargo-binutils
```

### Linux

Install the required tools:

```bash
sudo apt install nasm qemu-system-x86 python3
```

Clone Zinc:

```bash
git clone https://github.com/enesx32/zinc
cd zinc
./run.sh
```

### Windows

Use **Git Bash** or **WSL**.

Install NASM and QEMU:

```bash
winget install NASM.NASM
winget install SoftwareFreedomConservancy.QEMU
```

Install Python from [python.org](https://www.python.org/) and make sure NASM, QEMU, and Python are available on your `PATH`.

Then:

```bash
git clone https://github.com/enesx32/zinc
cd zinc
./run.sh
```

`run.sh` builds the kernel, assembles the bootloader, creates `build/zinc-os.iso`, and launches Zinc in QEMU.

## Project layout

```text
zinc/
├── .cargo/
│   └── config.toml
├── .vscode/
├── boot/
│   └── boot.asm
├── build/
├── docs/
│   └── showcase.md
├── images/
│   ├── help_example.png
│   ├── unknown_command_example.png
│   └── version.png
├── kernel/
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── linker.ld
├── shell/
│   ├── src/
│   │   ├── echo.rs
│   │   ├── help.rs
│   │   ├── lib.rs
│   │   └── version.rs
│   └── Cargo.toml
├── target/
├── zcore/
│   ├── constants/
│   │   ├── src/
│   │   └── Cargo.toml
│   ├── drivers/
│   │   ├── src/
│   │   └── Cargo.toml
│   ├── memory/
│   │   ├── src/
│   │   │   ├── alloc.rs
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   └── types/
│       ├── src/
│       │   ├── lib.rs
│       │   └── string.rs
│       └── Cargo.toml
├── .gitignore
├── build.sh
├── Cargo.lock
├── Cargo.toml
├── export-vm.sh
├── isogen.py
├── kernel.asm
├── README.md
├── run.sh
├── rust-toolchain.toml
└── x86_64-zinc.json
```

## How it boots

1. The BIOS loads the Zinc boot image.
2. `boot.asm` enables A20 and loads the GDT.
3. The bootloader enters protected mode.
4. Zinc creates page tables and enables PAE and 64-bit long mode.
5. The kernel is copied to `0x100000`.
6. The bootloader jumps to `_start`.
7. The Rust kernel initialises Zinc's global memory allocator.
8. The Brass shell is started.
9. Keyboard input is read and shell commands are executed.
