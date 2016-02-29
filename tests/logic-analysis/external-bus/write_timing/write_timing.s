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

.incdir "../../../common"
.include "common.s"

.macro test
  ld (\1), a
.endm

  di
  wait_vblank
  disable_lcd

  xor a
  test $3FFF
  test $7FFF
  test $9FFF ; not visible
  test $BFFF
  test $DFFF
  test $FDFF
  test $FE9F
  test $FEFF

  ; data not visible
  test $FF00
  test $FF01
  test $FF04
  test $FF0F
  test $FF1F
  test $FF2F
  test $FF3F
  test $FF4F
  test $FF5F
  test $FF6F
  test $FF7F
  test $FFFE
  test $FFFF

- halt
  nop
  jr -
