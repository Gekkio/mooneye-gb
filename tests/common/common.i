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

.define DIV $FF04
.export DIV

.macro nops ARGS count
  .dsb count, $00
.endm

.macro wait_vblank
- ld a, ($FF00+$44)
  cp $90
  jr nz, -
.endm

.macro disable_lcd
  ld hl, $FF40
  res 7, (HL)
.endm

.macro enable_lcd
  ld hl, $FF40
  set 7, (HL)
.endm

.org $1000

finish:
  ; .db $ED
  di
  ld sp, $d008
  push hl
  push de
  push bc
  push af
  ld sp, $dfff
  wait_vblank
  disable_lcd
  call reset_screen
  call load_font
  call print_regs
  enable_lcd
- nop
  jr -

.macro print_reg ARGS char addr
  ld a, char
  ld (HL+), a
  ld a, ':'
  ld (HL+), a
  ld a, ' '
  ld (HL+), a
  ld a, (addr)
  call print_a
  ld a, ' '
  ld (HL+), a
.endm

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

print_regs:
  ld hl, $9820
  print_reg 'A' $d001
  print_reg 'F' $d000
  ld bc, 20
  add hl, bc
  print_reg 'B' $d003
  print_reg 'C' $d002
  ld bc, 20
  add hl, bc
  print_reg 'D' $d005
  print_reg 'E' $d004
  ld bc, 20
  add hl, bc
  print_reg 'H' $d007
  print_reg 'L' $d006
  ret

.define NUM_CHARS 128
.define CHAR_BYTES 16

load_font:
  ld hl, $8010
  ld de, $2000
  ld bc, NUM_CHARS * CHAR_BYTES  
- ld a, (de)
  ld (hl+), a
  inc de
  dec bc
  ld a,b
  or c
  jr nz, -
  ret

reset_screen:
  xor a
  ld ($ff42), a
  ld ($ff43), a
  ld hl, $8000
  ld bc, $2000-1
- xor a
  ld (hl+), a
  dec bc
  ld a,b
  or c
  jr nz, -
  ret

.org $2000
font:
  ; 8x8 ASCII bitmap font by Darkrose
  ; http://opengameart.org/content/8x8-ascii-bitmap-font-with-c-source
  .incbin "font.bin"

.bank 0
.org $150
