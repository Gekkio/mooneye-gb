; This tests DI instruction timing by setting up a vblank interrupt
; interrupt with a write to IE.
;
; This test is for DMG/MGB, so DI is expected to disable interrupts
; immediately
; On CGB/GBA DI has a delay and this test fails in round 2!!

.incdir "../common"
.include "common.s"

.macro delay_long_time
  ld a, $09
  ld b, a
  ld a, $c9
  ld c, a

delay_long_time_\@:
  dec bc
  ld a,b
  or c
  jr nz, delay_long_time_\@
.endm

  di
  ld a, $01
  ld_ff_a IE

  ld hl, test_round1
  wait_vblank
  xor a
  ld_ff_a IF
  ei

  halt
  nop
  jp fail_halt

test_round1:
  ld hl, finish_round1
  ei

  delay_long_time
  nops 6

  ; This DI should never get executed
  di
  jp fail_round1

finish_round1:
  ld hl, test_round2
  wait_vblank
  xor a
  ld_ff_a IF
  ei

  halt
  nop
  jp fail_halt

test_round2:
  ld hl, fail_round2
  ei

  delay_long_time
  nops 5

  ; This time we let DI execute, because there is one less NOP
  di
  ; If DI doesn't have an immediate effect, we would get an interrupt here and
  ; fail the test.
  nop

test_finish:
  test_ok

fail_halt:
  test_failure_string "FAIL: HALT"

fail_round1:
  test_failure_string "FAIL: ROUND 1"

fail_round2:
  test_failure_string "FAIL: ROUND 2"

.org $40
  jp hl
