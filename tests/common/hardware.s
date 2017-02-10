; Copyright (C) 2014-2017 Joonas Javanainen <joonas.javanainen@gmail.com>
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

.memorymap
  defaultslot 1
  slot 0 start $0000 size $4000
  slot 1 start $4000 size $4000
  slot 2 start $C000 size $1000
  slot 3 start $D000 size $1000
  slot 4 start $A000 size $2000
  slot 5 start $FF80 size $007F
.endme

.define INTR_VBLANK (1 << 0)
.define INTR_STAT   (1 << 1)
.define INTR_TIMER  (1 << 2)
.define INTR_SERIAL (1 << 3)
.define INTR_JOYPAD (1 << 4)

.define INTR_VEC_VBLANK $40
.define INTR_VEC_STAT   $48
.define INTR_VEC_TIMER  $50
.define INTR_VEC_SERIAL $58
.define INTR_VEC_JOYPAD $60

.define VRAM  $8000
.define OAM   $FE00
.define HIRAM $FF80

.define VRAM_LEN  $2000
.define OAM_LEN   $A0
.define HIRAM_LEN $7F

.define P1    $FF00
.define SB    $FF01
.define SC    $FF02
.define DIV   $FF04
.define TIMA  $FF05
.define TMA   $FF06
.define TAC   $FF07
.define IF    $FF0F
.define NR10  $FF10
.define NR11  $FF11
.define NR12  $FF12
.define NR13  $FF13
.define NR14  $FF14
.define NR21  $FF16
.define NR22  $FF17
.define NR23  $FF18
.define NR24  $FF19
.define NR30  $FF1A
.define NR31  $FF1B
.define NR32  $FF1C
.define NR33  $FF1D
.define NR34  $FF1E
.define NR41  $FF20
.define NR42  $FF21
.define NR43  $FF22
.define NR44  $FF23
.define NR50  $FF24
.define NR51  $FF25
.define NR52  $FF26
.define LCDC  $FF40
.define STAT  $FF41
.define SCY   $FF42
.define SCX   $FF43
.define LY    $FF44
.define LYC   $FF45
.define DMA   $FF46
.define BGP   $FF47
.define OBP0  $FF48
.define OBP1  $FF49
.define WY    $FF4A
.define WX    $FF4B
.define KEY1  $FF4D
.define VBK   $FF4F
.define BOOT  $FF50
.define HDMA1 $FF51
.define HDMA2 $FF52
.define HDMA3 $FF53
.define HDMA4 $FF54
.define HDMA5 $FF55
.define RP    $FF56
.define BCPS  $FF68
.define BCPD  $FF69
.define OCPS  $FF6A
.define OCPD  $FF6B
.define SVBK  $FF70
.define IE    $FFFF
