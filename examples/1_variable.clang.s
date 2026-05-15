	.build_version macos, 26, 0	sdk_version 26, 2
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_main                           ; -- Begin function main
	.p2align	2
_main:                                  ; @main
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #16
	.cfi_def_cfa_offset 16
	mov	w0, #0                          ; =0x0
	str	wzr, [sp, #12]
	mov	w8, #42                         ; =0x2a
	str	w8, [sp, #8]
	mov	w8, #24                         ; =0x18
	str	w8, [sp, #4]
	ldr	w8, [sp, #8]
	str	w8, [sp]
	ldr	w8, [sp, #4]
	str	w8, [sp, #8]
	ldr	w8, [sp]
	str	w8, [sp, #4]
	add	sp, sp, #16
	ret
	.cfi_endproc
                                        ; -- End function
.subsections_via_symbols
