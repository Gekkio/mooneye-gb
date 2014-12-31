.incdir "../common"
.include "common.i"

  ld_a_ff DIV
  ld c, a
  nops 2
  ld_a_ff DIV
  ld b, a
  nops 15
  ld_a_ff DIV

  ; GBP MGB-001 (probably DMG as well):
  ; A, B should contain $AC
  ; C should contain $AB
  ; GBASP AGS-101 (probably GBC as well):
  ; A should contain $27
  ; B, C should contain $26

test_finish:
  save_results
  jp print_results
