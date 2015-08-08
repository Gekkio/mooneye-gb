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

; This tests EI instruction timing by forcing a serial
; interrupt with a write to IE/IF.

.incdir "../common"
.include "common.s"

  di
  ld a, INTR_SERIAL
  ld (IF), a
  ld (IE), a
  xor a
  ld b, a
  ld e, a

  ; We're expecting to see the effect of exactly one INC B instruction
  ei
  inc b
  inc b
  inc b

test_finish:
  ; GBP MGB-001 / GBC CGB-001 / GBASP AGS-101 (probably DMG/GBA as well)
  save_results
  assert_b $01
  assert_e $01
  jp process_results

.org INTR_VEC_SERIAL
  inc e
  jp test_finish
