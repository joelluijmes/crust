	.build_version macos, 26, 0	sdk_version 26, 2
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_main                           ; -- Begin function main
	.p2align	2
_main:                                  ; @main
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	mov	w0, #0                          ; =0x0
	str	wzr, [sp, #28]
	mov	w8, #42                         ; =0x2a
	str	w8, [sp, #24]
	ldr	w8, [sp, #24]
	str	w8, [sp, #20]
	ldr	w8, [sp, #24]
	str	w8, [sp, #16]
	mov	w8, #1                          ; =0x1
	str	w8, [sp, #12]
	mov	w8, #2                          ; =0x2
	str	w8, [sp, #8]
	mov	w8, #3                          ; =0x3
	str	w8, [sp, #4]
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
.subsections_via_symbols
