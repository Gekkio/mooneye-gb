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

.incdir "../../../common"
.include "common.s"

.define TEST_SCX $00

  di
  wait_vblank
  disable_lcd
  call reset_screen

  ; BGP should use light grey for colors 1-3
  ld a, %01010100
  ld (BGP), a

  ; Make tile $03 solid with color 0
  ld hl, $8000 + $03 * 16
  ld bc, 16
  ld a, $00
  call memset

  ; Make tile $07 solid with color 3
  ld hl, $8000 + $07 * 16
  ld bc, 16
  ld a, $FF
  call memset

  ; Copy checkerboard to VRAM
  ld hl, $9800
  ld de, vram_checkerboard
  ld bc, vram_checkerboard_end - vram_checkerboard
  call memcpy

  enable_lcd

  ld a, INTR_STAT
  ldh (<IE), a
  ld a, %01000000
  ldh (<STAT), a

  ld a, TEST_SCX
  ldh (<SCX), a

  ld a, $08
  ldh (<LYC), a

  xor a
  ldh (<IF), a
  ei

- halt
  nop
  ld a, %00001000
  ldh (<STAT), a
  halt
  nop
  ld a, %01000000
  ldh (<STAT), a
  jr -

vram_checkerboard:
  .repeat 16
    .repeat 16
      .db $03 $07
    .endr
    .repeat 16
      .db $07 $03
    .endr
  .endr
vram_checkerboard_end:

.org INTR_VEC_STAT
  reti
