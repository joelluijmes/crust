.global _main
.align 2

_main:
sub sp, sp, #16
stp x29, x30, [sp, #0]
add x29, sp, #0

; printf(...)
adrp x0, label_1@PAGE
add x0, x0, label_1@PAGEOFF
bl _printf

ldp x29, x30, [sp, #0]
add sp, sp, #16

; return 42
mov w0, #42
ret

.section __TEXT,__cstring,cstring_literals
label_1: .asciz "Hello!"
