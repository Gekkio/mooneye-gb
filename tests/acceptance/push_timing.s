; PUSH rr is expected to have the following timing:
; M = 0: instruction decoding
; M = 1: internal delay
; M = 2: memory access for high byte
; M = 3: memory access for low byte

.incdir "../common"
.include "common.i"

  di

  ; set first $20 bytes of VRAM to $81, so we
  ; have a known value when reading results
  wait_vblank
  ld hl, VRAM
  ld bc, $20
  ld a, $81
  call memset

  run_hiram_test

test_finish:
  ; GBP MGB-001 / GBC CGB-001 / GBASP AGS-101 (probably DMG/GBA as well)
  save_results
  assert_h $42
  assert_l $24
  assert_d $81
  assert_e $24
  jp print_results

hiram_test:
  ld sp, OAM+$10
  ld d, $42
  ld e, $24

  start_oam_dma $80
  ld a, 39
- dec a
  jr nz, -
  nops 2

  ; OAM is accessible at M=2
  push de
  nops 7
  pop hl

  start_oam_dma $80
  ld a, 39
- dec a
  jr nz, -
  nops 1

  ; OAM is accessible at M=3
  push de
  nops 7
  pop de

  jp test_finish
