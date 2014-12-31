; POP rr is expected to have the following timing:
; t = 0: instruction decoding
; t = 1: memory access for low byte
; t = 2: memory access for high byte

.incdir "../common"
.include "common.i"

  ld hl, DIV

.macro reset_div
  xor a
  ld (hl), a
.endm

  ; --- low byte tests

  ld sp, hl
  reset_div
  nops 61
  ; DIV increment happens at t = 2, so the low byte has already
  ; been popped and we should see $00
  pop bc
  ld d, c

  ld sp, DIV
  reset_div
  nops 62
  ; DIV increment happens at t = 1, so the low byte should be popped
  ; at the same time and we should see $01
  pop bc
  ld e, c

  ; Save the first two results to temporary storage
  ld sp, $CFFF
  push de

  ; --- high byte tests

  ld sp, DIV - 1
  reset_div
  nops 60
  ; DIV increment happens at t = 3, so the high byte has already
  ; been popped and we should see $00
  pop bc
  ld d, b

  ld sp, DIV - 1
  reset_div
  nops 61
  ; DIV increment happens at t = 2, so the high byte should be popped
  ; at the same time and we should see $01
  pop bc
  ld e, b

  ld sp, DIV - 1
  reset_div
  nops 62
  ; DIV increment happens at t = 1, so the high byte popping
  ; should see the increment and we should see $01
  pop af

  ; Restore old results from temporary storage
  ld sp, $CFFD
  pop bc

test_finish:
  ; GBP MGB-001 / GBASP AGS-101 (probably DMG/GBC as well)
  save_results
  assert_a $01
  assert_b $00
  assert_c $01
  assert_d $00
  assert_e $01
  jp print_results
