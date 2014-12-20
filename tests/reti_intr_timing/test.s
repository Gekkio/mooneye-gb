; This tests RETI instruction interrupt enable timing

.incdir "../common"
.include "common.i"

  di
  ld a, $09 ; Enable both vblank and serial interrupts (handlers at $40, $58)
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

  ; GBP MGB-001 / GBASP AGS-101 (probably DMG/GBC as well)
  ; B should contain $01
  ; D should contain $01
  ; E should contain $01

  jp finish

.org $40
  inc d
  reti

.org $58
  inc e
  jp finish
