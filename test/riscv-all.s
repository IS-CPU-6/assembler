label:
    lui x1, 0x12345
    auipc x1, 0x12345
    jal x1, label
    jalr x1, -12(x2)

    beq x1, x2, label
    bne x1, x2, label
    blt x1, x2, label
    bge x1, x2, label
    bltu x1, x2, label
    bgeu x1, x2, label
    
    lb x1, 4(x2)
    lh x1, 4(x2)
    lw x1, 4(x2)
    lbu x1, 4(x2)
    lhu x1, 4(x2)
    sb x1, 4(x2)
    sh x1, 4(x2)
    sw x1, 4(x2)

    addi x1, x2, -10
    slti x1, x2, -10
    sltiu x1, x2, -10
    xori x1, x2, 0xFF
    ori x1, x2, -10
    andi x1, x2, -10
    slli x1, x2, 4
    srli x1, x2, 4
    srai x1, x2, 4

    add x1, x2, x3
    sub x1, x2, x3
    sll x1, x2, x3
    slt x1, x2, x3
    sltu x1, x2, x3
    xor x1, x2, x3
    srl x1, x2, x3
    sra x1, x2, x3
    or x1, x2, x3
    and x1, x2, x3

    ebreak

    mul x1, x2, x3
    mulh x1, x2, x3
    mulhsu x1, x2, x3
    div x1, x2, x3
    divu x1, x2, x3
    rem x1, x2, x3
    remu x1, x2, x3

    flw f1, -12(x2)
    fsw f1, -12(x2)

    fadd.s f1, f2, f3