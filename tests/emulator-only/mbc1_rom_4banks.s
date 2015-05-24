.define CART_TYPE 1 ; MBC1
.define CART_ROM_BANKS 4

.incdir "../common"
.include "common.i"

.macro request_bank ARGS bank
  ld a, bank
  ld ($2000), a
  ld a, bank >> 5
  ld ($4000), a
.endm

.macro expect_value ARGS value
  ld a, ($4000)
  cp value
  jr z, +
  test_failure
+ nop
.endm

  di

  request_bank 0
  expect_value $01 ; MBC1 quirk
  request_bank 1
  expect_value $01
  request_bank 2
  expect_value $02
  request_bank 3
  expect_value $03

  request_bank 4
  expect_value $00 ; Wrap
  request_bank 5
  expect_value $01

  request_bank 32
  expect_value $01 ; MBC1 quirk
  request_bank 33
  expect_value $01

  request_bank 36
  expect_value $00 ; Wrap

  save_results
  call print_results

.bank 0
.org $0000
.db $00

.bank 1
.org $0000
.db $01

.bank 2
.org $0000
.db $02

.bank 3
.org $0000
.db $03
