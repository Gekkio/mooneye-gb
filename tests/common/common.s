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
.include "lib/quit.s"
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
  reg_f db
  reg_a db
  reg_c db
  reg_b db
  reg_e db
  reg_d db
  reg_l db
  reg_h db
.endst

.ramsection "Runtime-State" slot 5
  v_regs_save instanceof reg_dump
  v_regs_flags db
  v_regs_assert instanceof reg_dump
.ends

.bank 1 slot 1
.section "Runtime" FREE

check_asserts_cb:
  ld de, v_regs_save
  print_string_literal "REGISTERS"
  call print_newline
  call print_newline
  call print_reg_dump
  call print_newline

  ldh a, (<v_regs_flags)
  or a
  jr z, +
  print_string_literal "ASSERTIONS"
  call print_newline
  call print_newline
  call @check_asserts

  ld a, d
  or a
  jr z, +
  call print_newline
  print_string_literal "TEST FAILED"
+ ret

  @check_asserts:
    xor a
    ld d, a

    ldh a, (<v_regs_flags)
    ld e, a

    .macro __check_assert ARGS flag str value expected
      bit flag, e
      jr z, __check_assert_skip\@

      print_string_literal str
      print_string_literal ": "

      ldh a, (<value)
      ld c, a
      ldh a, (<expected)
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
    __check_assert 0 "A" v_regs_save.reg_a v_regs_assert.reg_a
    __check_assert 1 "F" v_regs_save.reg_f v_regs_assert.reg_f
    call print_newline
    print_string_literal "  "
    __check_assert 2 "B" v_regs_save.reg_b v_regs_assert.reg_b
    __check_assert 3 "C" v_regs_save.reg_c v_regs_assert.reg_c
    call print_newline
    print_string_literal "  "
    __check_assert 4 "D" v_regs_save.reg_d v_regs_assert.reg_d
    __check_assert 5 "E" v_regs_save.reg_e v_regs_assert.reg_e
    call print_newline
    print_string_literal "  "
    __check_assert 6 "H" v_regs_save.reg_h v_regs_assert.reg_h
    __check_assert 7 "L" v_regs_save.reg_l v_regs_assert.reg_l
    call print_newline

    ret

.ends

.bank 0 slot 0
.org $150
