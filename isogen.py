import sys

SECTOR = 2048
PVD_LBA = 16
BOOT_RECORD_LBA = 17
TERMINATOR_LBA = 18
L_PATH_LBA = 19
M_PATH_LBA = 20
ROOT_LBA = 21
CATALOG_LBA = 22
IMAGE_LBA = 23

def put_le16(buf, off, v):
    buf[off] = v & 0xFF
    buf[off + 1] = (v >> 8) & 0xFF

def put_be16(buf, off, v):
    buf[off] = (v >> 8) & 0xFF
    buf[off + 1] = v & 0xFF

def put_le32(buf, off, v):
    buf[off] = v & 0xFF
    buf[off + 1] = (v >> 8) & 0xFF
    buf[off + 2] = (v >> 16) & 0xFF
    buf[off + 3] = (v >> 24) & 0xFF

def put_be32(buf, off, v):
    buf[off] = (v >> 24) & 0xFF
    buf[off + 1] = (v >> 16) & 0xFF
    buf[off + 2] = (v >> 8) & 0xFF
    buf[off + 3] = v & 0xFF

def put_both16(buf, off, v):
    put_le16(buf, off, v)
    put_be16(buf, off + 2, v)

def put_both32(buf, off, v):
    put_le32(buf, off, v)
    put_be32(buf, off + 4, v)

def put_text(buf, off, text, length, pad):
    i = 0
    while i < length:
        if i < len(text):
            buf[off + i] = ord(text[i])
        else:
            buf[off + i] = pad
        i += 1

def put_bytes(buf, off, data):
    i = 0
    while i < len(data):
        buf[off + i] = data[i]
        i += 1

def dir_record(lba, size, flags, ident):
    id_len = len(ident)
    rec_len = 33 + id_len
    if id_len % 2 == 0:
        rec_len += 1
    rec = bytearray(rec_len)
    rec[0] = rec_len
    rec[1] = 0
    put_both32(rec, 2, lba)
    put_both32(rec, 10, size)
    rec[18] = 126
    rec[19] = 9
    rec[20] = 28
    rec[21] = 0
    rec[22] = 0
    rec[23] = 0
    rec[24] = 0
    rec[25] = flags
    rec[26] = 0
    rec[27] = 0
    put_both16(rec, 28, 1)
    rec[32] = id_len
    put_bytes(rec, 33, ident)
    return rec

def build_pvd(total_sectors, path_table_size):
    pvd = bytearray(SECTOR)
    pvd[0] = 1
    put_text(pvd, 1, "CD001", 5, 0x20)
    pvd[6] = 1
    pvd[7] = 0
    put_text(pvd, 8, "", 32, 0x20)
    put_text(pvd, 40, "ZINC_OS", 32, 0x20)
    put_both32(pvd, 80, total_sectors)
    put_both16(pvd, 120, 1)
    put_both16(pvd, 124, 1)
    put_both16(pvd, 128, SECTOR)
    put_both32(pvd, 132, path_table_size)
    put_le32(pvd, 140, L_PATH_LBA)
    put_le32(pvd, 144, 0)
    put_be32(pvd, 148, M_PATH_LBA)
    put_be32(pvd, 152, 0)
    root = dir_record(ROOT_LBA, SECTOR, 2, bytearray([0]))
    put_bytes(pvd, 156, root)
    put_text(pvd, 190, "", 128, 0x20)
    put_text(pvd, 318, "", 128, 0x20)
    put_text(pvd, 446, "", 128, 0x20)
    put_text(pvd, 574, "ZINC OS", 128, 0x20)
    put_text(pvd, 702, "", 37, 0x20)
    put_text(pvd, 739, "", 37, 0x20)
    put_text(pvd, 776, "", 37, 0x20)
    put_text(pvd, 813, "2026092800000000", 16, 0x30)
    pvd[829] = 0
    put_text(pvd, 830, "2026092800000000", 16, 0x30)
    pvd[846] = 0
    put_text(pvd, 847, "0000000000000000", 16, 0x30)
    pvd[863] = 0
    put_text(pvd, 864, "2026092800000000", 16, 0x30)
    pvd[880] = 0
    pvd[881] = 1
    return pvd

