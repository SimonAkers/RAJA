.data
label0:     .asciiz "Expression: "
label1:     .asciiz "Quotient: "
label2:     .asciiz "Remainder: "
fs:         .asciiz " / "
nl:         .asciiz "\n"

.text

li $t0, 22      # Dividend
li $t1, 3       # Divisor

div $t0, $t1



# Print label
li $v0, 4
la $a0, label0
syscall

# Print dividend
li $v0, 1
move $a0, $t0
syscall

# Print forward slash (division symbol)
li $v0, 4
la $a0, fs
syscall

# Print divisor
li $v0, 1
move $a0, $t1
syscall

# Print newline
li $v0, 4
la $a0, nl
syscall



# Print label
li $v0, 4
la $a0, label1
syscall

# Print quotient
li $v0, 1
mflo $a0
syscall

# Print newline
li $v0, 4
la $a0, nl
syscall



# Print label
li $v0, 4
la $a0, label2
syscall

# Print remainder
li $v0, 1
mfhi $a0
syscall

# Print newline
li $v0, 4
la $a0, nl
syscall



# Exit
li $v0, 10
syscall