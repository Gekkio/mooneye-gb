; This tests starting another OAM DMA while one is already active

.incdir "../common"
.include "common.s"

  di

  ; set $8000 to $01
  wait_vblank
  ld a, $01
  ld (VRAM), a

  run_hiram_test

test_finish:
  ; GBP MGB-001 / GBC CGB-001 / GBASP AGS-101 (probably DMG/GBA as well)
  save_results
  assert_c $FF
  assert_d $01
  jp process_results

hiram_test:
  ld hl, OAM
  ld b, 40
  start_oam_dma $80
  nops 7
  ld_ff_a DMA
- dec b
  jr nz, -

  ; the memory read is aligned to happen exactly one cycle before the second OAM DMA end,
  ; so we should still see $FF
  ld a, (hl)
  ld c, a

  ld b, 40
  start_oam_dma $80
  nops 7
  ld_ff_a DMA
- dec b
  jr nz, -
  nops 1

  ; the memory read is aligned to happen exactly after the second OAM DMA ends, so
  ; we should see $01
  ld a, (hl)
  ld d, a

  jp test_finish
