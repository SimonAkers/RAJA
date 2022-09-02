# Write some data to memory
.data
.word 0xAABBCCDD

.text
li $v0, 10
syscall
