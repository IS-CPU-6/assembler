SECTIONS
{
  . = 0x80000000;

  .text : {
    *(.text)
  }

  tohost = 0x20000000;
  fromhost = 0x20000004;
}