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

; This test checks that the OAM area has no unused bits
; On DMG the sprite flags have unused bits, but they are still
; writable and readable normally

; Verified results:
;   pass: DMG, MGB, SGB, SGB2, CGB, AGB, AGS
;   fail: -

.incdir "../common"
.include "common.s"

  di
  wait_vblank
  disable_lcd

  ld hl, OAM
  ld b, $a0

-
; Write all 1s (= $FF) and expect the same value back
  ld a, $FF
  ld (hl), a
  ld a, (hl)
  cp $FF
  jp nz, fail_1

; Write all 0s (= $00) and expect the same value back
  ld a, $00
  ld (hl), a
  ld a, (hl)
  cp $00
  jp nz, fail_0
  
  inc hl
  dec b
  jr nz, -

test_finish:
  test_ok

fail_1:
  test_failure_string "FAIL: ALL 1s"
fail_0:
  test_failure_string "FAIL: ALL 0s"
