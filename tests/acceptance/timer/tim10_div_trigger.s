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

; This test verifies that the timer count changes are actually triggered
; by bit 5 going low when writing to the DIV register in 65536 Hz mode.
;
; 32 cycles after resetting the internal div counter, bit 5 of the
; internal div counter will have been set. Writing to the DIV register
; at this time will cause bit 5 to change from high to low which in
; turn triggers a timer increment.

; Verified results:
;   pass: DMG, MGB, SGB, SGB2, CGB, AGB, AGS
;   fail: -

.incdir "../../common"
.include "common.s"

test:
  di
  xor a
  ld b,4
  ldh (<IE), a
  ldh (<IF), a
  ldh (<DIV), a
  ld a, b
  ldh (<TIMA), a
  ldh (<TMA),a
  ld a, %00000110 ; Start 65536 Hz timer (64 cycles)
  ldh (<TAC), a
  ld a,b
  ldh (<DIV),a
  nop
  ldh (<TIMA), a
  nop
  ldh (<DIV),a
  nops 12
  ldh a,(<TIMA)
  ld d,a

  ld a,b
  ldh (<TIMA), a
  ldh (<DIV),a
  nop
  ldh (<TIMA), a
  nop
  ldh (<DIV),a
  nops 13
  ldh a,(<TIMA)
  ld e,a

  save_results
  assert_d $05
  assert_e $06
  jp process_results

