_start:
    addi x5, x0, 10

    mv   x6, x0
    addi x7, x0, 1

    bge  x0, x5, return_t1
    addi x28, x0, 1

loop:
    bge  x28, x5, done

    add  x29, x6, x7
    mv   x6, x7
    mv   x7, x29

    addi x28, x28, 1
    j    loop

done:
    mv   x10, x7
    j    exit

return_t1:
    mv   x10, x6

exit: