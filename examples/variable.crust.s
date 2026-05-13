.global _main
.align 2

_main:
sub sp, sp, #48
stp x29, x30, [sp, #32]
mov x29, sp

; x = 42
mov w8, #42
str w8, [sp, #36]

; y = x
ldr w8, [sp, #36]
str w8, [sp, #32]

; z = x
ldr w8, [sp, #36]
str w8, [sp, #28]

; a = 1
mov w8, #1
str w8, [sp, #24]

; b = 2
mov w8, #2
str w8, [sp, #20]

; c = 3
mov w8, #3
str w8, [sp, #16]

ldp x29, x30, [sp, #32]
add sp, sp, #48

; return 0
mov w0, #0
ret

