MEMORY
{
  FLASH     : ORIGIN = 0x01000000, LENGTH = 256K
  RAM       : ORIGIN = 0x21000000, LENGTH = 64K
  SHARED_RAM: ORIGIN = 0x20070000, LENGTH = 64K
}

_shared_ram_start = ORIGIN(SHARED_RAM);
_shared_ram_end = _shared_ram_start + LENGTH(SHARED_RAM);

SECTIONS
{
  .shared_ram (NOLOAD) :
  {
    KEEP(*(.shared_ram .shared_ram.*));
  } > SHARED_RAM
}
