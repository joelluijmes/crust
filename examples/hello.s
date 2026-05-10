.global _main
.align 2

_main:
; fd 1 = stdout
mov x0, 1
; x1: address of the string
adrp x1, label_1@PAGE
add x1, x1, label_1@PAGEOFF
; x2: length of the string
mov x2, 8
; x16: 4 = syscall write
mov x16, 4
svc 0x80

; syscall exit with code in x0
mov x0, 42
mov x16, 1
svc 0x80

label_1: .ascii "Hello!"