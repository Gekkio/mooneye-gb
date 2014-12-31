.incdir "../common"
.include "common.i"

  ldh a, (DIV-$FF00)
  ld c, a
  nops 1
  ldh a, (DIV-$FF00)
  ld b, a
  nops 15
  ldh a, (DIV-$FF00)

  ; GBP MGB-001 (probably DMG as well):
  ; A should contain $AC
  ; B, C should contain $AB
  ; GBASP AGS-101 (probably GBC as well):
  ; A, B, C should contain $26

test_finish:
  save_results
  jp print_results
