; If IME=1, HALT is expected to immediately service an interrupt.
; So, if the interrupt service routine doesn't return,
; the instruction after HALT should never get executed

.incdir "../common"
.include "common.s"

  ei

  ; B = 0
  xor a
  ld b, a

  ; Enable timer interrupt
  ld a, $04
  ld_ff_a IE, a

  ; TIMA = $F0
  ld a, $F0
  ld_ff_a TIMA, a

  ; Start timer at 262144 Hz
  ld a, $05
  ld_ff_a TAC, a

  halt
  ; This should never get executed
  inc b

  test_failure

test_finish:
  ; GBP MGB-001 / GBC CGB-001 / GBASP AGS-101 (probably DMG/GBA as well)
  save_results
  assert_b $00
  jp process_results

.org $50
  jp test_finish
