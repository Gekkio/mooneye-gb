; This test checks that the OAM area has no unused bits
; On DMG the sprite flags have unused bits, but they are still
; writable and readable normally

.incdir "../common"
.include "common.i"

  di
  wait_vblank
  disable_lcd

  ld hl, OAM
  ld b, $a0

-
; Write all 1s (= $FF) and expect the same value back
  ld a, $FF
  ld (hl), a
  ld a, (hl)
  cp $FF
  jr nz, fail_1

; Write all 0s (= $00) and expect the same value back
  ld a, $00
  ld (hl), a
  ld a, (hl)
  cp $00
  jr nz, fail_0
  
  inc hl
  dec b
  jr nz, -

test_finish:
  enable_lcd
  ; GBP MGB-001 / GBC CGB-001 / GBASP AGS-101 (probably DMG/GBA as well)
  save_results
  jp print_results

fail_1:
  enable_lcd
  test_failure
fail_0:
  enable_lcd
  test_failure
