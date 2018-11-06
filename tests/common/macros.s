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

; --- Macros ---

.macro c_string ARGS string
  .db string, $00
.endm

.macro nops ARGS count
  .repeat count
    nop
  .endr
.endm

.macro delay_long_time ARGS iterations
  ld a, >iterations
  ld b, a
  ld a, <iterations
  ld c, a
  ; = 6 cycles

_delay_long_time_\@:
  dec bc
  ld a,b
  or c
  jr nz, _delay_long_time_\@
  ; = iterations * 7 - 1 cycles

  ; total: iterations * 7 + 5 cycles
.endm


.macro halt_execution
  ld b, b ; magic breakpoint
- nop
  jr -
.endm

.macro disable_lcd
  ld hl, LCDC
  res 7, (HL)
.endm

.macro enable_lcd
  ld hl, LCDC
  set 7, (HL)
.endm

.macro wait_ly ARGS value
_wait_ly_\@:
  ldh a, (<LY)
  cp value
  jr nz, _wait_ly_\@
.endm

.macro wait_vblank
  ; wait for LY=143 first to ensure we get a fresh v-blank
  wait_ly 143
  ; wait for LY=144
  wait_ly 144
.endm

.macro save_results
  di
  ld sp, regs_save + 8
  push hl
  push de
  push bc
  push af
  ld sp, $fffe
  xor a
  ld hl, regs_flags
  ld (hl), a
  ld hl, regs_assert
  ld bc, 8
  call memset
.endm

.macro assert_a ARGS value
  ld a, value
  ld (regs_assert.a), a
  ld hl, regs_flags
  set 0, (hl)
.endm
.macro assert_f ARGS value
  ld a, value
  ld (regs_assert.f), a
  ld hl, regs_flags
  set 1, (hl)
.endm
.macro assert_b ARGS value
  ld a, value
  ld (regs_assert.b), a
  ld hl, regs_flags
  set 2, (hl)
.endm
.macro assert_c ARGS value
  ld a, value
  ld (regs_assert.c), a
  ld hl, regs_flags
  set 3, (hl)
.endm
.macro assert_d ARGS value
  ld a, value
  ld (regs_assert.d), a
  ld hl, regs_flags
  set 4, (hl)
.endm
.macro assert_e ARGS value
  ld a, value
  ld (regs_assert.e), a
  ld hl, regs_flags
  set 5, (hl)
.endm
.macro assert_h ARGS value
  ld a, value
  ld (regs_assert.h), a
  ld hl, regs_flags
  set 6, (hl)
.endm
.macro assert_l ARGS value
  ld a, value
  ld (regs_assert.l), a
  ld hl, regs_flags
  set 7, (hl)
.endm

; Copy test procedure to hiram $FF80 and jump to it.
; This is for tests that involve OAM DMA.
; During OAM DMA the CPU cannot access any other memory,
; so our code needs to be there
.macro run_hiram_test
  ld hl, HIRAM
  ld de, hiram_test
  ld bc, $60 ; 0x60 bytes should be enough
  call memcpy
  ; jump to test procedure in hiram
  jp HIRAM
.endm

.macro start_oam_dma ARGS address
  wait_vblank
  ld a, address
  ldh (<DMA), a
.endm

.macro test_failure
  test_failure_string "TEST FAILED"
.endm

.macro test_failure_dump ARGS string
  print_results _test_failure_dump_cb_\@
_test_failure_dump_cb_\@:
  ld de, regs_save
  print_string_literal "REGISTERS"
  call print_newline
  call print_newline
  call print_reg_dump
  call print_newline
  print_string_literal "TEST FAILED"
  ld d, $42
  ret
.endm

.macro test_failure_string ARGS string
  print_results _test_failure_cb_\@
_test_failure_cb_\@:
  print_string_literal string
  ld d, $42
  ret
.endm

.macro test_ok
  test_ok_string "TEST OK"
.endm

.macro test_ok_string ARGS string
  print_results _test_ok_cb_\@
_test_ok_cb_\@:
  print_string_literal string
  ld d, $00
  ret
.endm


.macro print_results ARGS cb
  di
  call disable_lcd_safe
  call reset_screen
  call print_load_font

  ld hl, $9820
  call cb

  enable_lcd
  wait_vblank
  ; Extra vblank to account for initial (invisible) frame
  wait_vblank
  ld a, d
  and a
  jr nz, _print_results_halt_\@
  ; Magic numbers signal a successful test
  ld b, 3
  ld c, 5
  ld d, 8
  ld e, 13
  ld h, 21
  ld l, 34
_print_results_halt_\@:
  halt_execution
.endm

.macro print_string_literal ARGS string
  call print_inline_string
  c_string string
.endm
