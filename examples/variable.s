.global _main
.align 2

_main:
sub sp, sp, #16

; x = 42
mov w8, #42
str w8, [sp, #12]

; y = 69
mov w8, #69
str w8, [sp, #8]

; z = 1000
mov w8, #1000
str w8, [sp, #4]

; syscall exit with code in x0
mov x0, #0
mov x16, #1
svc 0x80

