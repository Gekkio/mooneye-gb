; This tests the behaviour of IE and IF flags by forcing a serial
; interrupt with a write to IF. The interrupt handler increments
; E, so we can track how many times the interrupt has been
; triggered

.incdir "../common"
.include "common.i"

  ; Make sure IE, IF, and E are all $00
  di
  xor a
  ld (IF), a
  ld (IE), a
  ld e, a
  ei

  ; Write serial interrupt bit to IF and wait
  ; Since IE is $00, we are *not* expecting an
  ; interrupt
  ld hl, IF
  ld a, $08
  ld (hl), a
  nops 64
  ld b, e
  ld a, (hl)
  ld c, a
  ; B contains counter E value
  ; C contains register IF value

  ; Write serial interrupt bit to IE and wait
  ; We already wrote it to IF, so now we expect
  ; one interrupt trigger
  ld hl, IE
  ld a, $08
  ld (hl), a
  nops 64
  ld d, e
  ld hl, IF
  ld a, (hl)
  ld e, a
  ; D contains counter E value
  ; E contains register IF value

  ; GBP MGB-001 / GBASP AGS-101 (probably DMG/GBC as well)
  ; B should contain $00
  ; C should contain $E8
  ; D should contain $01
  ; E should contain $E0

  jp finish

.org $58
  inc e
  reti
