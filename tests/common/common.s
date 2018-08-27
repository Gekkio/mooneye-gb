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

.include "hardware.s"
.include "macros.s"

; --- Cartridge configuration ---

.ifndef CART_TYPE
  .define CART_TYPE 0
.endif
.ifndef CART_ROM_BANKS
  .define CART_ROM_BANKS 2
.endif
.ifndef CART_RAM_SIZE
  .define CART_RAM_SIZE 0
.endif

.rombanksize $4000
.rombanks CART_ROM_BANKS

.emptyfill $FF

.ifdef CART_CGB
  .romgbc
.else
  .ifdef CART_SGB
    .romsgb
  .else
    .romdmg
  .endif
.endif

.ifndef CART_NO_TITLE
  .name "mooneye-gb test"
.endif

.licenseecodenew "ZZ"
.cartridgetype CART_TYPE
.ramsize CART_RAM_SIZE
.countrycode $01
.nintendologo
.version $00

.ifndef CART_NO_GLOBAL_CHECKSUM
  .computegbchecksum
.endif
.computegbcomplementcheck

; --- Library functions ---

.bank 1 slot 1
.include "lib/clear_oam.s"
.include "lib/clear_vram.s"
.include "lib/clear_wram.s"
.include "lib/disable_lcd_safe.s"
.include "lib/memcmp.s"
.include "lib/memcpy.s"
.include "lib/memset.s"
.include "lib/print_bin1.s"
.include "lib/print_bin4.s"
.include "lib/print_bin8.s"
.include "lib/print_hex4.s"
.include "lib/print_hex8.s"
.include "lib/print_inline_string.s"
.include "lib/print_load_font.s"
.include "lib/print_newline.s"
.include "lib/print_reg_dump.s"
.include "lib/print_string.s"
.include "lib/quit_dump_mem.s"
.include "lib/reset_screen.s"

; --- Cartridge header ---

.bank 0 slot 0
.org $100
.section "Header" force
  nop
  jp $150
.ends

; --- Runtime ---

.bank 1 slot 1
.section "Font" free
font:
  ; 8x8 ASCII bitmap font by Darkrose
  ; http://opengameart.org/content/8x8-ascii-bitmap-font-with-c-source
  .incbin "font.bin" fsize FONT_SIZE
.ends

.struct reg_dump
  f db
  a db
  c db
  b db
  e db
  d db
  l db
  h db
.endst

.ramsection "Runtime-State" slot 2
  regs_save instanceof reg_dump
  regs_flags db
  regs_assert instanceof reg_dump
.ends

.bank 1 slot 1
.section "Runtime" FREE
  process_results:
    print_results _process_results_cb
  _process_results_cb:
    ld de, regs_save
    print_string_literal "REGISTERS"
    call print_newline
    call print_newline
    call print_reg_dump
    call print_newline

    ld a, (regs_flags)
    or a
    jr z, +
    print_string_literal "ASSERTIONS"
    call print_newline
    call print_newline
    call _check_asserts

    ld a, d
    or a
    jr z, +
    call print_newline
    print_string_literal "TEST FAILED"
+   ret

  _check_asserts:
    xor a
    ld d, a

    ld a, (regs_flags)
    ld e, a

    .macro __check_assert ARGS flag str value expected
      bit flag, e
      jr z, __check_assert_skip\@

      print_string_literal str
      print_string_literal ": "

      ld a, (value)
      ld c, a
      ld a, (expected)
      cp c
      jr z, __check_assert_ok\@
    __check_assert_fail\@:
      call print_hex8
      print_string_literal "! "
      inc d
      jr __check_assert_out\@
    __check_assert_ok\@:
      print_string_literal "OK  "
      jr __check_assert_out\@
    __check_assert_skip\@:
      print_string_literal "       "
    __check_assert_out\@:
    .endm

    print_string_literal "  "
    __check_assert 0 "A" regs_save.a regs_assert.a
    __check_assert 1 "F" regs_save.f regs_assert.f
    call print_newline
    print_string_literal "  "
    __check_assert 2 "B" regs_save.b regs_assert.b
    __check_assert 3 "C" regs_save.c regs_assert.c
    call print_newline
    print_string_literal "  "
    __check_assert 4 "D" regs_save.d regs_assert.d
    __check_assert 5 "E" regs_save.e regs_assert.e
    call print_newline
    print_string_literal "  "
    __check_assert 6 "H" regs_save.h regs_assert.h
    __check_assert 7 "L" regs_save.l regs_assert.l
    call print_newline

    ret
.ends

.bank 0 slot 0
.org $150
