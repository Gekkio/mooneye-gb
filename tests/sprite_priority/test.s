; *Manual test* for sprite priority
; See expected.png for a reference image
.incdir "../common"
.include "common.i"

  di
  wait_vblank
  disable_lcd
  call reset_screen
  call load_font

  ; OBP palette 0 should use only black
  ld a, $ff
  ld (OBP0), a
  ; OBP palette 1 should use only light grey
  ld a, $55
  ld (OBP1), a

  ; Clear OAM
  ld hl, OAM
  ld bc, $a0
  xor a
  call memset

  ; Copy data to OAM
  ld hl, OAM
  ld de, data
  ld bc, data_end - data
  call memcpy

  ; Enable sprites
  ld hl, LCDC
  set 1, (HL)

  enable_lcd
  halt_execution

data:
  ;    Y   X  CH   Flags: $00 uses OBP0, $10 uses OBP1
  ; Priority with same X coordinate
  .db  32  8  'O'  $10 ; Light grey should be on top
  .db  32  8  'O'  $00
  .db  32  8  'O'  $00
  .db  32  8  'O'  $00
  .db  32  8  'O'  $00
  .db  32  8  'O'  $00
  .db  32  8  'O'  $00
  .db  32  8  'O'  $00
  .db  32  8  'O'  $00
  .db  32  8  'O'  $00
  .db  32  8  $10  $00 ; 11th sprite should not be displayed

  ; Priority with different X coordinate
  .db  48  96 '9'  $00
  .db  48  88 '8'  $00
  .db  48  80 '7'  $00
  .db  48  72 '6'  $00
  .db  48  64 '5'  $00
  .db  48  56 '4'  $00
  .db  48  48 '3'  $00
  .db  48  40 '2'  $00
  .db  48  32 '1'  $00
  .db  48  24 '0'  $00
  .db  48  16 $10  $00 ; 11th sprite should not be displayed

  ; These overlap slightly with the earlier higher priority sprites,
  ; so in overlapping areas these sprites should not be drawn
  .db  52  96 '9'  $10
  .db  52  88 '8'  $10
  .db  52  80 '7'  $10
  .db  52  72 '6'  $10
  .db  52  64 '5'  $10
  .db  52  56 '4'  $10
  .db  52  48 '3'  $10
  .db  52  40 '2'  $10
  .db  52  32 '1'  $10
  .db  52  24 '0'  $10
  .db  52  16 $10  $10 ; 11th sprite should not be displayed

  ;           $10 = unprintable character = solid rectangle in the font
  ; Draw order is based on X coordinate, so in both following groups
  ; the black area should be bigger than the light grey
  .db  64  12 $10  $10
  .db  64  8  $10  $00
  .db  80  8  $10  $00
  .db  80  12 $10  $10
data_end:
  nop
