# Boot rom emulation accuracy

## Open questions

## Answered questions

### Is it possible to restore the bootrom by writing some value to $FF50?

*Answer*: No

This was tested on a GBP (MGB-001) with the following test ROM, which attempts to write all possible values to $FF50:

      ld hl, $0000
      ld b, $00         ; value to be written to $FF50

    - ld a, b
      ld ($FF00+$50), a
      ld a, (HL)
      cp $31            ; DMG bootrom should have $31 at $0000
      jr z, +
      inc b             ; attempt next value
      jr nz, -          ; retry until overflow

    + nop

      ; if A is $FF and B is $00, test failed
      ; A should be $31
      ; B should contain the written value
        
      jp finish
