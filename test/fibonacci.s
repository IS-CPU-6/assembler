_start:
    addi x5, x0, 10
    
    addi x6, x0, 0
    addi x7, x0, 1
    
    bge  x0, x5, return_t1
    addi x28, x0, 1

loop:
    bge  x28, x5, done
    
    add  x29, x6, x7
    addi x6, x7, 0
    addi x7, x29, 0
    
    addi x28, x28, 1
    jal  x0, loop

done:
    addi x10, x7, 0
    jal  x0, exit

return_t1:
    addi x10, x6, 0

exit:
