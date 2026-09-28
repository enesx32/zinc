[bits 16]
[org 0x7c00]

KERNEL_OFFSET equ 0x7e00
KERNEL_DEST   equ 0x100000

start:
    jmp 0x0000:normalize

normalize:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7c00
    cld

    in al, 0x92
    or al, 2
    and al, 0xfe
    out 0x92, al

    lgdt [gdt_descriptor]

    mov eax, cr0
    or eax, 1
    mov cr0, eax

    jmp CODE32_SEG:init_pm32

[bits 32]
init_pm32:
    mov ax, DATA32_SEG
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    mov esp, 0x90000

    call setup_page_tables

    mov eax, cr4
    or eax, (1 << 5) | (1 << 9) | (1 << 10)
    mov cr4, eax

    mov eax, 0x1000
    mov cr3, eax

    mov ecx, 0xc0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    mov eax, cr0
    or eax, (1 << 31)
    mov cr0, eax

    jmp CODE64_SEG:init_lm64

setup_page_tables:
    mov edi, 0x1000
    xor eax, eax
    mov ecx, 3072
    rep stosd

    mov dword [0x1000], 0x2003
    mov dword [0x2000], 0x3003
    mov dword [0x3000], 0x00000083
    mov dword [0x3008], 0x00200083
    ret

[bits 64]
init_lm64:
    mov ax, DATA64_SEG
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    mov rsp, 0x90000

    mov rsi, KERNEL_OFFSET
    mov rdi, KERNEL_DEST
    mov rcx, 4096
    rep movsq

    mov rax, KERNEL_DEST
    jmp rax

align 16
gdt_start:
    dq 0x0000000000000000
    dw 0xffff, 0x0000, 0x9a00, 0x00cf
    dw 0xffff, 0x0000, 0x9200, 0x00cf
    dw 0x0000, 0x0000, 0x9a00, 0x0020
    dw 0x0000, 0x0000, 0x9200, 0x0000
gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

CODE32_SEG equ 0x08
DATA32_SEG equ 0x10
CODE64_SEG equ 0x18
DATA64_SEG equ 0x20

times 510-($-$$) db 0
dw 0xaa55