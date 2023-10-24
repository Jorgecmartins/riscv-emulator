MEMORY
{
    FLASH (rwx) : ORIGIN = 0x40000, LENGTH = 16K
}

SECTIONS
{
    .handler_table : {
        *(.handler_table);
    } > FLASH

    .text : {
        *(.test);
    } > FLASH

    .data : {
        *(.data);
        *(.sdata);
        . = ALIGN(4); 
    } > FLASH

    .bss : {
        *(.bss);
        *(.sbss);
        . = ALIGN(4);
        *(.keep_me);
    } > FLASH

}