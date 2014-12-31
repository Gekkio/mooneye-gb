.incdir "../common"
.include "common.i"

  ld_a_ff DIV
  ld c, a
  nops 1
  ld_a_ff DIV
  ld b, a
  nops 15
  ld_a_ff DIV

  ; GBP MGB-001 (probably DMG as well):
  ; A should contain $AC
  ; B, C should contain $AB
  ; GBASP AGS-101 (probably GBC as well):
  ; A, B, C should contain $26

test_finish:
  save_results
  jp print_results
