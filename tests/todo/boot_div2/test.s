.incdir "../common"
.include "common.i"

  ld_a_ff DIV
  ld d, a
  nops 2
  ld_a_ff DIV
  ld c, a
  nops 15
  ld_a_ff DIV
  ld b, a
  ld_a_ff DIV

  ; GBP MGB-001 (probably DMG as well):
  ; A, B, C should contain $AC
  ; D should contain $AB
  ; GBASP AGS-101 (probably GBA as well):
  ; A, B should contain $27
  ; C, D should contain $26
  ; GBC CGB-001
  ; A should contain $27
  ; B, C, D should contain $26

test_finish:
  save_results
  jp print_results
