nop         addi x0, x0, 0
mv          addi {{params[0]}}, {{params[1]}}, 0
not         xori {{params[0]}}, {{params[1]}}, -1
neg         sub {{params[0]}}, x0, {{params[1]}}
seqz        sltiu {{params[0]}}, {{params[1]}}, 1
snez        sltu {{params[0]}}, x0, {{params[1]}}
sltz        slt {{params[0]}}, {{params[1]}}, x0
sgtz        slt {{params[0]}}, x0, {{params[1]}}
beqz        beq {{params[0]}}, x0, {{params[1]}}
bnez        bne {{params[0]}}, x0, {{params[1]}}
blez        bge x0, {{params[0]}}, {{params[1]}}
bgez        bge {{params[0]}}, x0, {{params[1]}}
bltz        blt {{params[0]}}, x0, {{params[1]}}
bgtz        blt x0, {{params[0]}}, {{params[1]}}
bgt         blt {{params[1]}}, {{params[0]}}, {{params[2]}}
ble         bge {{params[1]}}, {{params[0]}}, {{params[2]}}
bgtu        bltu {{params[1]}}, {{params[0]}}, {{params[2]}}
bleu        bgeu {{params[1]}}, {{params[0]}}, {{params[2]}}
j           jal x0, {{params[0]}}
ret         jalr x0, 0(x1)
rdcycle     csrrs {{params[0]}}, cycle, x0
rdcycleh    csrrs {{params[0]}}, cycleh, x0
rdtime      csrrs {{params[0]}}, time, x0
rdtimeh     csrrs {{params[0]}}, timeh, x0
rdinstret   csrrs {{params[0]}}, instret, x0
rdinstreth  csrrs {{params[0]}}, instreth, x0
csrr        csrrs {{params[0]}}, {{params[1]}}, x0
csrw        csrrw x0, {{params[0]}}, {{params[1]}}
csrs        csrrs x0, {{params[0]}}, {{params[1]}}
csrc        csrrc x0, {{params[0]}}, {{params[1]}}
csrwi       csrrwi x0, {{params[0]}}, {{params[1]}}
csrsi       csrrsi x0, {{params[0]}}, {{params[1]}}
csrci       csrrci x0, {{params[0]}}, {{params[1]}}
