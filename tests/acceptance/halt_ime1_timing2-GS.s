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

; This tests whether HALT adds any kind of delay in the IME=1 case
;
; HALT is expected to immediately service the interrupt, with exactly
; same timing as if a long series of NOP instructions were used to wait
; for the interrupt

; Verified results:
;   pass: DMG, MGB, SGB, SGB2
;   fail: CGB, AGB, AGS

.incdir "../common"
.include "common.s"

.macro clear_IF
  xor a
  ldh (<IF), a
.endm

.macro enable_IE_vblank
  ld a, INTR_VBLANK
  ldh (<IE), a
.endm

  di
  wait_ly 10
  enable_IE_vblank

  clear_IF
  ld hl, test_round1

  ei
  halt
  nop
  jp fail_halt

test_round1:
  ld hl, finish_round1
  clear_IF
  ei

  nops 12
  xor a
  ldh (<DIV), a

  delay_long_time 2502
  nops 7

  di
  jp fail_round1

finish_round1:
  ldh a, (<DIV)
  ld d, a

  clear_IF
  ld hl, test_round2

  ei
  halt
  nop
  jp fail_halt

test_round2:
  ld hl, finish_round2
  clear_IF
  ei

  nops 11
  xor a
  ldh (<DIV), a

  delay_long_time 2502
  nops 8

  di
  jp fail_round2

finish_round2:
  ldh a, (<DIV)
  ld e, a

  clear_IF
  ld hl, test_round3

  ei
  halt
  nop
  jp fail_halt

test_round3:
  ld hl, finish_round3
  clear_IF
  ei

  nops 12
  xor a
  ldh (<DIV), a

  halt
  nop
  jp fail_round3

finish_round3:
  ldh a, (<DIV)
  ld b, a

  clear_IF
  ld hl, test_round4

  ei
  halt
  nop
  jp fail_halt

test_round4:
  ld hl, finish_round4
  clear_IF
  ei

  nops 11
  xor a
  ldh (<DIV), a

  halt
  nop
  jp fail_round4

finish_round4:
  ldh a, (<DIV)
  ld c, a
  save_results
  assert_b $11
  assert_c $12
  assert_d $11
  assert_e $12
  jp process_results

fail_halt:
  test_failure_string "FAIL: HALT"

fail_round1:
  test_failure_string "FAIL: ROUND 1"

fail_round2:
  test_failure_string "FAIL: ROUND 2"

fail_round3:
  test_failure_string "FAIL: ROUND 3"

fail_round4:
  test_failure_string "FAIL: ROUND 4"

.org INTR_VEC_VBLANK
  jp hl
