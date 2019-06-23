addi x2, x0, 0x5
addi x1, x0, 0x7
add x3, x2, x1
add x3, x3, x3
add x4, x3, x0
lui x5, 0x10000
sw x4, 0x0(x5)
lb x1, 0x0(x5)
add x1, x1, x1
addi x2, x0, 0xc0
bge x1, x2, 0x8
jal x0, -0xc
addi x2, x0, 0x1
sub x1, x1, x2
add x1, x1, x0