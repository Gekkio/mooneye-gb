; DIV increments are supposed to happen every 64 cycles,
; and the "internal counter" is supposed to reset when DIV is reset
;
; ld a, (hl) is expected to have the following timing:
; M = 0: instruction decoding
; M = 1: memory access from (HL)

.incdir "../common"
.include "common.s"

  ld hl, DIV

.macro reset_div
  xor a
  ld (hl), a
.endm

  ; --- Test: increment is too late

  reset_div
  nops 61
  ; DIV increment should happen at M = 2, so the memory read
  ; should not see the increment, and we should get A = $00
  ld a, (hl)
  ld b, a

  ; --- Test: internal counter reset

  ; padding so if the internal counter is not reset, the next
  ; test should incorrectly see the increment
  nops 27

  ; repeat earlier test
  reset_div
  nops 61
  ; DIV increment should happen at M = 2, so the memory read
  ; should not see the increment, and we should get A = $00
  ld a, (hl)
  ld c, a

  ; --- Test: increment is exactly on time

  reset_div
  nops 62
  ; DIV increment should happen at M = 1, so the memory read
  ; should see the increment, and we should get A = $01
  ld a, (hl)
  ld d, a

test_finish:
  ; GBP MGB-001 / GBC CGB-001 / GBASP AGS-101 (probably DMG/GBA as well)
  save_results
  assert_b $00
  assert_c $00
  assert_d $01
  jp process_results
