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

; CALL nn is expected to have the following timing:
; M = 0: instruction decoding
; M = 1: nn read: memory access for low byte
; M = 2: nn read: memory access for high byte
; M = 3: internal delay
; M = 4: PC push: memory access for high byte
; M = 5: PC push: memory access for low byte

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
  assert_b $81
  assert_c $81
  assert_d $81
  assert_e $B9
  assert_h $FF
  assert_l $D6
  jp process_results

hiram_test:
  ld sp, OAM+$20
  start_oam_dma $80
  ld a, 38
- dec a
  jr nz, -
  nops 2
  call $FF80 + (finish_round1 - hiram_test)
  ; OAM is accessible at M=6, so we expect to see
  ; incorrect low and high bytes (= $81 written by OAM DMA)

finish_round1:
  pop bc

  start_oam_dma $80
  ld a, 38
- dec a
  jr nz, -
  nops 3
  call $FF80 + (finish_round2 - hiram_test)
  ; OAM is accessible at M=5, so we expect to see
  ; incorrect (= $81 written by OAM DMA) high byte, but correct low byte

finish_round2:
  pop de

  start_oam_dma $80
  ld a, 38
- dec a
  jr nz, -
  nops 4
  call $FF80 + (finish_round3 - hiram_test)
  ; OAM is accessible at M=4, so we expect to see
  ; correct high byte and low byte

finish_round3:
  pop hl

  jp test_finish
