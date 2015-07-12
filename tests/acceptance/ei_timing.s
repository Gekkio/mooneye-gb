; This tests EI instruction timing by forcing a serial
; interrupt with a write to IE/IF.

.incdir "../common"
.include "common.s"

  di
  ld a, INTR_SERIAL
  ld (IF), a
  ld (IE), a
  xor a
  ld b, a
  ld e, a

  ; We're expecting to see the effect of exactly one INC B instruction
  ei
  inc b
  inc b
  inc b

test_finish:
  ; GBP MGB-001 / GBC CGB-001 / GBASP AGS-101 (probably DMG/GBA as well)
  save_results
  assert_b $01
  assert_e $01
  jp process_results

.org INTR_VEC_SERIAL
  inc e
  jp test_finish
