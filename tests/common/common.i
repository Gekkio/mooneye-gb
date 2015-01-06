;

.memorymap
  defaultslot 0
  slotsize $4000
  slot 0 $0000
  slot 1 $4000
  slot 2 $C000
.endme

.rombanksize $4000
.rombanks 2

.emptyfill $FF
.cartridgetype 0
.ramsize 0
.romdmg
.name "mooneye-gb test"

.bank 0

.org $100
  nop
  jp $150

  ; Nintendo logo
  .db $CE, $ED, $66, $66, $CC, $0D, $00, $0B
  .db $03, $73, $00, $83, $00, $0C, $00, $0D
  .db $00, $08, $11, $1F, $88, $89, $00, $0E
  .db $DC, $CC, $6E, $E6, $DD, $DD, $D9, $99
  .db $BB, $BB, $67, $63, $6E, $0E, $EC, $CC
  .db $DD, $DC, $99, $9F, $BB, $B9, $33, $3E

.org $14A
  .db $00 ; Destination code: 00 - Japanese
  .db $00 ; Licensee code
  .db $00 ; ROM version

.computegbcomplementcheck
.computegbchecksum

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

.macro nops ARGS count
  .dsb count, $00
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

.macro wait_vblank
  ; wait for LY=143 first to ensure we get a fresh v-blank
  wait_ly $89
  ; wait for LY=144
  wait_ly $90
.endm

.macro wait_ly ARGS value
- ld_a_ff LY
  cp value
  jr nz, -
.endm

.macro disable_lcd
  ld hl, LCDC
  res 7, (HL)
.endm

.macro enable_lcd
  ld hl, LCDC
  set 7, (HL)
.endm

.macro ld_a_ff ARGS addr
  ldh a, (addr - $FF00)
.endm

.macro ld_ff_a ARGS addr
  ldh (addr - $FF00), a
.endm

.org $1000

.macro save_results
  di
  ld sp, $d908
  push hl
  push de
  push bc
  push af
  xor a
  ld b, $08
  ld hl, $d000
- ld (HL+), a
  dec b
  jr nz, -
  ld b, a
.endm

.macro assert_a ARGS value
  ld a, value
  ld ($d001), a
  set 0, b
.endm
.macro assert_f ARGS value
  ld a, value
  ld ($d000), a
  set 1, b
.endm
.macro assert_b ARGS value
  ld a, value
  ld ($d003), a
  set 2, b
.endm
.macro assert_c ARGS value
  ld a, value
  ld ($d002), a
  set 3, b
.endm
.macro assert_d ARGS value
  ld a, value
  ld ($d005), a
  set 4, b
.endm
.macro assert_e ARGS value
  ld a, value
  ld ($d004), a
  set 5, b
.endm
.macro assert_h ARGS value
  ld a, value
  ld ($d007), a
  set 6, b
.endm
.macro assert_l ARGS value
  ld a, value
  ld ($d006), a
  set 7, b
.endm

print_results:
  ld a, b
  ld ($d008), a
  ld sp, $dfff
  wait_vblank
  disable_lcd
  call reset_screen
  call load_font
  call print_regs
  call check_asserts
  enable_lcd
  wait_vblank
  wait_vblank
  ld a, e
  debug
- nop
  jr -

print_digit:
  ld a, $0F
  and b
  cp $0a
  jr c, +
  add $07
+ add $30
  ld (HL+),a
  ret

print_a:
  ld b, a
  swap b
  call print_digit
  swap b
  call print_digit
  ret

.macro print_char ARGS char
  ld a, char
  ld (HL+), a
.endm

.macro check_assert ARGS char value expected
  print_char char
  print_char ':'
  print_char ' '

  ld a, (expected)
  ld c, a
  ld a, (value)
  cp c
  jr z, +
  ld a, c
  call print_a
  print_char '!'
  ld a, $01
  jr ++
+ print_char 'O'
  print_char 'K'
  print_char ' '
  xor a
