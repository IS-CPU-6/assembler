SECTIONS
{
  . = 0x80000000;

  .text : {
    *(.text)
  }

  tohost = 0x700000f8;
  fromhost = 0x700000fc;
  _start = 0x80000000;
}