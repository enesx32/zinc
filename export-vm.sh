#!/usr/bin/env bash
set -e

if [ ! -f build/zinc-os.img ]; then
    echo "build/zinc-os.img not found, run ./run.sh first"
    exit 1
fi

qemu-img convert -f raw -O vdi build/zinc-os.img build/zinc-os.vdi
qemu-img convert -f raw -O vmdk build/zinc-os.img build/zinc-os.vmdk

echo "Created build/zinc-os.vdi (VirtualBox) and build/zinc-os.vmdk (VMware)"