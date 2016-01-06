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

; Tests initial register values

; Verified results:
;   pass: DMG
;   fail: MGB, SGB, SGB2, CGB, AGB, AGS

.incdir "../common"
.include "common.s"

; First, let's check SP since it's not part of the normal save_results
; mechanism
.define EXPECTED_SP $FFFE

  ld (sp_save), sp
  ld sp, $FFFE

  push af

  ld a, (sp_save)
  cp <EXPECTED_SP
  jr nz, invalid_sp

  ld a, (sp_save+1)
  cp >EXPECTED_SP
  jr nz, invalid_sp

  pop af

; Now, let's check all the other registers

  save_results
  assert_a $01
  assert_f $B0
  assert_b $00
  assert_c $13
  assert_d $00
  assert_e $D8
  assert_h $01
  assert_l $4D
  jp process_results

invalid_sp:
  test_failure_string "INVALID SP VALUE"

.ramsection "Test-State" slot 2
  sp_save dw
.ends
