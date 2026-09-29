; tell NASM to generate 16-bit code because the CPU starts in real mode
[bits 16]
; tell NASM this code will be loaded at address 0x7c00 so labels get correct addresses
[org 0x7c00]

; address where the BIOS leaves the kernel in memory (right after the 2 KB bootloader)
KERNEL_OFFSET equ 0x8400
; address (1 MB) where the kernel is copied to and where it expects to run
KERNEL_DEST   equ 0x100000
; number of 8-byte chunks to copy, 3840 * 8 = 30 KB
KERNEL_QWORDS equ 3840
; address of the VGA text buffer
VGA_BASE      equ 0xb8000
; VGA attribute for light green text on black
COLOR_GREEN   equ 0x0a
; VGA attribute for white text on black
COLOR_WHITE   equ 0x0f

; entry point of the bootloader, the BIOS jumps here
start:
    ; far jump to set CS to 0
    jmp 0x0000:normalize

; we land here with CS = 0 so all addresses match the org above
normalize:
    ; disable interrupts because we have no interrupt handlers set up
    cli
    ; set ax to 0
    xor ax, ax
    ; set the data segment to 0
    mov ds, ax
    ; set the extra segment to 0
    mov es, ax
    ; set the stack segment to 0
    mov ss, ax
    ; put the stack just below the bootloader so it grows downward into free memory
    mov sp, 0x7c00
    ; clear the direction flag so string instructions (rep stosw/stosd/movsq) count upward
    cld

    ; point the extra segment at the VGA text buffer (segment 0xb800 = address 0xb8000)
    mov ax, 0xb800
    ; load it into es
    mov es, ax
    ; start writing at the top-left cell of the screen
    xor di, di
    ; a blank cell: attribute 0x07 (grey on black) and character 0x20 (space)
    mov ax, 0x0720
    ; the screen has 80 * 25 = 2000 cells
    mov cx, 2000
    ; fill all cells with the blank cell to clear the BIOS text
    rep stosw
    ; set ax back to 0
    xor ax, ax
    ; restore the extra segment to 0
    mov es, ax

    ; point si at the message to print
    mov si, msg_bootloader
    ; print the [ SUCCESS ] line
    call print16

    ; read the control port
    in al, 0x92
    ; set bit 1, which turns on the A20 line so memory above 1 MB is reachable
    or al, 2
    ; clear bit 0 because setting it would reset the whole machine
    and al, 0xfe
    ; write the value back to enable A20
    out 0x92, al

    ; point si at the message to print
    mov si, msg_a20
    ; print the [ SUCCESS ] line
    call print16

    ; load the GDT register with the size and address stored at gdt_descriptor
    lgdt [gdt_descriptor]

    ; point si at the message to print
    mov si, msg_gdt
    ; print the [ SUCCESS ] line
    call print16

    ; read control register 0 into eax
    mov eax, cr0
    ; set bit 0 (PE) to turn on protected mode
    or eax, 1
    ; write it back, the CPU is now in protected mode
    mov cr0, eax

    ; far jump to load the 32-bit code segment and flush the CPU's prefetched 16-bit instructions
    jmp CODE32_SEG:init_pm32

; prints "[ SUCCESS ] " in green followed by the string at ds:si in white (real mode)
print16:
    ; save the registers we are about to change
    push ax
    ; save bx
    push bx
    ; save di
    push di
    ; save es
    push es
    ; point es at the VGA text buffer segment
    mov ax, 0xb800
    ; load it into es
    mov es, ax
    ; di = byte offset of the current line on screen
    mov di, [cursor]
    ; bx points at the "[ SUCCESS ] " tag text
    mov bx, tag_success
    ; use green for the tag
    mov ah, COLOR_GREEN
.tag_loop:
    ; load the next character of the tag
    mov al, [bx]
    ; a zero byte marks the end of the string
    cmp al, 0
    ; the tag is finished, go print the message
    je .tag_done
    ; write character (al) and color (ah) into the screen cell
    mov [es:di], ax
    ; move to the next screen cell (2 bytes per cell)
    add di, 2
    ; move to the next character of the tag
    inc bx
    ; repeat for the next character
    jmp .tag_loop
