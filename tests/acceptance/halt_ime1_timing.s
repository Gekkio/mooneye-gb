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

; If IME=1, HALT is expected to immediately service an interrupt.
; So, if the interrupt service routine doesn't return,
; the instruction after HALT should never get executed

; Verified results:
;   pass: DMG, MGB, SGB, SGB2, CGB, AGB, AGS
;   fail: -

.incdir "../common"
.include "common.s"

  ei

  ; B = 0
  xor a
  ld b, a

  ; Enable timer interrupt
  ld a, INTR_TIMER
  ldh (<IE), a

  ; TIMA = $F0
  ld a, $F0
  ldh (<TIMA), a

  ; Start timer at 262144 Hz
  ld a, $05
  ldh (<TAC), a

  halt
  ; This should never get executed
  inc b

  test_failure

test_finish:
  save_results
  assert_b $00
  jp process_results

.org INTR_VEC_TIMER
  jp test_finish
