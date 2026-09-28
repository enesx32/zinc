ENTRY(_start)

PHDRS
{
    kernel PT_LOAD FLAGS(7);
}

SECTIONS
{
    . = 0x100000;

    .text : ALIGN(16)
    {
        KEEP(*(.text._start))
        *(.text .text.*)
    } :kernel

    .rodata : ALIGN(16)
    {
        *(.rodata .rodata.*)
        *(.data.rel.ro .data.rel.ro.*)
    } :kernel

    .data : ALIGN(16)
    {
        *(.data .data.*)
        *(.got .got.*)
    } :kernel

    .bss : ALIGN(16)
    {
        *(.bss .bss.*)
        *(COMMON)
    } :kernel

    /DISCARD/ :
    {
        *(.eh_frame*)
        *(.comment)
    }
}