; PUSH rr is expected to have the following timing:
; t = 0: instruction decoding
; t = 1: internal delay
; t = 2: memory access for high byte
; t = 3: memory access for low byte

.incdir "../common"
.include "common.i"

.macro start_oam_dma
  ; start OAM DMA $8000 -> $FE00
  wait_vblank
  ld a, $80
  ld_ff_a DMA
.endm

  di

  ; set first $20 bytes of VRAM to $81, so we
  ; have a known value when reading results
  wait_vblank
  ld hl, VRAM
  ld bc, $20
  ld a, $81
  call memset

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
  assert_h $42
  assert_l $24
  assert_d $81
  assert_e $24
  jp print_results

test:
  ld sp, OAM+$10
  ld d, $42
  ld e, $24

  start_oam_dma
  ld a, 39
- dec a
  jr nz, -
  nops 2

  ; OAM is accessable at t=2
  push de
  nops 7
  pop hl

  start_oam_dma
  ld a, 39
- dec a
  jr nz, -
  nops 1

  ; OAM is accessable at t=3
  push de
  nops 7
  pop de

  jp test_finish
