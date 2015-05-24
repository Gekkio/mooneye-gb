.memorymap
  defaultslot 1
  slot 0 start $0000 size $4000
  slot 1 start $4000 size $4000
  slot 2 start $C000 size $1000
  slot 3 start $D000 size $1000
  slot 4 start $A000 size $1000
.endme

.define VRAM $8000
.define OAM  $FE00

.define P1   $FF00
.define SB   $FF01
.define SC   $FF02
.define DIV  $FF04
.define TIMA $FF05
.define TMA  $FF06
.define TAC  $FF07
.define IF   $FF0F
.define LCDC $FF40
.define STAT $FF41
.define SCY  $FF42
.define SCX  $FF43
.define LY   $FF44
.define LYC  $FF45
.define DMA  $FF46
.define BGP  $FF47
.define OBP0 $FF48
.define OBP1 $FF49
.define WY   $FF4A
.define WX   $FF4B
.define IE   $FFFF

.macro ld_a_ff ARGS addr
  ldh a, (addr - $FF00)
.endm

.macro ld_ff_a ARGS addr
  ldh (addr - $FF00), a
.endm

.macro c_string ARGS string
  .db string, $00
.endm

; --- Macros ---

.macro nops ARGS count
  .repeat count
    nop
  .endr
.endm

.macro halt_execution
  .ifdef ACCEPTANCE_TEST
    debug
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
wait_ly_\@:
  ld_a_ff LY
  cp value
  jr nz, wait_ly_\@
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
  ld_ff_a DMA
.endm

.macro test_failure
  test_failure_string "TEST FAILED"
.endm

.macro test_failure_string ARGS string
  print_results test_failure_cb_\@
test_failure_cb_\@:
  print_string_literal string
  ld d, $42
  ret
.endm

.macro test_ok
  print_results test_ok_cb_\@
test_ok_cb_\@:
  print_string_literal "TEST OK"
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
  jr nz, +
  ; Magic numbers signal a successful test
  ld b, 3
  ld c, 5
  ld d, 8
  ld e, 13
  ld h, 21
  ld l, 34
+ halt_execution
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

.rombanksize $4000
.rombanks CART_ROM_BANKS

.emptyfill $FF
.cartridgetype CART_TYPE
.ramsize CART_RAM_SIZE
.romdmg
.name "mooneye-gb test"
.computegbcomplementcheck
.computegbchecksum

; --- Cartridge header ---

.bank 0 slot 0
.org $100
.section "Header" force
  nop
  jp $150

  ; Nintendo logo
  .db $CE, $ED, $66, $66, $CC, $0D, $00, $0B
  .db $03, $73, $00, $83, $00, $0C, $00, $0D
  .db $00, $08, $11, $1F, $88, $89, $00, $0E
  .db $DC, $CC, $6E, $E6, $DD, $DD, $D9, $99
  .db $BB, $BB, $67, $63, $6E, $0E, $EC, $CC
  .db $DD, $DC, $99, $9F, $BB, $B9, $33, $3E
.ends

.org $14A
.section "Header-Extra" force
  .db $00 ; Destination code: 00 - Japanese
  .db $00 ; Licensee code
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
    ld_ff_a SCY
    ld_ff_a SCX
    ld a, $FC
    ld_ff_a BGP

    call clear_vram
    ret

  process_results:
    print_results process_results_cb
  process_results_cb:
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
    call check_asserts

    ld a, d
    or a
    jr z, +
    call print_newline
    print_string_literal "TEST FAILED"
+   ret

  check_asserts:
    xor a
    ld d, a

    ld a, (regs_flags)
    ld e, a

    .macro check_assert ARGS flag str value expected
      bit flag, e
      jr z, check_assert_skip\@

      print_string_literal str
      print_string_literal ": "

      ld a, (value)
      ld c, a
      ld a, (expected)
      cp c
      jr z, check_assert_ok\@
    check_assert_fail\@:
      call print_a
      print_string_literal "! "
      inc d
      jr check_assert_out\@
    check_assert_ok\@:
      print_string_literal "OK  "
      jr check_assert_out\@
    check_assert_skip\@:
      print_string_literal "       "
    check_assert_out\@:
    .endm

    print_string_literal "  "
    check_assert 0 "A" regs_save.a regs_assert.a
    check_assert 1 "F" regs_save.f regs_assert.f
    call print_newline
    print_string_literal "  "
    check_assert 2 "B" regs_save.b regs_assert.b
    check_assert 3 "C" regs_save.c regs_assert.c
    call print_newline
    print_string_literal "  "
    check_assert 4 "D" regs_save.d regs_assert.d
    check_assert 5 "E" regs_save.e regs_assert.e
    call print_newline
    print_string_literal "  "
    check_assert 6 "H" regs_save.h regs_assert.h
    check_assert 7 "L" regs_save.l regs_assert.l
    call print_newline

    ret
.ends

.bank 0 slot 0
.org $150
