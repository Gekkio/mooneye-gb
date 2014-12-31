; This tests the whether another OAM DMA can be started

.incdir "../common"
.include "common.i"

.macro start_oam_dma
  ; wait for vblank and set $FE00 to $00, so we can wait until
  ; we see $01 after DMA
  wait_vblank
  xor a
  ld (hl), a

  ; start OAM DMA $8000 -> $FE00
  ld a, $80
  ld_ff_a DMA
.endm

  di

  ; set $8000 to $01
  wait_vblank
  ld a, $01
  ld (VRAM), a

  ; copy test procedure to hiram $FF80
  ; during OAM DMA the CPU cannot access any other memory,
  ; so our code needs to be there
  ld hl, $FF80
  ld de, test
  ld bc, $60 ; 0x60 bytes should be enough
  call memcpy

  ; set hl, reset registers
  ld hl, OAM
  xor a
  ld b, a
  ld c, a
  ld d, a

  ; jump to test procedure in hiram
  jp $FF80

test_finish:
  ; GBP MGB-001 / GBASP AGS-101 (probably DMG/GBC as well)
  save_results
  assert_c $FF
  assert_d $01
  jp print_results

test:
  ld b, 40
  start_oam_dma
- dec b
  jr nz, -

  ; the memory read is aligned to happen exactly one cycle before the OAM DMA end,
  ; so we should still see $FF
  ld a, (hl)
  ld c, a

  ld b, 40
  start_oam_dma
- dec b
  jr nz, -
  nops 1

  ; the memory read is aligned to happen exactly after the OAM DMA ends, so
  ; we should see $01
  ld a, (hl)
  ld d, a

  jp test_finish
