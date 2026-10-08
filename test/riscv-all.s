label:
    lui x1, 0x12345
    auipc x1, 0x12345
    jal x1, -12
    jalr x1, -12(x2)

    beq x1, x2, -12
    bne x1, x2, -12
    blt x1, x2, -12
    bge x1, x2, -12
    bltu x1, x2, -12
    bgeu x1, x2, -12
    