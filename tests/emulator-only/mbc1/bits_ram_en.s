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

; Tests that RAM_EN is mapped to correct addresses, and RAM disable/enable
; happens with the right data values.

; Results have been verified with MBC1 research

.define CART_TYPE 3 ; MBC1
.define CART_ROM_BANKS 4
.define CART_RAM_SIZE 2

.include "common.s"

  ld sp, $FFFE

  ; Enable RAM
  ld a, $0A
  ld ($0000), a

  ; Copy RAM data
  ld hl, $A000
  ld bc, _sizeof_ram_data_enabled
  ld de, ram_data_enabled
  call memcpy

test_round1
  ld hl, $1FFF

- ld a, l
  ld (test_address_l), a
  ld a, h
  ld (test_address_h), a
  push hl

  ; Disable RAM
  ld (hl), $00

  ld de, ram_data_disabled
  call compare_ram_data
  jp c, fail_round1_disable

  pop hl
  push hl

  ; Enable RAM
  ld (hl), $0A

  ld de, ram_data_enabled
  call compare_ram_data
  jp c, fail_round1_enable

  pop hl
  ld a, h
  or l
  dec hl
  jr nz, -

test_round2:
  xor a
  ld (ram_en_value), a

- ; Disable RAM
  xor a
  ld ($0000), a

  ld de, ram_data_disabled
  call compare_ram_data
  jp c, fail_round2_disable

  ld a, (ram_en_value)

  ; Write RAM_EN
  ld ($0000), a

  ld hl, ram_en_expectations
  add l
  ld l, a
  ld a, (hl)
  and a

  jr z, @expect_disabled
@expect_enabled:
  ld de, ram_data_enabled
  jr +
@expect_disabled:
  ld de, ram_data_disabled

+ call compare_ram_data
  jp c, fail_round2_expect

  ld a, (ram_en_value)
  inc a
  ld (ram_en_value), a
  jr nz, -

  test_ok

ram_data_enabled:
.db $19 $9D $91 $12 $F6 $12 $64 $4D $E4 $34 $3B $2E $FB $C7 $1F $3F
ram_data_disabled:
.dsb 16 $FF

; Inputs:
;   DE: ram data address
; Outputs:
;   cf 0 if data matched
compare_ram_data:
  ld hl, $A000
  ld bc, _sizeof_ram_data_enabled
  jp memcmp

fail_round1_disable:
  print_results @cb
fail_round1_disable@cb:
  print_string_literal "R1: Test failed"
  call print_newline
  call fail_round1_print_test_address
  call print_newline
  print_string_literal "RAM not disabled"
  ld d, $42
  ret

fail_round1_enable:
  print_results @cb
fail_round1_enable@cb:
  print_string_literal "R1: Test failed"
  call print_newline
  call fail_round1_print_test_address
  call print_newline
  print_string_literal "RAM not enabled"
  ld d, $42
  ret

fail_round1_print_test_address:
  ld a, (test_address_h)
  call print_a
  ld a, (test_address_l)
  call print_a
  ret

fail_round2_disable:
  test_failure_string "R2: RAM not disabled"

fail_round2_expect:
  print_results @cb
fail_round2_expect@cb:
  print_string_literal "R2: Test failed"
  call print_newline
  print_string_literal "RAM_EN="
  ld a, (ram_en_value)
  call print_a

  ld d, $42
  ret

.org $2000
ram_en_expectations:
.repeat 16
.db $00 $00 $00 $00 $00 $00 $00 $00 $00 $00 $FF $00 $00 $00 $00 $00
.endr

.ramsection "Test-State" slot 2
  test_address .dw
  test_address_l db
  test_address_h db
  ram_en_value db
.ends
