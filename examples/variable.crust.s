.global _main
.align 2

_main:
sub sp, sp, #32
stp x29, x30, [sp, #16]
add x29, sp, #16

; a = 42
mov w8, #42
str w8, [sp, #12]

; b = a
ldr w8, [sp, #12]
str w8, [sp, #8]

; c = 10
mov w8, #10
str w8, [sp, #4]

ldp x29, x30, [sp, #16]
add sp, sp, #32

; return 0
mov w0, #0
ret

