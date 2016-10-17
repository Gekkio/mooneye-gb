; Copyright (C) 2014-2016 Joonas Javanainen <joonas.javanainen@gmail.com>
;
; Permission is hereby granted, free of charge, to any person obtaining a copy
; of this software and associated documentation files (the "Software"), to deal
; in the Software without restriction, including without limitation the rights
; to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
; copies of the Software, and to permit persons to whom the Software is
; furnished to do so, subject to the following conditions:
;
; The above copyright notice and this permission notice shall be included in
; all copies or substantial portions of the Software.
;
; THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
; IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
; FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
; AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
; LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
; OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
; SOFTWARE.

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
