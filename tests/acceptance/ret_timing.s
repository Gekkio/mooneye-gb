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

; RET is expected to have the following timing:
; M = 0: instruction decoding
; M = 1: PC pop: memory access for low byte
; M = 2: PC pop: memory access for high byte
; M = 3: internal delay

; Verified results:
;   pass: DMG, MGB, SGB, SGB2, CGB, AGB, AGS
;   fail: -

.incdir "../common"
.include "common.s"

  ; Copy $FF80 callback
  ld hl, $FF80
  ld de, hiram_cb
  ld bc, $02
  call memcpy

test_round1:
  wait_vblank
  ld hl, OAM - 1
  ld a, $80
  ld (hl), a

  ld hl, VRAM
  ld a, $20
  ld (hl), a

  ld SP, OAM-1
  ld hl, finish_round1

  ld b, 39
  start_oam_dma $80  
- dec b
  jr nz, -
  nops 3

  ret

finish_round1:
  or a
  jr z, test_round2

  test_failure_string "FAIL: ROUND 1"

test_round2:
  wait_vblank
  ld hl, OAM
  ld a, $FF
  ld (hl), a

  ld SP, OAM-1
  ld hl, finish_round2

  ld b, 39
  start_oam_dma $80
- dec b
  jr nz, -
  nops 4

  ret

finish_round2:
  or a
  jr nz, test_success

  test_failure_string "FAIL: ROUND 2"

test_success:
  test_ok

hiram_cb:
  xor a
  jp hl

.org $2080
  ld a, $01
  jp hl
