    .data
prompt:     .asciiz "Enter float: "
result:     .asciiz "Result: "
nl:         .asciiz "\n"

    .text
# Print prompt
la $a0, prompt
li $v0, 4
syscall

# Read float
li $v0, 6
syscall

# Move input to another register
move $f12, $f0

# Print result message prefix
la $a0, result
li $v0, 4
syscall

# Print float
li $v0, 2
syscall

# Exit
li $v0, 10
syscall