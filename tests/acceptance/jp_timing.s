; This file is part of Mooneye GB.
; Copyright (C) 2014-2015 Joonas Javanainen <joonas.javanainen@gmail.com>
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

; JP nn is expected to have the following timing:
; M = 0: instruction decoding
; M = 1: nn read: memory access for low byte
; M = 2: nn read: memory access for high byte
; M = 3: internal delay

.incdir "../common"
.include "common.s"

  di

  wait_vblank
  ; copy rest of wram_test to VRAM
  ld hl, VRAM
  ld de, (wram_test + 2)
  ld bc, $10
  call memcpy

  ; also copy wram_test to OAM
  ld hl, OAM - 2
  ld de, wram_test
  ld bc, $10
  call memcpy

  run_hiram_test

test_finish:
  ; GBP MGB-001 / GBC CGB-001 / GBASP AGS-101 (probably DMG/GBA as well)
  test_ok

; test procedure which will be copied to WRAM/OAM
; the first two bytes of JP nn will be at $FDFE, so
; the high byte of nn is at the first byte of OAM during testing
wram_test:
  jp $1a00

fail_round1:
  test_failure_string "FAIL: ROUND 1"

fail_round2:
  test_failure_string "FAIL: ROUND 2"

; $1F80 - $1FE0 will be copied to $FF80 - $FFE0
.org $1f80
hiram_test:
  ; set low byte of nn to $ca
  ld a, $ca
  ld (OAM - 1), a

  ld b, 38
  start_oam_dma $80
- dec b
  jr nz, -
  nops 3
  ld hl, OAM - 2
  jp hl
  ; the memory read of nn is aligned to happen exactly one cycle
  ; before the OAM DMA end, so high byte of nn = $FF
  ; therefore the call becomes:
  ;   jp $ffca

test_round2:
  ; set low byte of nn to $da
  ld a, $da
  ld (OAM - 1), a

  ld b, 38
  start_oam_dma $80
- dec b
  jr nz, -
  nops 4
  ld hl, OAM - 2
  jp hl
  ; the memory read of nn is aligned to happen exactly after OAM DMA
  ; ends, so high byte of nn = $1a
  ; therefore the call becomes:
  ;   jp $1ada

; this will be copied to $FFCA
.org $1fca
finish_round1:
  nops 2
  jp $FF80 + (test_round2 - hiram_test)

; this will be copied to $FFDA
.org $1fda
  jp fail_round2

.org $1aca
  jp fail_round1

.org $1ada
finish_round2:
  nops 2
  jp test_finish
