; Copyright (C) 2014-2018 Joonas Javanainen <joonas.javanainen@gmail.com>
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

.section "quit_dump_mem"
; Inputs:
;   A: number of bytes
;   HL: source address
; Outputs: -
; Preserved: -
quit_dump_mem:
  push af
  push hl
  call disable_lcd_safe
  call reset_screen
  call print_load_font

  ld hl, $9800
  pop de
  pop bc
@line:
  ld a, d
  call print_a
  ld a, e
  call print_a

- ld a, (de)
  call print_a
  inc de
  dec b
  jr z, +

  ld a, l
  and $1f
  cp 20
  jr nz, -

  call print_newline
  jr @line

+
  enable_lcd
  halt_execution
.ends