++
  ld (expected), a
  ld bc, 26
  add hl, bc
  ret
.endm

check_assert_a:
  check_assert 'A' $d901 $d001
check_assert_f:
  check_assert 'F' $d900 $d000
check_assert_b:
  check_assert 'B' $d903 $d003
check_assert_c:
  check_assert 'C' $d902 $d002
check_assert_d:
  check_assert 'D' $d905 $d005
check_assert_e:
  check_assert 'E' $d904 $d004
check_assert_h:
  check_assert 'H' $d907 $d007
check_assert_l:
  check_assert 'L' $d906 $d006

check_asserts:
  ld bc, 64
  add hl, bc
  ld a, ($d008)
  ld e, a
  bit 0, e
  call nz, check_assert_a
  bit 1, e
  call nz, check_assert_f
  bit 2, e
  call nz, check_assert_b
  bit 3, e
  call nz, check_assert_c
  bit 4, e
  call nz, check_assert_d
  bit 5, e
  call nz, check_assert_e
  bit 6, e
  call nz, check_assert_h
  bit 7, e
  call nz, check_assert_l

  xor a
  ld e, a

  ld bc, $d008
- dec bc
  ld a, (bc)
  add e
  ld e, a
  ld a, c
  or a
  jr nz, -

  ret

.macro print_reg ARGS char addr
  print_char char
  print_char ':'
  print_char ' '
  ld a, (addr)
  call print_a
  print_char ' '
.endm

print_regs:
  ld hl, $9820
  print_reg 'A' $d901
  print_reg 'F' $d900
  ld bc, 20
  add hl, bc
  print_reg 'B' $d903
  print_reg 'C' $d902
  ld bc, 20
  add hl, bc
  print_reg 'D' $d905
  print_reg 'E' $d904
  ld bc, 20
  add hl, bc
  print_reg 'H' $d907
  print_reg 'L' $d906
  ld bc, 20
  add hl, bc
  ret

.macro test_failure
  ld sp, $fffe
  call print_failure
.endm

print_failure:
  wait_vblank
  disable_lcd
  call reset_screen
  call load_font
  ld hl, $9820
  print_char 'F'
  print_char 'A'
  print_char 'I'
  print_char 'L'
  print_char 'E'
  print_char 'D'
  ld bc, 26
  add hl, bc
  print_char 'P'
  print_char 'C'
  print_char ':'
  print_char ' '

  ; load PC
  ld sp, $fffe - 2
  pop bc
  ; subtract $06 to get the PC value where test_failure was called
  ld de, $0006
  sub16

  ld a, b
  call print_a
  ld a, c
  call print_a
  enable_lcd
- nop
  jr -

.define NUM_CHARS 128
.define CHAR_BYTES 16

load_font:
  ld hl, $8010
  ld de, $2000
  ld bc, NUM_CHARS * CHAR_BYTES  
  call memcpy
  ret

; HL target
; DE source
; BC number of bytes
memcpy:
- ld a, (de)
  ld (hl+), a
  inc de
  dec bc
  ld a,b
  or c
  jr nz, -
  ret

; HL target
; BC number of bytes
; A value
memset:
  ld d, a
- ld a, d
  ld (hl+), a
  dec bc
  ld a, b
  or c
  jr nz, -
  ret

reset_screen:
  xor a
  ld ($ff42), a
  ld ($ff43), a
  ld hl, VRAM
  ld bc, $2000-1
- xor a
  ld (hl+), a
  dec bc
  ld a,b
  or c
  jr nz, -
  ret

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

.macro start_oam_dma ARGS value
  wait_vblank
  ld a, $80
  ld_ff_a DMA
.endm

.org $2000
font:
  ; 8x8 ASCII bitmap font by Darkrose
  ; http://opengameart.org/content/8x8-ascii-bitmap-font-with-c-source
  .incbin "font.bin"

.bank 0
.org $150
