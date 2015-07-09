; This tests whether HALT adds any kind of delay in the IME=0 case
;
; HALT is expected to immediately continue execution, with exactly
; same timing as if a long series of NOP instructions were used to wait
; for the interrupt

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
  wait_ly 10
  enable_IE_vblank

  clear_IF
  ld hl, test_round1

  ei
  halt
  nop
  jp fail_halt

test_round1:
  ld hl, fail_intr
  clear_IF

  nops 13
  xor a
  ld_ff_a DIV

  halt
  nops 6 ; Equivalent to interrupt + JP HL in the IME=1 case

finish_round1:
  ld_a_ff DIV
  ld d, a

  clear_IF
  ld hl, test_round2

  ei
  halt
  nop
  jp fail_halt

test_round2:
  ld hl, fail_intr
  clear_IF

  nops 12
  xor a
  ld_ff_a DIV

  halt
  nops 6 ; Equivalent to interrupt + JP HL in the IME=1 case

finish_round2:
  ld_a_ff DIV
  ld e, a
  save_results
  assert_d $11
  assert_e $12
  jp process_results

fail_halt:
  test_failure_string "FAIL: HALT"

fail_intr:
  test_failure_string "FAIL: INTERRUPT"

.org $40
  jp hl
