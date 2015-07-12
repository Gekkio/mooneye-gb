; This tests RETI instruction interrupt enable timing

.incdir "../common"
.include "common.s"

  di
  ld a, INTR_VBLANK | INTR_SERIAL
  ld (IF), a
  ld (IE), a
  xor a
  ld b, a
  ld d, a
  ld e, a

  ; We're expecting to see the effect of exactly one INC B instruction
  ; before we get the vblank interrupt (handler at $40)
  ei
  inc b
  ; Handler $40 is supposed to be executed here
  ; We expect not to see the second inc b, because RETI causes us to
  ; jump to handler $58
  inc b

test_finish:
  ; GBP MGB-001 / GBC CGB-001 / GBASP AGS-101 (probably DMG/GBA as well)
  save_results
  assert_b $01
  assert_d $01
  assert_e $01
  jp process_results

.org INTR_VEC_VBLANK
  inc d
  reti

.org INTR_VEC_SERIAL
  inc e
  jp test_finish
