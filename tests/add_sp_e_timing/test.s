; ADD SP, e is expected to have the following timing:
; M = 0: instruction decoding
; M = 1: memory access for e
; M = 2: internal delay
; M = 3: internal delay

.incdir "../common"
.include "common.i"

  di

  wait_vblank
  ; copy rest of wram_test to VRAM
  ld hl, VRAM
  ld de, (wram_test + 1)
  ld bc, $10
  call memcpy

  ; also copy wram_test to OAM
  ld hl, OAM - 1
  ld de, wram_test
  ld bc, $10
  call memcpy

  ld sp, $CFFF
  run_hiram_test

test_finish:
  ; GBP MGB-001 / GBC CGB-001 / GBASP AGS-101 (probably DMG/GBA as well)
  save_results
  assert_b $CF
  assert_c $FE
  assert_d $D0
  assert_e $41
  jp print_results

; test procedure which will be copied to WRAM/OAM
; the first byte of ADD SP, e will be at $FDFF, so
; the e parameter is at the first byte of OAM during testing
wram_test:
  ; if OAM DMA is still running $42 will be replaced with $FF
  add sp, $42
  ; save result to temporary storage
  ld hl, sp+$00
  ld sp, $CFFF
  push hl
  ; set HL = DE
  push de
  pop hl
  jp hl

hiram_test:
  ld b, 38
  start_oam_dma $80
- dec b
  jr nz, -
  nops 1
  ; set hl to address of finish_round1 in hiram
  ld de, $FF80 + (finish_round1 - hiram_test)
  ld hl, OAM - 1
  jp hl
  ; the memory read of ADD SP, e is aligned to happen exactly one cycle
  ; before the OAM DMA end, so e = $FF = -1

finish_round1:
  pop bc
  ld sp, $CF80
  push bc
  ld sp, $CFFF

  ld b, 38
  start_oam_dma $80
- dec b
  jr nz, -
  nops 2
  ; set hl to address of finish_round2 in hiram
  ld de, $FF80 + (finish_round2 - hiram_test)
  ld hl, OAM - 1
  jp hl
  ; the memory read of ADD SP, e is aligned to happen exactly one cycle
  ; before the OAM DMA end, so e = $42 = 42

finish_round2:
  pop de
  ld sp, $CF80 - 2
  pop bc
  jp test_finish
