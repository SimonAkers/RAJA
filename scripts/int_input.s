.data
prompt:
.asciiz "Enter integer: "

.text
la $a0, prompt
li $v0, 4
syscall

li $v0, 5
syscall

li $v0, 10
syscall