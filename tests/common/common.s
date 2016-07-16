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

.include "hardware.s"

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
  ; = 4 cycles

_delay_long_time_\@:
  dec bc
  ld a,b
  or c
  jr nz, _delay_long_time_\@
  ; = iterations * 7 - 1 cycles

  ; total: iterations * 7 + 3 cycles
.endm


.macro halt_execution
  .ifdef ACCEPTANCE_TEST
    .db $ED ; Magic undefined opcode
  .endif
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

; BC = BC - DE
.macro sub16
  ld a, c
  sub e
  ld c, a

  ld a, b
  sbc d
  ld b, a
.endm

.macro wait_ly ARGS value
_wait_ly_\@:
  ldh a, (<LY)
  cp value
  jr nz, _wait_ly_\@
.endm

.macro wait_vblank
  ; wait for LY=143 first to ensure we get a fresh v-blank
  wait_ly $89
  ; wait for LY=144
  wait_ly $90
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
  ld hl, $FF80
  ld de, hiram_test
  ld bc, $60 ; 0x60 bytes should be enough
  call memcpy
  ; jump to test procedure in hiram
  jp $FF80
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
  call print_regs
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
  enable_lcd
  wait_vblank
  disable_lcd
  call reset_screen
  call print_load_font

  ld hl, $9820
  call cb

  enable_lcd
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
.ifndef CART_CGB
  .define CART_CGB 0
.endif

.rombanksize $4000
.rombanks CART_ROM_BANKS

.emptyfill $FF
.cartridgetype CART_TYPE
.ramsize CART_RAM_SIZE

.ifeq CART_CGB 1
  .romgbc
.else
  .romdmg
.endif

.countrycode $01
.licenseecodenew "ZZ"

.name "mooneye-gb test"
.computegbcomplementcheck
.computegbchecksum

.nintendologo

; --- Cartridge header ---

.bank 0 slot 0
.org $100
.section "Header" force
  nop
  jp $150
.ends

.org $14C
.section "Header-Extra" force
  .db $00 ; ROM version
.ends

; --- Runtime ---

.include "print.s"

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
  memdump_len db
  memdump_addr dw
.ends

.bank 1 slot 1
.section "Runtime" FREE
  ; Inputs:
  ;   HL target
  ;   DE source
  ;   BC number of bytes
  ; Clobbers:
  ;   AF, BC, DE, HL
  memcpy:
-   ld a, (de)
    ld (hl+), a
    inc de
    dec bc
    ld a,b
    or c
    jr nz, -
    ret

  ; Inputs:
  ;   HL target
  ;   A value
  ;   BC number of bytes
  ; Clobbers:
  ;   AF, BC, HL
  memset:
    ld d, a
-   ld a, d
    ld (hl+), a
    dec bc
    ld a, b
    or c
    jr nz, -
    ret

  ; Clobbers:
  ;   AF, BC, DE, HL
  clear_vram:
    ld hl, VRAM
    ld bc, $2000
    xor a
    call memset
    ret

  reset_screen:
    xor a
    ldh (<SCY), a
    ldh (<SCX), a

.ifeq CART_CGB 1
    ld a, $82
    ldh (<BCPS), a
    xor a
  .repeat 6
    ldh (<BCPD), a
  .endr
.else
    ld a, $FC
    ldh (<BGP), a
.endif

    call clear_vram
    ret

  process_results:
    print_results _process_results_cb
  _process_results_cb:
    ld de, regs_save
    print_string_literal "REGISTERS"
    call print_newline
    call print_newline
    call print_regs
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

  dump_mem:
    ld (memdump_len), a
    ld a, l
    ld (memdump_addr), a
    ld a, h
    ld (memdump_addr + 1), a

    enable_lcd
    wait_vblank
    disable_lcd
    call reset_screen
    call print_load_font

    ld hl, $9800

    ld a, (memdump_addr + 1)
    ld d, a
    ld a, (memdump_addr)
    ld e, a
  _dump_mem_line:
    ld a, d
    call print_a
    ld a, e
    call print_a

-   ld a, (de)
    call print_a
    inc de
    ld a, (memdump_len)
    dec a
    jr z, +
    ld (memdump_len), a
    ld a, l
    and $1F
    cp 20
    jr nz, -
    call print_newline
    jr _dump_mem_line

+
    enable_lcd
    halt_execution

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
      call print_a
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
