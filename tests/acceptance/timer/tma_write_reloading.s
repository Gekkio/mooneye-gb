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

; This test checks when writes to the TMA register get picked while
; the timer is reloading.

; Verified results:
;   pass: DMG, MGB, SGB, SGB2, CGB, AGB, AGS
;   fail: -

.incdir "../../common"
.include "common.s"

test:
  di
  xor a
  ld b,$fe
  ld h,$7f
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
  ldh (<TIMA), a
  ldh (<DIV),a
  ld a,h
  nops 16
  nops 12
  ldh (<TMA),a
  ldh a,(<TIMA)
  ld d,a

  ld a,b
  ldh (<TIMA), a
  ldh (<TMA),a
  ldh (<DIV),a
  ldh (<TIMA), a
  ldh (<DIV),a
  ld a,h
  nops 16
  nops 13
  ldh (<TMA),a
  ldh a,(<TIMA)
  ld e,a

  ld a,b
  ldh (<TIMA), a
  ldh (<TMA),a
  ldh (<DIV),a
  ldh (<TIMA), a
  ldh (<DIV),a
  ld a,h
  nops 16
  nops 14
  ldh (<TMA),a
  ldh a,(<TIMA)
  ld c,a

  ld a,b
  ldh (<TIMA), a
  ldh (<TMA),a
  ldh (<DIV),a
  ldh (<TIMA), a
  ldh (<DIV),a
  ld a,h
  nops 16
  nops 15
  ldh (<TMA),a
  ldh a,(<TIMA)
  ld l,a

  save_results
  assert_c $fe
  assert_d $7f
  assert_e $7f
  assert_l $fe
  jp process_results

