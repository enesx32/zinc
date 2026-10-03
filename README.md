# **zinc-os**
![Version](https://img.shields.io/badge/version-v3.3.0-blue)

A tiny hobby operating system written in Rust and x86-64 assembly. Zinc boots from a custom BIOS bootloader, switches the CPU into 64-bit long mode, and runs a Rust kernel

## Features
- Custom bootloader
- Rust kernel
- VGA text mode output
- Bootable ISO built by a small Python script, so no `xorriso` or `mkisofs` is needed

## Native API - Zcore
The zcore library provides drivers like the VGA buffer and Keyboard drivers<br/>
it also gives types for `String` and `Basic String`<br/>
lastly it has a `zcore/util` folder for general developing stuff

## How to install
Download `zinc-os.iso` from the [Releases](https://github.com/enesx32/zinc/releases) page.

### QEMU
```bash
qemu-system-x86_64 -cdrom zinc-os.iso
```
### QEMU + Python
```bash
bash run.sh
```

### VirtualBox
1. Click **New**, set **Type** to `Other` and **Version** to `Other/Unknown (64-bit)`.
2. Do not add a virtual hard disk.
3. In **Settings > System**, make sure **Enable EFI** is off.
4. In **Settings > Storage**, put `zinc-os.iso` in the optical drive.
5. Start the VM.

### VMware
1. Create a new VM and choose **I will install the operating system later**.
2. Pick `Other` and `Other 64-bit`.
3. Set the CD/DVD drive to use `zinc-os.iso` and keep the firmware on BIOS, not UEFI.

Zinc runs in 64-bit mode, so hardware virtualization (VT-x or AMD-V) must be enabled in your PC's BIOS.

## Build from source

### Requirements
- Rust (1.85 or newer) with the bare-metal target and LLVM tools
- NASM
- QEMU
- Python 3

```bash
rustup target add x86_64-unknown-none
rustup component add llvm-tools
cargo install cargo-binutils
```

### Linux
```bash
sudo apt install nasm qemu-system-x86 python3
git clone https://github.com/enesx32/zinc
cd zinc
./run.sh
```

### Windows
Use `Git Bash` or `WSL`. For Git Bash, install the tools first:

```bash
winget install NASM.NASM
winget install SoftwareFreedomConservancy.QEMU
```

Install Python from python.org and make sure NASM, QEMU and Python are on your `PATH`. Then:

```bash
git clone https://github.com/enesx32/zinc
cd zinc
./run.sh
```

`run.sh` builds the kernel, assembles the bootloader, creates `build/zinc-os.iso`, and launches it in QEMU.

## Project layout
```
zinc/
├── .cargo/
│   └── config.toml
├── .vscode/
├── boot/
│   └── boot.asm
├── build/
├── kernel/
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── linker.ld
├── target/
├── zcore/
│   ├── drivers/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── vga_buffer.rs
│   │   │   └── keyboard.rs
│   │   └── Cargo.toml
│   ├── constants/
│   │   ├── src/
│   │   │   ├── colors.rs
│   │   │   ├── lib.rs
│   │   │   └── keys.rs
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
1. The BIOS loads the boot image from the CD to `0x7C00` and jumps to it.
2. `boot.asm` enables A20, loads a GDT, and enters protected mode.
3. It builds page tables, enables PAE and long mode, and jumps to 64-bit code.
4. The kernel is copied to `0x100000` and started at `_start`.
5. The kernel writes `hello from kernel` to VGA memory at `0xB8000`.

## Limitations
- BIOS boot only (no UEFI)
- The kernel can be at most 32 KB. To go bigger, raise the size in `run.sh` and the copy size in `boot.asm`.
- The bootloader only boots from an ISO, not a raw disk image