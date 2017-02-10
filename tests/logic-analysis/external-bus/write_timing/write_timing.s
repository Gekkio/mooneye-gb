; Copyright (C) 2014-2017 Joonas Javanainen <joonas.javanainen@gmail.com>
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

.macro test
  ld (\1), a
.endm

  di
  wait_vblank
  disable_lcd

  xor a
  test $3FFF
  test $7FFF
  test $9FFF ; not visible
  test $BFFF
  test $DFFF
  test $FDFF
  test $FE9F
  test $FEFF

  ; data not visible
  test $FF00
  test $FF01
  test $FF04
  test $FF0F
  test $FF1F
  test $FF2F
  test $FF3F
  test $FF4F
  test $FF5F
  test $FF6F
  test $FF7F
  test $FFFE
  test $FFFF

- halt
  nop
  jr -
