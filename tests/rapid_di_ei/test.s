; This tests how sequential DI/EI instructions work by forcing a serial
; interrupt with a write to IE/IF. The interrupt handler increments
; E, so we can track how many times the interrupt has been
; triggered

.incdir "../common"
.include "common.i"

.macro reset
  ld a, $08
  ld (IF), a
  ld (IE), a
  xor a
  ld e, a
.endm

  di
  reset

  ; Rapid EI/DI should *not* result in any interrupts
  ei
  di
  ei
  di
  ld b, e

  reset

  ; EI followed by DI should *not* result in any interrupts
  ei
  di
  nop
  nop
  ld c, e

  reset

  ; A nop after EI should cause an interrupt
  ei
  nop
  di
  ld d, e

  reset

  ; Two nops after EI should cause an interrupt
  ei
  nop
  nop
  di

  ; GBP MGB-001 / GBASP AGS-101 (probably DMG/GBC as well)
  ; B should contain $00
  ; C should contain $00
  ; D should contain $01
  ; E should contain $01

  jp finish

.org $58
  inc e
  reti
