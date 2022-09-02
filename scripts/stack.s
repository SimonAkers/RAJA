# Example of writing to the stack
addi $sp, $sp, -4

li $t0, 0x42
sw $t0, 0($sp)

# Exit
li $v0, 10
syscall
