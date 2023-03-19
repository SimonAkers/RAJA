    .data
prompt:     .asciiz "Enter integer: "
result:     .asciiz "Result: "
nl:         .asciiz "\n"

    .text
# Print prompt
la $a0, prompt
li $v0, 4
syscall

# Read integer
li $v0, 5
syscall

# Move input to another register
move $t0, $v0

# Print newline
la $a0, nl
li $v0, 4
syscall

# Print result message prefix
la $a0, result
syscall

# Print integer
li $v0, 1
move $a0, $t0
syscall

# Exit
li $v0, 10
syscall