.tag_done:
    ; use white for the message text
    mov ah, COLOR_WHITE
.msg_loop:
    ; load the next character of the message from ds:si and advance si
    lodsb
    ; a zero byte marks the end of the string
    cmp al, 0
    ; the message is finished
    je .done
    ; write character (al) and color (ah) into the screen cell
    mov [es:di], ax
    ; move to the next screen cell
    add di, 2
    ; repeat for the next character
    jmp .msg_loop
.done:
    ; move the cursor down one line (80 cells * 2 bytes = 160 bytes)
    add word [cursor], 160
    ; restore es
    pop es
    ; restore di
    pop di
    ; restore bx
    pop bx
    ; restore ax
    pop ax
    ; return to the caller
    ret

; from here on NASM generates 32-bit code
[bits 32]
; first code that runs in 32-bit protected mode
init_pm32:
    ; load the 32-bit data segment selector into ax
    mov ax, DATA32_SEG
    ; point the data segment at it
    mov ds, ax
    ; point the extra segment at it
    mov es, ax
    ; point fs at it
    mov fs, ax
    ; point gs at it
    mov gs, ax
    ; point the stack segment at it
    mov ss, ax
    ; set up a 32-bit stack at 0x90000
    mov esp, 0x90000

    ; point esi at the message to print
    mov esi, msg_pm
    ; print the [ SUCCESS ] line
    call print32

    ; build the page tables needed for long mode
    call setup_page_tables

    ; point esi at the message to print
    mov esi, msg_tables
    ; print the [ SUCCESS ] line
    call print32

    ; read control register 4 into eax
    mov eax, cr4
    ; set bit 5 (PAE, required for long mode), bit 9 (OSFXSR) and bit 10 (OSXMMEXCPT) for SSE support
    or eax, (1 << 5) | (1 << 9) | (1 << 10)
    ; write it back to enable those features
    mov cr4, eax

    ; the top-level page table (PML4) is at address 0x1000
    mov eax, 0x1000
    ; tell the CPU where the page tables are by loading cr3
    mov cr3, eax

    ; select the EFER model-specific register (0xC0000080)
    mov ecx, 0xc0000080
    ; read the MSR selected by ecx into edx:eax
    rdmsr
    ; set bit 8 (LME) to allow long mode
    or eax, 1 << 8
    ; write the modified value back to EFER
    wrmsr

    ; read control register 0 again
    mov eax, cr0
    ; set bit 31 (PG) to turn on paging, which activates long mode because LME is set
    or eax, (1 << 31)
    ; write it back, the CPU is now in compatibility mode inside long mode
    mov cr0, eax

    ; point esi at the message to print
    mov esi, msg_paging
    ; print the [ SUCCESS ] line
    call print32

    ; far jump to the 64-bit code segment to enter full 64-bit mode
    jmp CODE64_SEG:init_lm64

; builds a minimal page table that identity-maps the first 4 MB
setup_page_tables:
    ; edi points at the start of the tables at 0x1000
    mov edi, 0x1000
    ; set eax to 0 so we can fill memory with zeros
    xor eax, eax
    ; 3072 dwords = 12 KB, enough for three 4 KB tables
    mov ecx, 3072
    ; write ecx zero dwords starting at edi to clear all three tables
    rep stosd

    ; PML4 entry 0 points to the PDPT at 0x2000 (flags 3 = present + writable)
    mov dword [0x1000], 0x2003
    ; PDPT entry 0 points to the page directory at 0x3000 (present + writable)
    mov dword [0x2000], 0x3003
    ; page directory entry 0 maps physical 0-2 MB (0x83 = present + writable + huge 2 MB page)
    mov dword [0x3000], 0x00000083
    ; page directory entry 1 maps physical 2-4 MB (starts at 0x200000, same flags)
    mov dword [0x3008], 0x00200083
    ; return to the caller
    ret

