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

; This tests starting another OAM DMA while one is already active

; Verified results:
;   pass: DMG, MGB, SGB, SGB2, CGB, AGB, AGS
;   fail: -

.incdir "../common"
.include "common.s"

  di

  ; set $8000 to $01
  wait_vblank
  ld a, $01
  ld (VRAM), a

  run_hiram_test

test_finish:
  save_results
  assert_c $FF
  assert_d $01
  jp process_results

hiram_test:
  ld hl, OAM
  ld b, 40
  start_oam_dma $80
  nops 7
  ldh (<DMA), a
- dec b
  jr nz, -

  ; the memory read is aligned to happen exactly one cycle before the second OAM DMA end,
  ; so we should still see $FF
  ld a, (hl)
  ld c, a

  ld b, 40
  start_oam_dma $80
  nops 7
  ldh (<DMA), a
- dec b
  jr nz, -
  nops 1

  ; the memory read is aligned to happen exactly after the second OAM DMA ends, so
  ; we should see $01
  ld a, (hl)
  ld d, a

  jp test_finish
