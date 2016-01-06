; This file is part of Mooneye GB.
; Copyright (C) 2014-2016 Joonas Javanainen <joonas.javanainen@gmail.com>
;
; Mooneye GB is free software: you can redistribute it and/or modify
; it under the terms of the GNU General Public License as published by
; the Free Software Foundation, either version 3 of the License, or
; (at your option) any later version.
;
; Mooneye GB is distributed in the hope that it will be useful,
; but WITHOUT ANY WARRANTY; without even the implied warranty of
; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
; GNU General Public License for more details.
;
; You should have received a copy of the GNU General Public License
; along with Mooneye GB.  If not, see <http://www.gnu.org/licenses/>.

; This tests how sequential DI/EI instructions work by forcing a serial
; interrupt with a write to IE/IF. The interrupt handler increments
; E, so we can track how many times the interrupt has been
; triggered

; Verified results:
;   pass: DMG, MGB, SGB, SGB2, CGB, AGB, AGS
;   fail: -

.incdir "../common"
.include "common.s"

.macro reset
  ld a, INTR_SERIAL
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

test_finish:
  save_results
  assert_b $00
  assert_c $00
  assert_d $01
  assert_e $01
  jp process_results

.org INTR_VEC_SERIAL
  inc e
  reti
