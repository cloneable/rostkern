ENTRY(_boot)

SECTIONS {
    . = ALIGN(0x1000);

    .bootloader :
    {
        *(.bootsector)
    }
}
