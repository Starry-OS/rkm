SECTIONS
{
    . = 0x0;

    .modinfo : {
        *(.modinfo)
    }

    .text : {
        *(.text .text.*)
        *(.ltext .ltext.*)
        KEEP(*(.text.init))
        KEEP(*(.text.exit))
    }

    .rodata : {
        *(.rodata .rodata.*)
        *(.lrodata .lrodata.*)
        . = ALIGN(8);
    }

    .data : {
        *(.data .data.*)
    }

    .bss : {
        *(.bss .bss.*)
        *(COMMON)
    }
    
    .plt : { BYTE(0) }
	.got : { BYTE(0) }
	.got.plt : { BYTE(0) }

    /DISCARD/ : {
        *(.comment)
        *(.note.GNU-stack)
        *(.gnu.version)
        *(.eh_frame)
        *(.eh_frame_hdr)
    }
}