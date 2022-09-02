# Example of several error messages

.data
.space 1
FOO:
.word 0x40

.text
la $t0, FOO
lw $t0, 0($t0)
lw $t0, 0($t0)

# Add 5 to contents of $t0
addi $t0, 5

li $v0, 10
syscall
