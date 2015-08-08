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

; We know that the effect of EI has a delay. This tests how EI before HALT behaves.
;
; If EI is before HALT, the HALT instruction is expected to perform its normal
; IME=1 behaviour

.incdir "../common"
.include "common.s"

.macro clear_IF
  xor a
  ld_ff_a IF
.endm

.macro enable_IE_vblank
  ld a, INTR_VBLANK
  ld_ff_a IE
.endm

  di
  wait_ly $00
  clear_IF
  enable_IE_vblank

  ei
  halt
  di

result_ime0:
  test_failure_string "IME=0"

result_ime1:
  test_ok

.org INTR_VEC_VBLANK
  jp result_ime1
