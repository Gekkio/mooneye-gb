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

; This test checks that the IE register has no unused bits
;
; The Game Boy only uses the low 5 bits of IE, but the upper 3 bits
; are still writable and readable normally. It is very likely that
; IE is technically not a separate register, but simply a part of
; the high RAM area.

; Verified results:
;   pass: DMG, MGB, SGB, SGB2, CGB, AGB, AGS
;   fail: -

.incdir "../common"
.include "common.s"

  di

; Write all 1s (= $FF)
  ld a, $FF
  ldh (<IE), a
  ldh a, (<IE)
  ld b, a

; Write all 0s (= $00)
  ld a, $00
  ldh (<IE), a
  ldh a, (<IE)
  ld c, a

test_finish:
  save_results
  assert_b $FF
  assert_c $00
  jp process_results
