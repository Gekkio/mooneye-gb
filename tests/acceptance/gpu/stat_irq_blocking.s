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

; This tests how the internal STAT IRQ signal can block
; subsequent STAT interrupts if the signal is never cleared

; Verified results:
;   pass: DMG, MGB, CGB, AGB, AGS
;   fail: -

.incdir "../../common"
.include "common.s"

  di

test_round1:
  ld b, $00
  ld hl, test_round2
  wait_vblank

  ld a, $10 ; enable mode=1 interrupt
  ldh (<STAT), a
  ei
  ld a, INTR_STAT
  ldh (<IE), a

  ; We are already in vblank, but an interrupt should occur
fail_round1:
  di
  test_failure_string "FAIL: MODE=1 INTR"

  ; The STAT mode=1 interrupt should throw us here
test_round2:
  ld hl, fail_round2
  ei

  ld a, $78; enable all stat interrupts
  ldh (<STAT), a

  ; Now that all stat interrupts are enabled, the only mode
  ; that can clear the internal interrupt line is mode 3
  ; However, if we arrange things so that LY=LYC coincidence
  ; is set before mode 3 and kept set, the interrupt line is not cleared
  ld b, $00
ly_iteration:
  ld a, b
  cp $90 ; exit if LY>=144
  jr nc, finish_round2

  ldh (<LYC), a

  ; Wait until LY = b
  ; In practice we are in mode=2 when this happens
- ldh a, (<STAT)
  and $04
  jr z, -

  ; Wait until mode = 0
  ; At this point we have gone past mode=3
  ; while having LY=LYC
- ldh a, (<STAT)
  and $03
  jr nz, -

  inc b
  jr ly_iteration

finish_round2:
  di
  test_ok

fail_round2:
  save_results
  test_failure_dump

.org INTR_VEC_STAT
  jp hl
