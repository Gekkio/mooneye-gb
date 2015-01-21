; interrupt servicing is expected to have the following timing:
; M = 0: internal delay
; M = 1: internal delay
; M = 2: internal delay
; M = 3: PC push: memory access for high byte
; M = 4: PC push: memory access for low byte

.incdir "../common"
.include "common.i"

  di

.macro trigger_intr
  ld a, $08
  ld_ff_a IF, a
.endm

  ld a, $08
  ld_ff_a IE, a

  ; set first $20 bytes of VRAM to $81, so we
  ; have a known value when reading results
  wait_vblank
  ld hl, VRAM
  ld bc, $20
  ld a, $81
  call memset

  ei
  run_hiram_test

test_finish:
  ; GBP MGB-001 / GBC CGB-001 / GBASP AGS-101 (probably DMG/GBA as well)
  save_results
  assert_b $81
  assert_c $A7
  assert_d $FF
  assert_e $D1
  jp print_results

hiram_test:
  ld sp, OAM+$10
  ld d, $42
  ld e, $24

  start_oam_dma $80
  ld a, 36
- dec a
  jr nz, -
  nops 4
  ; set hl to address of test_round1 in hiram
  ld hl, $FF80 + (test_round1 - hiram_test)
  trigger_intr
  ; OAM is accessible at M=4, so we expect to see
  ; incorrect (= $81 written by OAM DMA) high byte, but correct low byte

  ; never executed
  test_failure

test_round1:
  pop bc
  push bc
  ei

  start_oam_dma $80
  ld a, 36
- dec a
  jr nz, -
  nops 5
  ; set hl to address of test_round2 in hiram
  ld hl, $FF80 + (test_round2 - hiram_test)
  trigger_intr
  ; OAM is accessible at M=3, so we expect to see
  ; correct high byte and low byte

  ; never executed
  test_failure

test_round2:
  pop de

  jp test_finish

.org $58
  jp hl
