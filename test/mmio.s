_start:
    lui x1, 0x70000
    sw x0, 0(x1)
    sw x0, 0xfc(x1)
exit: