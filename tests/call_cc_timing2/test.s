; CALL cc, nn is expected to have the following timing:
; M = 0: instruction decoding
; M = 1: nn read: memory access for low byte
; M = 2: nn read: memory access for high byte
; M = 3: internal delay
; M = 4: PC push: memory access for high byte
; M = 5: PC push: memory access for low byte

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
  assert_b $81
  assert_c $81
  assert_d $81
  assert_e $B9
  assert_h $FF
  assert_l $D6
  jp print_results

hiram_test:
  ld sp, OAM+$20
  start_oam_dma $80
  ld a, 38
- dec a
  jr nz, -
  nops 1
  scf
  call c, $FF80 + (finish_round1 - hiram_test)
  ; OAM is accessible at M=6, so we expect to see
  ; incorrect low and high bytes (= $81 written by OAM DMA)

finish_round1:
  pop bc

  start_oam_dma $80
  ld a, 38
- dec a
  jr nz, -
  nops 2
  scf
  call c, $FF80 + (finish_round2 - hiram_test)
  ; OAM is accessible at M=5, so we expect to see
  ; incorrect (= $81 written by OAM DMA) high byte, but correct low byte

finish_round2:
  pop de

  start_oam_dma $80
  ld a, 38
- dec a
  jr nz, -
  nops 3
  scf
  call c, $FF80 + (finish_round3 - hiram_test)
  ; OAM is accessible at M=4, so we expect to see
  ; correct high byte and low byte

finish_round3:
  pop hl

  jp test_finish
