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

; This test checks when the timer count changes in 16384 Hz mode.
;
; The TIMA register is expected to increment every 256 cycles after the
; div counter has been reset.

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
  ld a, %00000111 ; Start 16384 Hz timer (256 cycles)
  ldh (<TAC), a
  xor a
  ldh (<DIV),a
  ld a,b
  ldh (<TIMA), a
  xor a
  ldh (<DIV),a
  nops 60
  ldh a,(<TIMA)
  ld d,a

  ld a,b
  ldh (<TIMA), a
  xor a
  ldh (<DIV),a
  ld a,b
  ldh (<TIMA), a
  xor a
  ldh (<DIV),a
  nops 61
  ldh a,(<TIMA)
  ld e,a

  save_results
  assert_d $04
  assert_e $05
  jp process_results

