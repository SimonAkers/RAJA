li $t0, 22
li $t1, 3

div $t0, $t1

mflo $t2
mfhi $t3

# Print quotient
li $v0, 1
move $a0, $t2
syscall

# Print remainder
move $a0, $t3
syscall


li $v0, 10
syscall