.global _main
.align 2

_main:
sub sp, sp, #32

; x = 42
mov w8, #42
str w8, [sp, #28]

; y = x
ldr w8, [sp, #28]
str w8, [sp, #24]

; z = x
ldr w8, [sp, #28]
str w8, [sp, #20]

; a = 1
mov w8, #1
str w8, [sp, #16]

; b = 2
mov w8, #2
str w8, [sp, #12]

; c = 3
mov w8, #3
str w8, [sp, #8]

; syscall exit with code in x0
mov x0, #0
mov x16, #1
svc 0x80

