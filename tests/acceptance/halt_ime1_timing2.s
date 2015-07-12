; This tests whether HALT adds any kind of delay in the IME=1 case
;
; HALT is expected to immediately service the interrupt, with exactly
; same timing as if a long series of NOP instructions were used to wait
; for the interrupt

.incdir "../common"
.include "common.s"

.macro clear_IF
  xor a
  ld_ff_a IF
.endm

.macro enable_IE_vblank
  ld a, INTR_VBLANK
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
  ld hl, finish_round1
  clear_IF
  ei

  nops 12
  xor a
  ld_ff_a DIV

  delay_long_time 2502
  nops 7

  di
  jp fail_round1

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
  ld hl, finish_round2
  clear_IF
  ei

  nops 11
  xor a
  ld_ff_a DIV

  delay_long_time 2502
  nops 8

  di
  jp fail_round2

finish_round2:
  ld_a_ff DIV
  ld e, a

  clear_IF
  ld hl, test_round3

  ei
  halt
  nop
  jp fail_halt

test_round3:
  ld hl, finish_round3
  clear_IF
  ei

  nops 12
  xor a
  ld_ff_a DIV

  halt
  nop
  jp fail_round3

finish_round3:
  ld_a_ff DIV
  ld b, a

  clear_IF
  ld hl, test_round4

  ei
  halt
  nop
  jp fail_halt

test_round4:
  ld hl, finish_round4
  clear_IF
  ei

  nops 11
  xor a
  ld_ff_a DIV

  halt
  nop
  jp fail_round4

finish_round4:
  ld_a_ff DIV
  ld c, a
  save_results
  assert_b $11
  assert_c $12
  assert_d $11
  assert_e $12
  jp process_results

fail_halt:
  test_failure_string "FAIL: HALT"

fail_round1:
  test_failure_string "FAIL: ROUND 1"

fail_round2:
  test_failure_string "FAIL: ROUND 2"

fail_round3:
  test_failure_string "FAIL: ROUND 3"

fail_round4:
  test_failure_string "FAIL: ROUND 4"

.org INTR_VEC_VBLANK
  jp hl
