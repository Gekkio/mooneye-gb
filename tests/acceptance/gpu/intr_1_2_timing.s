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

; Tests how long does it take to get from STAT mode=1 interrupt to STAT mode=2 interrupt
; No sprites, scroll or window.

.incdir "../../common"
.include "common.s"

.macro clear_interrupts
  xor a
  ld_ff_a IF
.endm

  di
  wait_vblank
  ld hl, STAT
  ld a, INTR_STAT
  ld_ff_a IE

.macro test_iter ARGS delay
  call setup_and_wait_mode1
  nops delay
  call setup_and_wait_mode2
.endm

  test_iter 5
  ld d, b
  test_iter 4
  ld e, b
  save_results
  assert_d $14
  assert_e $15
  jp process_results

setup_and_wait_mode1:
  wait_ly $42
  ld a, %00010000
  ld_ff_a STAT
  clear_interrupts
  ei

  halt
  nop
  jp fail_halt

setup_and_wait_mode2:
  ld a, %00100000
  ld_ff_a STAT
  clear_interrupts
  ei
  xor a
  ld b,a
- inc b
  jr -

fail_halt:
  test_failure_string "FAIL: HALT"

.org INTR_VEC_STAT
  add sp,+2
  ret
