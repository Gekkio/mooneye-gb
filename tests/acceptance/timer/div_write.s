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

; This test verifies that the timer is affected by resetting the DIV register
; by writing to it. The timer uses the same internal counter as the DIV
; register, so resetting DIV also resets the timer.
; The basic idea of this test is very simple:
;   1. start the timer
;   2. keep resetting DIV in a loop by writing to it
;   3. run N iterations of the loop
;   4. if an interrupt happened, test failed

; Common reasons for failing this test:
;   "FAIL: INTR"
;     Your emulator does not use the internal counter for the timer, so DIV
;     resets did not prevent the timer increase and interrupt from happening

; Verified results:
;   pass: DMG, MGB, SGB, SGB2, CGB, AGB, AGS
;   fail: -

.incdir "../../common"
.include "common.s"

test:
  ld a, INTR_TIMER
  ldh (<IE), a
  xor a
  ldh (<IF), a
  ldh (<DIV), a
  ld a, $FF
  ldh (<TIMA), a
  ld a, %00000100 ; Start 4096 Hz timer
  ldh (<TAC), a

  ld bc, $FFFF ; loop counter

  ei

- xor a
  ldh (<DIV), a
  dec bc
  ld a, c
  or b
  jr nz, -

  di
  test_ok

test_failure:
  test_failure_string "FAIL: INTR"

.org INTR_VEC_TIMER
  jp test_failure
