; test

.incdir "../../common"
.include "common.s"

.macro clear_interrupts
  xor a
  ld_ff_a IF
.endm

.macro scroll_x ARGS value
  ld a, value
  ld_ff_a SCX
.endm

  di
  wait_vblank
  ld hl, LY
  ld a, $08
  ld_ff_a STAT
  ld a, INTR_STAT
  ld_ff_a IE

.macro perform_test ARGS scanline delay_a delay_b
  ld d, scanline
  .if scanline == $00
    ld e, $99
  .else
    ld e, scanline - 1
  .endif
  test_iter scanline delay_a
  cp $14
  jp nz, test_fail
  test_iter scanline delay_b
  cp $13
  jp nz, test_fail
.endm

.macro test_iter ARGS scanline delay
  call setup_and_wait
  nops delay
  ld c, $00
_test_iter_\@:
  inc c
  ld a, (hl)
  cp scanline + 1
  jr nz, _test_iter_\@
  ld a, c
.endm

  perform_test $42 2 3
  perform_test $43 2 3
  scroll_x $01
  perform_test $42 1 2
  perform_test $43 1 2
  scroll_x $02
  perform_test $42 1 2
  perform_test $43 1 2
  scroll_x $03
  perform_test $42 1 2
  perform_test $43 1 2
  scroll_x $04
  perform_test $42 1 2
  perform_test $43 1 2
  scroll_x $05
  perform_test $42 0 1
  perform_test $43 0 1
  scroll_x $06
  perform_test $42 0 1
  perform_test $43 0 1
  scroll_x $07
  perform_test $42 0 1
  perform_test $43 0 1
  scroll_x $08
  perform_test $42 2 3
  perform_test $43 2 3

  test_ok

test_fail:
  ld_a_ff SCX
  save_results
  jp process_results

setup_and_wait:
  wait_vblank
- ld a, (hl)
  cp e
  jr nz, -
  clear_interrupts
  ei

  halt
  nop
  jp fail_halt

fail_halt:
  test_failure_string "FAIL: HALT"

.org INTR_VEC_STAT
  add sp,+2
  ret
