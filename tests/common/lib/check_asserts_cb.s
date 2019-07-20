; Copyright (C) 2014-2019 Joonas Javanainen <joonas.javanainen@gmail.com>
;
; Permission is hereby granted, free of charge, to any person obtaining a copy
; of this software and associated documentation files (the "Software"), to deal
; in the Software without restriction, including without limitation the rights
; to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
; copies of the Software, and to permit persons to whom the Software is
; furnished to do so, subject to the following conditions:
;
; The above copyright notice and this permission notice shall be included in
; all copies or substantial portions of the Software.
;
; THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
; IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
; FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
; AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
; LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
; OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
; SOFTWARE.

.section "check_asserts_cb"
check_asserts_cb:
  ld de, v_regs_save
  call print_newline
  print_string_literal "Registers"
  call print_newline
  call print_newline
  call print_reg_dump
  call print_newline

  ldh a, (<v_regs_flags)
  or a
  jr z, +
  print_string_literal "Assertions"
  call print_newline
  call print_newline
  call @check_asserts

  ld a, d
  or a
  jr z, +
  call print_newline
  print_string_literal "Test failed"
+ ret

  @check_asserts:
    xor a
    ld d, a

    ldh a, (<v_regs_flags)
    ld e, a

    .macro __check_assert ARGS flag str value expected
      bit flag, e
      jr z, @skip\@

      print_string_literal str
      print_string_literal ": "

      ldh a, (<value)
      ld c, a
      ldh a, (<expected)
      cp c
      jr z, @ok\@
    @fail\@:
      call print_hex8
      print_string_literal "! "
      inc d
      jr @out\@
    @ok\@:
      print_string_literal "OK  "
      jr @out\@
    @skip\@:
      print_string_literal "       "
    @out\@:
    .endm

    print_string_literal "  "
    __check_assert 0 "A" v_regs_save.reg_a v_regs_assert.reg_a
    __check_assert 1 "F" v_regs_save.reg_f v_regs_assert.reg_f
    call print_newline
    print_string_literal "  "
    __check_assert 2 "B" v_regs_save.reg_b v_regs_assert.reg_b
    __check_assert 3 "C" v_regs_save.reg_c v_regs_assert.reg_c
    call print_newline
    print_string_literal "  "
    __check_assert 4 "D" v_regs_save.reg_d v_regs_assert.reg_d
    __check_assert 5 "E" v_regs_save.reg_e v_regs_assert.reg_e
    call print_newline
    print_string_literal "  "
    __check_assert 6 "H" v_regs_save.reg_h v_regs_assert.reg_h
    __check_assert 7 "L" v_regs_save.reg_l v_regs_assert.reg_l
    jp print_newline
.ends
