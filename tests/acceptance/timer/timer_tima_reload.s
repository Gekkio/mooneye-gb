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

; The test checks what values appear in the TIMA register when the
; timer overflows.
;
; Apparently the TIMA register contains 00 for 4 cycles before being
; reloaded with the value from the TMA register. The TIMA increments
; do still happen every 64 cycles, there is no additional 4 cycle
; delay.

; Verified results:
;   pass: DMG, MGB, SGB, SGB2, CGB, AGB, AGS
;   fail: -

.incdir "../../common"
.include "common.s"

test:
  di
  xor a
  ld b,$fe
  ldh (<IE), a
  ldh (<IF), a
  ldh (<DIV), a
  ld a, b
  ldh (<TIMA), a
  ldh (<TMA),a
  ld a, %00000110 ; Start 65536 Hz timer (64 cycles)
  ldh (<TAC), a
  ld a,b
  ldh (<DIV),a
  ldh (<TIMA), a
  ldh (<DIV),a
  nops 16
  nops 12
  ldh a,(<TIMA)
  ld d,a

  ld a,b
  ldh (<TIMA), a
  ldh (<DIV),a
  ldh (<TIMA), a
  ldh (<DIV),a
  nops 16
  nops 13
  ldh a,(<TIMA)
  ld e,a

  ld a,b
  ldh (<TIMA), a
  ldh (<DIV),a
  ldh (<TIMA), a
  ldh (<DIV),a
  nops 16
  nops 14
  ldh a,(<TIMA)
  ld c,a

  ld a,b
  ldh (<TIMA), a
  ldh (<DIV),a
  ldh (<TIMA), a
  ldh (<DIV),a
  nops 16
  nops 16
  nops 16
  nops 12
  ldh a,(<TIMA)
  ld h,a

  ld a,b
  ldh (<TIMA), a
  ldh (<DIV),a
  ldh (<TIMA), a
  ldh (<DIV),a
  nops 16
  nops 16
  nops 16
  nops 13
  ldh a,(<TIMA)
  ld l,a

  ld a,b
  ldh (<TIMA), a
  ldh (<DIV),a
  ldh (<TIMA), a
  ldh (<DIV),a
  nops 16
  nops 16
  nops 16
  nops 14
  ldh a,(<TIMA)
  ld b,a

  save_results
  assert_b $fe
  assert_c $fe
  assert_d $ff
  assert_e $00
  assert_h $ff
  assert_l $00
  jp process_results

