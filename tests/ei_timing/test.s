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

  ; GBP MGB-001 / GBASP AGS-101 (probably DMG/GBC as well)
  ; B should contain $01
  ; E should contain $01

  jp finish

.org $58
  inc e
  jp finish
