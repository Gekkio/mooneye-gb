; This tests the duration of OAM DMA

.incdir "../common"
.include "common.i"

  di

  ; set $8000 to $01
  wait_vblank
  ld a, $01
  ld (VRAM), a

  run_hiram_test

test_finish:
  ; GBP MGB-001 / GBASP AGS-101 (probably DMG/GBC as well)
  save_results
  assert_c $FF
  assert_d $01
  jp print_results

hiram_test:
  ld hl, OAM
  ld b, 40
  start_oam_dma $80
- dec b
  jr nz, -

  ; the memory read is aligned to happen exactly one cycle before the OAM DMA end,
  ; so we should still see $FF
  ld a, (hl)
  ld c, a

  ld b, 40
  start_oam_dma $80
- dec b
  jr nz, -
  nops 1

  ; the memory read is aligned to happen exactly after the OAM DMA ends, so
  ; we should see $01
  ld a, (hl)
  ld d, a

  jp test_finish
