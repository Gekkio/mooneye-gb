; JP cc, nn is expected to have the following timing:
; M = 0: instruction decoding
; M = 1: nn read: memory access for low byte
; M = 2: nn read: memory access for high byte
; M = 3: internal delay

.incdir "../common"
.include "common.i"

  di

  wait_vblank
  ; copy rest of wram_test to VRAM
  ld hl, VRAM
  ld de, (wram_test + 2)
  ld bc, $10
  call memcpy

  ; also copy wram_test to OAM
  ld hl, OAM - 2
  ld de, wram_test
  ld bc, $10
  call memcpy

  run_hiram_test

test_finish:
  ; GBP MGB-001 / GBC CGB-001 / GBASP AGS-101 (probably DMG/GBA as well)
  save_results
  jp print_results

; test procedure which will be copied to WRAM/OAM
; the first two bytes of JP cc, nn will be at $FDFE, so
; the high byte of nn is at the first byte of OAM during testing
wram_test:
  jp c, $1a00

; $1F80 - $1FE0 will be copied to $FF80 - $FFE0
.org $1f80
hiram_test:
  ; set low byte of nn to $ca
  ld a, $ca
  ld (OAM - 1), a

  ld b, 38
  start_oam_dma $80
- dec b
  jr nz, -
  nops 2
  ld hl, OAM - 2
  scf
  jp hl
  ; the memory read of nn is aligned to happen exactly one cycle
  ; before the OAM DMA end, so high byte of nn = $FF
  ; therefore the call becomes:
  ;   jp c, $ffca

test_round2:
  ; set low byte of nn to $da
  ld a, $da
  ld (OAM - 1), a

  ld b, 38
  start_oam_dma $80
- dec b
  jr nz, -
  nops 3
  ld hl, OAM - 2
  scf
  jp hl
  ; the memory read of nn is aligned to happen exactly after OAM DMA
  ; ends, so high byte of nn = $1a
  ; therefore the call becomes:
  ;   jp c, $1ada

; this will be copied to $FFCA
.org $1fca
finish_round1:
  nops 2
  jp $FF80 + (test_round2 - hiram_test)

; this will be copied to $FFDA
.org $1fda
fail_round2:
  test_failure

.org $1aca
fail_round1:
  test_failure

.org $1ada
finish_round2:
  nops 2
  jp test_finish
