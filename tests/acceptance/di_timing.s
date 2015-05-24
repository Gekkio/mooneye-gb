; This tests DI instruction timing by setting up a timer interrupt
; interrupt with a write to IE.
;
; DI is expected to disable interrupts immediately

.incdir "../common"
.include "common.s"

.macro start_timer
  ei
  ld a, $05
  ld_ff_a TAC
.endm

.macro stop_timer
  di
  xor a
  ld_ff_a TAC
.endm

.macro reset_tima
  ld a, $fa
  ld_ff_a TIMA
.endm

  di
  ld a, $04
  ld_ff_a IE

  xor a
  ld_ff_a TMA

test_round1:
  stop_timer
  reset_tima
  ld hl, test_round2

  start_timer
  ; Timer increments every 4 cycles, and we have TIMA = $FA in the beginning,
  ; so we should see an interrupt 24 cycles after timer was started (= 24 nops)
  nops 24
  ; This DI should never get executed
  jp fail_round1

test_round2:
  stop_timer
  reset_tima
  ld hl, fail_round2

  start_timer
  ; This time we let DI execute
  nops 23
  di
  ; If DI doesn't have an immediate effect, we would get an interrupt here and
  ; fail the test.
  nop

test_finish:
  test_ok

fail_round1:
  test_failure_string "FAIL: ROUND 1"

fail_round2:
  test_failure_string "FAIL: ROUND 2"

.org $50
  jp hl
