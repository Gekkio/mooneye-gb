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

.define VRAM $8000
.define OAM  $FE00

.define P1   $FF00
.define SB   $FF01
.define SC   $FF02
.define DIV  $FF04
.define TIMA $FF05
.define TMA  $FF06
.define TAC  $FF07
.define IF   $FF0F
.define LCDC $FF40
.define STAT $FF41
.define SCY  $FF42
.define SCX  $FF43
.define LY   $FF44
.define LYC  $FF45
.define DMA  $FF46
.define BGP  $FF47
.define OBP0 $FF48
.define OBP1 $FF49
.define WY   $FF4A
.define WX   $FF4B
.define IE   $FFFF

.macro ld_a_ff ARGS addr
  ldh a, (addr - $FF00)
.endm

.macro ld_ff_a ARGS addr
  ldh (addr - $FF00), a
.endm
