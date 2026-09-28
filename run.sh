#!/usr/bin/env bash
set -e

./build.sh

nasm -f bin boot/boot.asm -o build/boot.bin

cp build/kernel.bin build/kernel_padded.bin
truncate -s 32k build/kernel_padded.bin

cat build/boot.bin build/kernel_padded.bin > build/os.img
truncate -s 512k build/os.img

qemu-system-x86_64 -drive format=raw,file=build/os.img -no-reboot