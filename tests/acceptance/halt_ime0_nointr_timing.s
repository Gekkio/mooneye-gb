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

; This tests whether HALT adds any kind of delay in the IME=0 case
;
; HALT is expected to immediately continue execution, with exactly
; same timing as if a long series of NOP instructions were used to wait
; for the interrupt

; Verified results:
;   pass: DMG, MGB, SGB, SGB2, CGB, AGB, AGS
;   fail: -

.incdir "../common"
.include "common.s"

.macro clear_IF
  xor a
  ld_ff_a IF
.endm

.macro enable_IE_vblank
  ld a, INTR_VBLANK
  ld_ff_a IE
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
  ld hl, fail_intr
  clear_IF

  nops 13
  xor a
  ld_ff_a DIV

  halt
  nops 6 ; Equivalent to interrupt + JP HL in the IME=1 case

finish_round1:
  ld_a_ff DIV
  ld d, a

  clear_IF
  ld hl, test_round2

  ei
  halt
  nop
  jp fail_halt

test_round2:
  ld hl, fail_intr
  clear_IF

  nops 12
  xor a
  ld_ff_a DIV

  halt
  nops 6 ; Equivalent to interrupt + JP HL in the IME=1 case

finish_round2:
  ld_a_ff DIV
  ld e, a
  save_results
  assert_d $11
  assert_e $12
  jp process_results

fail_halt:
  test_failure_string "FAIL: HALT"

fail_intr:
  test_failure_string "FAIL: INTERRUPT"

.org INTR_VEC_VBLANK
  jp hl
