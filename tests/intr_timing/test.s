; Serving an interrupt is supposed to take 5 cycles.
; We know from div_timing that this code should not see a div increment:
;   reset_div
;     nops 61
;   ld a, (hl)
; On the other hand, this code should see the increment:
;   reset_div
;     nops 62
;   ld a, (hl)
; We set up a similar scenario by triggering an interrupt using IE/IF flags.
; In total we have
;   reset_div
;     x nops               x cycles
;     trigger_intr 2 + 3 = 5 cycles
;     interrupt handling:  5 cycles
;     jp hl                1 cycle
;   ld a, (bc)
; So, x=50 is equivalent to the nops 61 case,
; and x=51 is equivalent to the nops 62 case

.incdir "../common"
.include "common.i"

  ld bc, DIV

  ld a, $08
  ld_ff_a IE, a

.macro reset_div
  xor a
  ld (bc), a
.endm

.macro trigger_intr
  ld a, $08
  ld_ff_a IF, a
.endm

  ei
  ld hl, test_round1

  reset_div
  nops 50
  trigger_intr

  ; never executed
  test_failure

test_round1:
  ld a, (bc)
  ld d, a

  ei
  ld hl, test_round2

  reset_div
  nops 51
  trigger_intr

  ; never executed
  test_failure

test_round2:
  ld a, (bc)
  ld e, a

  jp test_finish

test_finish:
  ; GBP MGB-001 / GBC CGB-001 / GBASP AGS-101 (probably DMG/GBA as well)
  save_results
  assert_d $00
  assert_e $01
  jp print_results

.org $58
  jp hl
