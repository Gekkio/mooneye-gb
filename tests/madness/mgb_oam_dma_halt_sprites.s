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

; This tests the weird behaviour when the CPU is halted when OAM DMA is
; active *and* the existing OAM data and new OAM data match certain patterns

; The basic idea is very simple:
; 1. Prepare OAM DMA (run in hiram)
; 2. Halt after the OAM DMA has started
; 3. Observe results
; No interrupts are enabled, so the CPU never wakes up, but the GPU is still
; drawing. However, since OAM DMA is in the middle of OAM access
; (but not proceeding with it!), the GPU will see weird values and does
; not render normally.

; The actual result depends on several things:
; - "magic values" that decide whether the GPU renders any sprites at all
; - the new OAM byte that was being written when the CPU clock halted
; - the existing byte that was supposed to be replaced when the CPU clock halted
; - the existing byte *after* the previous one

; Verified behaviour:
;   MGB: As described here and visualized by *_expected.png
;   DMG: A different sprite (probably different logic with the values)
;   CGB: Checkerboard without sprites (probably different logic with the values)
;   AGB/AGS: A different sprite (probably different logic with the values)

.incdir "../common"
.include "common.s"

  di
  wait_vblank
  disable_lcd
  call reset_screen
  call print_load_font

  ; OBP palette 0 should use only black
  ld a, $ff
  ld (OBP0), a
  ; OBP palette 1 should use only dark grey
  ld a, $AA
  ld (OBP1), a

  ; BGP should use light grey for colors 1-3
  ld a, $54
  ld (BGP), a

  ; Make tile $80 solid with color 3
  ld hl, $8000 + $80 * 16
  ld bc, 16
  ld a, $FF
  call memset

  ; Copy checkerboard to VRAM
  ld hl, $9800
  ld de, vram_checkerboard
  ld bc, vram_checkerboard_end - vram_checkerboard
  call memcpy

  ; Clear OAM
  ld hl, OAM
  ld bc, $a0
  xor a
  call memset

  ; Copy data to OAM
  ld hl, OAM
  ld de, initial_data
  ld bc, initial_data_end - initial_data
  call memcpy

  ; Enable sprites
  ld hl, LCDC
  set 1, (HL)

  enable_lcd
  wait_vblank
  run_hiram_test

hiram_test:
  ld hl, OAM
  ld b, 40
  start_oam_dma $20
  nop

  halt
  nop

vram_checkerboard:
  .repeat 16
    .repeat 16
      .db $00 $80
    .endr
    .repeat 16
      .db $80 $00
    .endr
  .endr
vram_checkerboard_end:

initial_data:
  ;   Y   X   C   F
  .db $FF $FF $30 $40
;             \------ These two values will affect how the sprite will be rendered

; In this test $30 is the byte that was supposed to be replaced, and $40 is the next byte.
; Only these bytes matter, and the Y and X values ($FF) are never used.

; In this case the sprite will be rendered as if this was the data:
; Y: ($30 | $1A) & $FC = $38  -> Y = 56
; X: $40 | $1A = $5A          -> X = 90
; C: ($30 | $1A) & $FC = $38  -> Sprite will be the character 8
; F: $40 | $1A = $5A          -> Above BG, horizontal flip, OBP1 palette

; Why & $FC? I have no idea, but it seems that the low two bits are always 0

  .db $9F $A7 $9F $A7
; This is the data that somehow enables sprite rendering. The position in OAM
; does not matter, and there can be more than one. As long as there is one properly
; aligned valid four-byte value, a sprite will be visible.
; The four values must be within these ranges:
;   $98 - $9F  =  152 - 159  <- ??? but 159 is the max visible Y value
;   $00 - $A7  =    0 - 167  <- this is the visible range for X values (yes, 0 is included)
;   $09 - $9F  =    9 - 159  <- ??? but 159 is the max visible Y value
;   $00 - $A7  =    0 - 167  <- this is the visible range for X values (yes, 0 is included)
; If any value is out of range, the data will have no effect.
initial_data_end:
  nop

.org $2000
  ;   Y   X   C   F
  .db $FF $FF $1A $FF
;             \-- this value affects how the sprite will be rendered

  .repeat 160-4
    .db $FF
  .endr
