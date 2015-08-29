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

; DIV increments are supposed to happen every 64 cycles,
; and the "internal counter" is supposed to reset when DIV is reset
;
; ld a, (hl) is expected to have the following timing:
; M = 0: instruction decoding
; M = 1: memory access from (HL)

; Verified results:
;   pass: DMG, MGB, SGB, SGB2, CGB, AGB, AGS
;   fail: -

.incdir "../common"
.include "common.s"

  ld hl, DIV

.macro reset_div
  xor a
  ld (hl), a
.endm

  ; --- Test: increment is too late

  reset_div
  nops 61
  ; DIV increment should happen at M = 2, so the memory read
  ; should not see the increment, and we should get A = $00
  ld a, (hl)
  ld b, a

  ; --- Test: internal counter reset

  ; padding so if the internal counter is not reset, the next
  ; test should incorrectly see the increment
  nops 27

  ; repeat earlier test
  reset_div
  nops 61
  ; DIV increment should happen at M = 2, so the memory read
  ; should not see the increment, and we should get A = $00
  ld a, (hl)
  ld c, a

  ; --- Test: increment is exactly on time

  reset_div
  nops 62
  ; DIV increment should happen at M = 1, so the memory read
  ; should see the increment, and we should get A = $01
  ld a, (hl)
  ld d, a

test_finish:
  save_results
  assert_b $00
  assert_c $00
  assert_d $01
  jp process_results
