; This tests EI instruction timing by forcing a serial
; interrupt with a write to IE/IF.

.incdir "../common"
.include "common.i"

  di
  ld a, $08
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
  ; GBP MGB-001 / GBASP AGS-101 (probably DMG/GBC as well)
  save_results
  assert_b $01
  assert_e $01
  jp print_results

.org $58
  inc e
  jp test_finish
