; This file is part of Mooneye GB.
; Copyright (C) 2014-2015 Joonas Javanainen <joonas.javanainen@gmail.com>
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

; POP rr is expected to have the following timing:
; M = 0: instruction decoding
; M = 1: memory access for low byte
; M = 2: memory access for high byte

.incdir "../common"
.include "common.s"

  ld hl, DIV

.macro reset_div
  xor a
  ld (hl), a
.endm

  ; --- low byte tests

  ld sp, hl
  reset_div
  nops 61
  ; DIV increment happens at M = 2, so the low byte has already
  ; been popped and we should see $00
  pop bc
  ld d, c

  ld sp, DIV
  reset_div
  nops 62
  ; DIV increment happens at M = 1, so the low byte should be popped
  ; at the same time and we should see $01
  pop bc
  ld e, c

  ; Save the first two results to temporary storage
  ld sp, $CFFF
  push de

  ; --- high byte tests

  ld sp, DIV - 1
  reset_div
  nops 60
  ; DIV increment happens at M = 3, so the high byte has already
  ; been popped and we should see $00
  pop bc
  ld d, b

  ld sp, DIV - 1
  reset_div
  nops 61
  ; DIV increment happens at M = 2, so the high byte should be popped
  ; at the same time and we should see $01
  pop bc
  ld e, b

  ld sp, DIV - 1
  reset_div
  nops 62
  ; DIV increment happens at M = 1, so the high byte popping
  ; should see the increment and we should see $01
  pop af

  ; Restore old results from temporary storage
  ld sp, $CFFD
  pop bc

test_finish:
  ; GBP MGB-001 / GBC CGB-001 / GBASP AGS-101 (probably DMG/GBA as well)
  save_results
  assert_a $01
  assert_b $00
  assert_c $01
  assert_d $00
  assert_e $01
  jp process_results
