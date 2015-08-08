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

; This tests DI instruction timing by setting up a vblank interrupt
; interrupt with a write to IE.
;
; This test is for DMG/MGB, so DI is expected to disable interrupts
; immediately
; On CGB/GBA DI has a delay and this test fails in round 2!!

.incdir "../common"
.include "common.s"

  di
  ld a, INTR_VBLANK
  ld_ff_a IE

  ld hl, test_round1
  wait_vblank
  xor a
  ld_ff_a IF
  ei

  halt
  nop
  jp fail_halt

test_round1:
  ld hl, finish_round1
  ei

  delay_long_time 2505
  nops 6

  ; This DI should never get executed
  di
  jp fail_round1

finish_round1:
  ld hl, test_round2
  wait_vblank
  xor a
  ld_ff_a IF
  ei

  halt
  nop
  jp fail_halt

test_round2:
  ld hl, fail_round2
  ei

  delay_long_time 2505
  nops 5

  ; This time we let DI execute, because there is one less NOP
  di
  ; If DI doesn't have an immediate effect, we would get an interrupt here and
  ; fail the test.
  nop

test_finish:
  test_ok

fail_halt:
  test_failure_string "FAIL: HALT"

fail_round1:
  test_failure_string "FAIL: ROUND 1"

fail_round2:
  test_failure_string "FAIL: ROUND 2"

.org INTR_VEC_VBLANK
  jp hl
