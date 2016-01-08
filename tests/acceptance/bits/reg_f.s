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

; This test checks that bottom 4 bits of the F register always return 0

; Verified results:
;   pass: DMG, MGB, SGB, SGB2, CGB, AGB, AGS
;   fail: -

.incdir "../../common"
.include "common.s"

  di
  ld sp, $FFFE

; Write all 1s (= $FF)
  ld e, $FF
  push de
  pop af
  push af

; Write all 0s (= $00)
  ld e, $00
  push de
  pop af
  push af

  pop de
  pop bc

test_finish:
  save_results
  assert_c $F0
  assert_e $00
  jp process_results
