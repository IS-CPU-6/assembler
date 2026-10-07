_start:
    li   t0, 10
    
    li   t1, 0
    li   t2, 1
    
    blez t0, return_t1
    li   t3, 1

loop:
    bge  t3, t0, done
    
    add  t4, t1, t2
    mv   t1, t2
    mv   t2, t4
    
    addi t3, t3, 1
    j    loop

done:
    mv   a0, t2
    j    exit

return_t1:
    mv   a0, t1

exit:
    li   a7, 10
    ecall