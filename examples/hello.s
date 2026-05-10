.global _main
.align 2

_main:
	; syscall exit with code in x0
	mov x0, 42
	mov x16, 1
	svc 0x80
