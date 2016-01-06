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

; Tests the initial values of hardware IO registers that are not
; affected by the test itself, timing, or random factors.
; Therefore, we skip $FF04 (DIV), $FF30-$FF3F (wave ram),
; $FF40 (LCDC) and $FF41 (STAT)

; Verified results:
;   pass: SGB, SGB2
;   fail: DMG, MGB, CGB, AGB, AGS

.incdir "../common"
.include "common.s"

  ld hl, $FF00
  ld de, hwio_data

--
  ld a, (de)
  ld c, a
  inc de

.repeat 8 INDEX i
  ld a, (hl)
  ld b, a

  bit 7 - i, c
  jr z, +

  ld a, (de)
  cp b
  jp nz, mismatch
+ inc de
  inc hl
.endr

  ld a, l
  cp $80
  jp nz, --

  test_ok

mismatch:
  ld (mismatch_data), a
  ld a, b
  ld (mismatch_mem), a
  ld a, l
  ld (mismatch_addr), a
  ld a, h
  ld (mismatch_addr + 1), a
  print_results mismatch_cb
mismatch_cb
  print_string_literal "MISMATCH AT $"
  ld a, (mismatch_addr + 1)
  call print_a
  ld a, (mismatch_addr)
  call print_a
  call print_newline
  call print_newline

  print_string_literal "EXPECTED "
  ld a, (mismatch_data)
  call print_a
  call print_newline

  print_string_literal "GOT      "
  ld a, (mismatch_mem)
  call print_a

  ld d, $42
  ret

hwio_data:
;   mask bits  values                                   address of first byte
;   |          |                                        |
.db %11110111, $FF, $00, $7E, $FF, $FF, $00, $00, $F8 ; $FF00
.db %11111111, $FF, $FF, $FF, $FF, $FF, $FF, $FF, $E1 ; $FF08
.db %11111111, $80, $BF, $F3, $FF, $BF, $FF, $3F, $00 ; $FF10
.db %11111111, $FF, $BF, $7F, $FF, $9F, $FF, $BF, $FF ; $FF18
.db %11111111, $FF, $00, $00, $BF, $77, $F3, $F0, $FF ; $FF20
.db %11111111, $FF, $FF, $FF, $FF, $FF, $FF, $FF, $FF ; $FF28
.db %00000000, $FF, $FF, $FF, $FF, $FF, $FF, $FF, $FF ; $FF30
.db %00000000, $FF, $FF, $FF, $FF, $FF, $FF, $FF, $FF ; $FF38
.db %00110111, $FF, $FF, $00, $00, $FF, $00, $FF, $FC ; $FF40
.db %11111111, $FF, $FF, $00, $00, $FF, $FF, $FF, $FF ; $FF48
.db %11111111, $FF, $FF, $FF, $FF, $FF, $FF, $FF, $FF ; $FF50
.db %11111111, $FF, $FF, $FF, $FF, $FF, $FF, $FF, $FF ; $FF58
.db %11111111, $FF, $FF, $FF, $FF, $FF, $FF, $FF, $FF ; $FF60
.db %11111111, $FF, $FF, $FF, $FF, $FF, $FF, $FF, $FF ; $FF68
.db %11111111, $FF, $FF, $FF, $FF, $FF, $FF, $FF, $FF ; $FF70
.db %11111111, $FF, $FF, $FF, $FF, $FF, $FF, $FF, $FF ; $FF78

.ramsection "Test-State" slot 2
  mismatch_addr dw
  mismatch_data db
  mismatch_mem db
.ends
