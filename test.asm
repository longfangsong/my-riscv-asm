addi t0, zero, 0x1
addi t1, zero, 0x1
loop:
    add t3, t0, t1
    add t0, t1, zero
    add t1, t3, zero
    rdtime t5
    sleep:
        rdtime t6
        sub t3, t6, t5
        addi t6, zero, 0x10
        blt t3, t6, sleep
    jal zero, loop