def build_boot_record():
    vd = bytearray(SECTOR)
    vd[0] = 0
    put_text(vd, 1, "CD001", 5, 0x20)
    vd[6] = 1
    put_text(vd, 7, "EL TORITO SPECIFICATION", 32, 0)
    put_le32(vd, 71, CATALOG_LBA)
    return vd

def build_terminator():
    vd = bytearray(SECTOR)
    vd[0] = 255
    put_text(vd, 1, "CD001", 5, 0x20)
    vd[6] = 1
    return vd

def build_path_table(little):
    table = bytearray(SECTOR)
    table[0] = 1
    table[1] = 0
    if little:
        put_le32(table, 2, ROOT_LBA)
        put_le16(table, 6, 1)
    else:
        put_be32(table, 2, ROOT_LBA)
        put_be16(table, 6, 1)
    table[8] = 0
    table[9] = 0
    return table

def build_root(image_size):
    root = bytearray(SECTOR)
    pos = 0
    entry = dir_record(ROOT_LBA, SECTOR, 2, bytearray([0]))
    put_bytes(root, pos, entry)
    pos += len(entry)
    entry = dir_record(ROOT_LBA, SECTOR, 2, bytearray([1]))
    put_bytes(root, pos, entry)
    pos += len(entry)
    name = bytearray(b"ZINC_OS.IMG;1")
    entry = dir_record(IMAGE_LBA, image_size, 0, name)
    put_bytes(root, pos, entry)
    return root

def build_catalog(image_size):
    sectors512 = (image_size + 511) // 512
    cat = bytearray(SECTOR)
    cat[0] = 1
    cat[1] = 0
    cat[30] = 0x55
    cat[31] = 0xAA
    total = 0
    i = 0
    while i < 32:
        total += cat[i] | (cat[i + 1] << 8)
        i += 2
    checksum = (0x10000 - (total & 0xFFFF)) & 0xFFFF
    put_le16(cat, 28, checksum)
    cat[32] = 0x88
    cat[33] = 0
    put_le16(cat, 34, 0)
    cat[36] = 0
    cat[37] = 0
    put_le16(cat, 38, sectors512)
    put_le32(cat, 40, IMAGE_LBA)
    return cat

@(lambda f: f())
def main():
    if len(sys.argv) != 3:
        print("usage: isogen.py <boot-image> <output.iso>")
        sys.exit(1)
    src = open(sys.argv[1], "rb")
    image = src.read()
    src.close()
    image_size = len(image)
    image_sectors = (image_size + SECTOR - 1) // SECTOR
    total_sectors = IMAGE_LBA + image_sectors
    iso = bytearray(total_sectors * SECTOR)
    put_bytes(iso, PVD_LBA * SECTOR, build_pvd(total_sectors, 10))
    put_bytes(iso, BOOT_RECORD_LBA * SECTOR, build_boot_record())
    put_bytes(iso, TERMINATOR_LBA * SECTOR, build_terminator())
    put_bytes(iso, L_PATH_LBA * SECTOR, build_path_table(True))
    put_bytes(iso, M_PATH_LBA * SECTOR, build_path_table(False))
    put_bytes(iso, ROOT_LBA * SECTOR, build_root(image_size))
    put_bytes(iso, CATALOG_LBA * SECTOR, build_catalog(image_size))
    put_bytes(iso, IMAGE_LBA * SECTOR, image)
    out = open(sys.argv[2], "wb")
    out.write(iso)
    out.close()
    print("wrote " + sys.argv[2] + " (" + str(total_sectors) + " sectors, boot image " + str(image_size) + " bytes)")