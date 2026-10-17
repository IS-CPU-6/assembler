_start:
    mv x2, x1
    li x1, 0x06543219
    la x1, _start
    la x1, exit

    j exit
    beqz x1, exit
    bnez x1, exit
    bltz x1, exit
    j _start
    beqz x1, _start
    bnez x1, _start
    bltz x1, _start

    call exit
    call _start
    ret

    seqz x1, x2
    snez x1, x2

    fneg.s f1, f2
    fmv.s f1, f2

exit: