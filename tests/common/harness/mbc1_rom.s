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

.define CART_TYPE 1 ; MBC1

.include "common.s"

.define g_bank_number    $FF80
.define g_actual_value   $FF81
.define g_expected_value $FF82
.define g_lower_upper    $FF83
.define g_mode           $FF84

.macro call_c000 ARGS target
  call target - c000_functions_start + $C000
.endm

.macro jp_c000 ARGS target
  jp target - c000_functions_start + $C000
.endm

  ld hl, $C000
  ld de, c000_functions_start
  ld bc, c000_functions_end - c000_functions_start
  call memcpy

  ld hl, $D000
  ld de, expected_banks
  ld bc, 128 + 128 + 128
  call memcpy

  jp $C000

fail:
  print_results _fail_cb
_fail_cb:
  print_string_literal "TEST FAILED"
  call print_newline
  ldh a, (<g_lower_upper)
  and a
  jr nz, +

  print_string_literal "$0000-$3FFF"
  jr ++

+ print_string_literal "$4000-$7FFF"

++
  call print_newline

  print_string_literal "MODE "
  ldh a, (<g_mode)
  call print_hex8

  call print_newline
  call print_newline

  print_string_literal "BANK NUMBER "
  ldh a, (<g_bank_number)
  call print_hex8
  call print_newline

  print_string_literal "EXPECTED    "
  ldh a, (<g_expected_value)
  call print_hex8
  call print_newline

  print_string_literal "ACTUAL      "
  ldh a, (<g_actual_value)
  call print_hex8
  call print_newline
  ld d, $42
  ret


c000_functions_start:

run_test_suite:
  xor a
  ld ($6000), a
  ldh (<g_mode), a
  call_c000 run_tests

  ld a, $01
  ld ($6000), a
  ldh (<g_mode), a
  call_c000 run_tests

  call_c000 restore_mbc1
  test_ok

run_tests:
  xor a
  ldh (<g_lower_upper), a
  call_c000 run_test_cases

  ld a, $40
  ldh (<g_lower_upper), a
  call_c000 run_test_cases

  ret

run_test_cases:
  xor a
--
  ldh (<g_bank_number), a

  call_c000 test_case
  ldh a, (<g_bank_number)
  inc a
  cp 128
  jr nz, --

  ret

; Inputs:
;   DE: address to check ($0000 or $4000)
test_case:
  ldh a, (<g_bank_number)
  call_c000 switch_bank

  ldh a, (<g_bank_number)
  call_c000 fetch_expected_value
  ldh (<g_expected_value), a
  ld b, a

  ldh a, (<g_lower_upper)
  ld d, a
  ld e, $00
  ld a, (de)
  cp b
  ret z

  ldh (<g_actual_value), a

  call_c000 restore_mbc1
  jp fail

; Inputs: -
; Preserved: BC, DE, HL
restore_mbc1:
  xor a
  ld ($6000), a
  ld a, 1
  jp_c000 switch_bank

; Inputs:
;   A: bank number
; Preserved: BC, DE, HL
switch_bank:
  push af
  or %11100000 ; set high bits to expose bugs
  ld ($2000), a
  pop af
  swap a
  sra a
  or %11111100 ; set high bits to expose bugs
  ld ($4000), a
  ret

; Inputs:
;   A: bank number
; Preserved: DE
fetch_expected_value:
  ld b, $00
  ld c, a
  ld hl, $D000
  add hl, bc

  ldh a, (<g_lower_upper)
  and a
  jr z, +

  ld bc, 256
  add hl, bc
  jr ++

+ ldh a, (<g_mode)
  and a
  jr z, ++

  ld bc, 128
  add hl, bc
  jr ++

++
  ld a, (hl)
  ret

c000_functions_end:


.repeat CART_ROM_BANKS INDEX bank

.bank bank
.org $0000
.db bank

.endr
