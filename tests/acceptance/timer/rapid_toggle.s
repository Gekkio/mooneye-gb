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

; This test rapidly starts and stops the timer.
; There are two behaviours that affect the test:
;   1. starting or stopping the timer does *not* reset its internal counter,
;      so repeated starting and stopping does not prevent timer increments
;   2. the timer circuit design causes some unexpected timer increases

; Common reasons for failing this test:
;   "FAIL: NO INTR"
;     Your emulator starts counting the timer when it is started or stopped.
;     This is incorrect, because in real hardware starting/stopping the timer
;     simply flips a single bit, and the timer state is otherwise preserved.
;   BC < $FFF8
;     Your emulator does not emulate the unexpected timer increases, so the
;     interrupt happens too late.

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
  ld a, $F0
  ldh (<TIMA), a
  ld a, %00000100 ; Start 4096 Hz timer
  ldh (<TAC), a

  ld bc, $FFFF

  ei

- ld a, %00000100 ; Start 4096 Hz timer
  ldh (<TAC), a
  ld a, %00000000 ; Stop timer
  ldh (<TAC), a
  dec bc
  ld a, c
  or b
  jr nz, -

  test_failure_string "FAIL: NO INTR"

test_finish:
  save_results
  assert_b $FF
  assert_c $D9
  jp process_results

.org INTR_VEC_TIMER
  jp test_finish
