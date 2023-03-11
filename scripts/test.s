.data
msg: 
.asciiz "Hello There\n"

msg2:
.asciiz "Hello Again\n"

.text
la $a0, msg
li $v0, 4
syscall

la $a0, msg2
syscall

li $v0, 10
syscall