; prints "[ SUCCESS ] " in green followed by the string at esi in white (32-bit mode)
print32:
    ; save the registers we are about to change
    push eax
    ; save ebx
    push ebx
    ; save edi
    push edi
    ; edi = byte offset of the current line on screen
    movzx edi, word [cursor]
    ; turn the offset into a real address inside the VGA buffer
    add edi, VGA_BASE
    ; ebx points at the "[ SUCCESS ] " tag text
    mov ebx, tag_success
    ; use green for the tag
    mov ah, COLOR_GREEN
.tag_loop:
    ; load the next character of the tag
    mov al, [ebx]
    ; a zero byte marks the end of the string
    cmp al, 0
    ; the tag is finished, go print the message
    je .tag_done
    ; write character (al) and color (ah) into the screen cell
    mov [edi], ax
    ; move to the next screen cell (2 bytes per cell)
    add edi, 2
    ; move to the next character of the tag
    inc ebx
    ; repeat for the next character
    jmp .tag_loop
.tag_done:
    ; use white for the message text
    mov ah, COLOR_WHITE
.msg_loop:
    ; load the next character of the message
    mov al, [esi]
    ; a zero byte marks the end of the string
    cmp al, 0
    ; the message is finished
    je .done
    ; write character (al) and color (ah) into the screen cell
    mov [edi], ax
    ; move to the next screen cell
    add edi, 2
    ; move to the next character of the message
    inc esi
    ; repeat for the next character
    jmp .msg_loop
.done:
    ; move the cursor down one line (80 cells * 2 bytes = 160 bytes)
    add word [cursor], 160
    ; restore edi
    pop edi
    ; restore ebx
    pop ebx
    ; restore eax
    pop eax
    ; return to the caller
    ret

; from here on NASM generates 64-bit code
[bits 64]
; first code that runs in 64-bit long mode
init_lm64:
    ; load the 64-bit data segment selector into ax
    mov ax, DATA64_SEG
    ; point the data segment at it
    mov ds, ax
    ; point the extra segment at it
    mov es, ax
    ; point fs at it
    mov fs, ax
    ; point gs at it
    mov gs, ax
    ; point the stack segment at it
    mov ss, ax

    ; set up the 64-bit stack pointer at 0x90000
    mov rsp, 0x90000

    ; point esi at the message to print (writing esi also clears the top half of rsi)
    mov esi, msg_lm
    ; print the [ SUCCESS ] line
    call print64

    ; source of the copy: where the BIOS loaded the kernel
    mov rsi, KERNEL_OFFSET
    ; destination of the copy: 1 MB
    mov rdi, KERNEL_DEST
    ; number of 8-byte chunks to copy (30 KB)
    mov rcx, KERNEL_QWORDS
    ; copy rcx quadwords from rsi to rdi
    rep movsq

    ; point esi at the message to print
    mov esi, msg_copied
    ; print the [ SUCCESS ] line
    call print64

    ; point esi at the message to print
    mov esi, msg_jump
    ; print the [ SUCCESS ] line
    call print64

    ; pass the byte offset of the next free screen line to the kernel in rdi (its first argument)
    movzx edi, word [cursor]
    ; put the kernel's address in rax
    mov rax, KERNEL_DEST
    ; jump to the kernel's first byte, which is _start
    jmp rax

; prints "[ SUCCESS ] " in green followed by the string at rsi in white (64-bit mode)
print64:
    ; save the registers we are about to change
    push rax
    ; save rbx
    push rbx
    ; save rdi
    push rdi
    ; edi = byte offset of the current line on screen (also clears the top half of rdi)
    movzx edi, word [cursor]
    ; turn the offset into a real address inside the VGA buffer
    add edi, VGA_BASE
    ; ebx points at the "[ SUCCESS ] " tag text
    mov ebx, tag_success
    ; use green for the tag
    mov ah, COLOR_GREEN
