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

; PUSH rr is expected to have the following timing:
; M = 0: instruction decoding
; M = 1: internal delay
; M = 2: memory access for high byte
; M = 3: memory access for low byte

; Verified results:
;   pass: DMG, MGB, SGB, SGB2, CGB, AGB, AGS
;   fail: -

.incdir "../common"
.include "common.s"

  di

  ; set first $20 bytes of VRAM to $81, so we
  ; have a known value when reading results
  wait_vblank
  ld hl, VRAM
  ld bc, $20
  ld a, $81
  call memset

  run_hiram_test

test_finish:
  save_results
  assert_h $42
  assert_l $24
  assert_d $81
  assert_e $24
  jp process_results

hiram_test:
  ld sp, OAM+$10
  ld d, $42
  ld e, $24

  start_oam_dma $80
  ld a, 39
- dec a
  jr nz, -
  nops 2

  ; OAM is accessible at M=2
  push de
  nops 7
  pop hl

  start_oam_dma $80
  ld a, 39
- dec a
  jr nz, -
  nops 1

  ; OAM is accessible at M=3
  push de
  nops 7
  pop de

  jp test_finish
