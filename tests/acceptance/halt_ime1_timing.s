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

; If IME=1, HALT is expected to immediately service an interrupt.
; So, if the interrupt service routine doesn't return,
; the instruction after HALT should never get executed

.incdir "../common"
.include "common.s"

  ei

  ; B = 0
  xor a
  ld b, a

  ; Enable timer interrupt
  ld a, INTR_TIMER
  ld_ff_a IE, a

  ; TIMA = $F0
  ld a, $F0
  ld_ff_a TIMA, a

  ; Start timer at 262144 Hz
  ld a, $05
  ld_ff_a TAC, a

  halt
  ; This should never get executed
  inc b

  test_failure

test_finish:
  ; GBP MGB-001 / GBC CGB-001 / GBASP AGS-101 (probably DMG/GBA as well)
  save_results
  assert_b $00
  jp process_results

.org INTR_VEC_TIMER
  jp test_finish