.tag_loop:
    ; load the next character of the tag
    mov al, [rbx]
    ; a zero byte marks the end of the string
    cmp al, 0
    ; the tag is finished, go print the message
    je .tag_done
    ; write character (al) and color (ah) into the screen cell
    mov [rdi], ax
    ; move to the next screen cell (2 bytes per cell)
    add rdi, 2
    ; move to the next character of the tag
    inc rbx
    ; repeat for the next character
    jmp .tag_loop
.tag_done:
    ; use white for the message text
    mov ah, COLOR_WHITE
.msg_loop:
    ; load the next character of the message
    mov al, [rsi]
    ; a zero byte marks the end of the string
    cmp al, 0
    ; the message is finished
    je .done
    ; write character (al) and color (ah) into the screen cell
    mov [rdi], ax
    ; move to the next screen cell
    add rdi, 2
    ; move to the next character of the message
    inc rsi
    ; repeat for the next character
    jmp .msg_loop
.done:
    ; move the cursor down one line (80 cells * 2 bytes = 160 bytes)
    add word [cursor], 160
    ; restore rdi
    pop rdi
    ; restore rbx
    pop rbx
    ; restore rax
    pop rax
    ; return to the caller
    ret

; align the GDT to a 16-byte boundary
align 16
; start of the Global Descriptor Table
gdt_start:
    ; descriptor 0 must be a null descriptor (all zeros)
    dq 0x0000000000000000
    ; 32-bit code segment: limit 0xffff, base 0, access 0x9a (present, ring 0, executable, readable), flags 0xcf (4 KB granularity, 32-bit, limit high bits 0xf)
    dw 0xffff, 0x0000, 0x9a00, 0x00cf
    ; 32-bit data segment: same layout but access 0x92 (present, ring 0, writable data)
    dw 0xffff, 0x0000, 0x9200, 0x00cf
    ; 64-bit code segment: access 0x9a, flags 0x20 sets the L bit (long mode), limit and base are ignored
    dw 0x0000, 0x0000, 0x9a00, 0x0020
    ; 64-bit data segment: access 0x92 (writable data), the rest is ignored in long mode
    dw 0x0000, 0x0000, 0x9200, 0x0000
; end of the GDT, used to calculate its size
gdt_end:

; the structure that lgdt loads
gdt_descriptor:
    ; size of the GDT in bytes minus one
    dw gdt_end - gdt_start - 1
    ; address of the start of the GDT
    dd gdt_start

; selector for the 32-bit code segment (descriptor 1, offset 8)
CODE32_SEG equ 0x08
; selector for the 32-bit data segment (descriptor 2, offset 16)
DATA32_SEG equ 0x10
; selector for the 64-bit code segment (descriptor 3, offset 24)
CODE64_SEG equ 0x18
; selector for the 64-bit data segment (descriptor 4, offset 32)
DATA64_SEG equ 0x20

; byte offset of the next free screen line, shared by all three print routines
cursor dw 0

; the green tag printed before every message (ends with a zero byte)
tag_success db "[ SUCCESS ] ", 0
; message printed by the bootloader as soon as it starts
msg_bootloader db "Bootloader loaded", 0
; message printed after A20 is enabled
msg_a20 db "A20 line enabled", 0
; message printed after the GDT is loaded
msg_gdt db "GDT loaded", 0
; message printed once the CPU is in 32-bit protected mode
msg_pm db "Entered protected mode (32-bit)", 0
; message printed after the page tables are built
msg_tables db "Page tables built", 0
; message printed after paging and long mode are switched on
msg_paging db "Paging and long mode enabled", 0
; message printed once the CPU runs 64-bit code
msg_lm db "Entered long mode (64-bit)", 0
; message printed after the kernel is copied to 1 MB
msg_copied db "Kernel copied to 1 MB", 0
; message printed right before jumping into the kernel
msg_jump db "Jumping to kernel", 0

; pad the bootloader with zeros up to exactly 2048 bytes so the kernel starts at 0x8400
times 2048-($-$$) db 0