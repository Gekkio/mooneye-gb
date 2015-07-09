; We know that the effect of EI has a delay. This tests how EI before HALT behaves.
;
; If EI is before HALT, the HALT instruction is expected to perform its normal
; IME=1 behaviour

.incdir "../common"
.include "common.s"

.macro clear_IF
  xor a
  ld_ff_a IF
.endm

.macro enable_IE_vblank
  ld a, $01
  ld_ff_a IE
.endm

  di
  wait_ly $00
  clear_IF
  enable_IE_vblank

  ei
  halt
  di

result_ime0:
  test_failure_string "IME=0"

result_ime1:
  test_ok

.org $40
  jp result_ime1
