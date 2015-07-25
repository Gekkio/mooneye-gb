; Tests how long does it take to get from STAT mode=1 interrupt to STAT mode=2 interrupt
; No sprites, scroll or window.

.incdir "../../common"
.include "common.s"

.macro clear_interrupts
  xor a
  ld_ff_a IF
.endm

  di
  wait_vblank
  ld hl, STAT
  ld a, INTR_STAT
  ld_ff_a IE

.macro test_iter ARGS delay
  call setup_and_wait_mode1
  nops delay
  call setup_and_wait_mode2
.endm

  test_iter 5
  ld d, b
  test_iter 4
  ld e, b
  save_results
  assert_d $14
  assert_e $15
  jp process_results

setup_and_wait_mode1:
  wait_ly $42
  ld a, %00010000
  ld_ff_a STAT
  clear_interrupts
  ei

  halt
  nop
  jp fail_halt

setup_and_wait_mode2:
  ld a, %00100000
  ld_ff_a STAT
  clear_interrupts
  ei
  xor a
  ld b,a
- inc b
  jr -

fail_halt:
  test_failure_string "FAIL: HALT"

.org INTR_VEC_STAT
  add sp,+2
  ret
