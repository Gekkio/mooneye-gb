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

; Serving an interrupt is supposed to take 5 M-cycles.
; We know from div_timing that this code should not see a div increment:
;   reset_div
;     nops 61
;   ld a, (hl)
; On the other hand, this code should see the increment:
;   reset_div
;     nops 62
;   ld a, (hl)
; We set up a similar scenario by triggering an interrupt using IE/IF flags.
; In total we have
;   reset_div
;     x nops               x cycles
;     trigger_intr 2 + 3 = 5 cycles
;     interrupt handling:  5 cycles
;     jp hl                1 cycle
;   ld a, (bc)
; So, x=50 is equivalent to the nops 61 case,
; and x=51 is equivalent to the nops 62 case

.incdir "../common"
.include "common.s"

  ld bc, DIV

  ld a, INTR_SERIAL
  ld_ff_a IE

.macro reset_div
  xor a
  ld (bc), a
.endm

.macro trigger_intr
  ld a, $08
  ld_ff_a IF
.endm

test_round1:
  ei
  ld hl, finish_round1

  reset_div
  nops 50
  trigger_intr

  ; never executed
  test_failure_string "FAIL: ROUND 1"

finish_round1:
  ld a, (bc)
  ld d, a

test_round2:
  ei
  ld hl, finish_round2

  reset_div
  nops 51
  trigger_intr

  ; never executed
  test_failure_string "FAIL: ROUND 2"

finish_round2:
  ld a, (bc)
  ld e, a

  jp test_finish

test_finish:
  ; GBP MGB-001 / GBC CGB-001 / GBASP AGS-101 (probably DMG/GBA as well)
  save_results
  assert_d $00
  assert_e $01
  jp process_results

.org INTR_VEC_SERIAL
  jp hl
