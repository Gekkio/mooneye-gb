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

; Tests how SCX affects the duration between STAT mode=0 interrupt and LY increment.
; No sprites or window.
;
; Expected behaviour:
;   (SCX mod 8) = 0   => LY increments 51 cycles after STAT interrupt
;   (SCX mod 8) = 1-4 => LY increments 50 cycles after STAT interrupt
;   (SCX mod 8) = 5-7 => LY increments 49 cycles after STAT interrupt

; Verified results:
;   pass: DMG, MGB, SGB, SGB2
;   fail: CGB, AGB, AGS

.incdir "../../common"
.include "common.s"

.macro clear_interrupts
  xor a
  ld_ff_a IF
.endm

.macro scroll_x ARGS value
  ld a, value
  ld_ff_a SCX
.endm

  di
  wait_vblank
  ld hl, LY
  ld a, $08
  ld_ff_a STAT
  ld a, INTR_STAT
  ld_ff_a IE

.macro perform_test ARGS scanline delay_a delay_b
  ld d, scanline - 1
  ld e, scanline
  test_iter scanline delay_a
  cp d
  jp nz, test_fail
  test_iter scanline delay_b
  cp e
  jp nz, test_fail
.endm

.macro test_iter ARGS scanline delay
  call setup_and_wait
  ; Interrupt processing: 5
  ; Interrupt vector: 4 + 4 = 8
  call standard_delay
  ; 6 + 23 + 4
  nops delay
  ; N cycles
  ld a, (hl)
  ; 1 cycle for decoding before memory read
  ; 5 + 4 + 4 + 6 + 23 + 4 + N + 1
  ; = 47 + N cycles before memory read
.endm

  perform_test $42 2 3
  perform_test $43 2 3
  scroll_x $01
  perform_test $42 1 2
  perform_test $43 1 2
  scroll_x $02
  perform_test $42 1 2
  perform_test $43 1 2
  scroll_x $03
  perform_test $42 1 2
  perform_test $43 1 2
  scroll_x $04
  perform_test $42 1 2
  perform_test $43 1 2
  scroll_x $05
  perform_test $42 0 1
  perform_test $43 0 1
  scroll_x $06
  perform_test $42 0 1
  perform_test $43 0 1
  scroll_x $07
  perform_test $42 0 1
  perform_test $43 0 1
  scroll_x $08
  perform_test $42 2 3
  perform_test $43 2 3

  test_ok

test_fail:
  ld b, a
  ld_a_ff SCX
  save_results
  ; A = SCX
  ; B = LY value
  ; D = scanline - 1
  ; E = scanline
  test_failure_dump

standard_delay:
  nops 23
  ret

setup_and_wait:
  wait_vblank
- ld_a_ff LY
  cp d
  jr nz, -
  clear_interrupts
  ei

  halt
  nop
  jp fail_halt

fail_halt:
  test_failure_string "FAIL: HALT"

.org INTR_VEC_STAT
  add sp,+2
  ret
