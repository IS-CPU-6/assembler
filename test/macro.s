_start:
    mv x2, x1
    li x1, 0x06543219
    la x1, end

    j end
    beqz x1, end
    bnez x1, end
    bltz x1, end

    call end
    ret

    seqz x1, x2
    snez x1, x2

    fneg.s f1, f2
    fmv.s f1, f2

exit: