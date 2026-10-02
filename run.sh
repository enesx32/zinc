#!/usr/bin/env bash
set -e

./build.sh

nasm -f bin boot/boot.asm -o build/boot.bin

cp build/kernel.bin build/kernel_padded.bin
truncate -s 64k build/kernel_padded.bin

cat build/boot.bin build/kernel_padded.bin > build/boot_image.bin

ls -l build/kernel.bin build/kernel_padded.bin build/boot_image.bin

if command -v python >/dev/null 2>&1; then
    PY=python
else
    PY=python3
fi

$PY isogen.py build/boot_image.bin build/zinc-os.iso

qemu-system-x86_64 -cdrom build/zinc-os.iso -boot d -no-reboot -d in_asm,int,cpu_reset -D build/qemu.log