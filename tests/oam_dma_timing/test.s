; This tests the duration of OAM DMA.

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
  assert_b $15
  assert_c $15
  assert_d $14
  jp print_results

test:
  ; OAM DMA without extra NOPs
  start_oam_dma
- inc b
  ld a, (hl)
  cp $01
  jr nz, -

  ; OAM DMA with 6 extra nops
  ; this should not affect the counter, but we are at the cycle count boundary
  start_oam_dma
  nops 6
- inc c
  ld a, (hl)
  cp $01
  jr nz, -

  ; OAM DMA with 7 extra nops
  ; this should affect the counter
  start_oam_dma
  nops 7
- inc d
  ld a, (hl)
  cp $01
  jr nz, -

  jp test_finish
