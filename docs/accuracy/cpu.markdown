# CPU emulation accuracy

## Open questions

### Some instructions take more cycles than just the memory accesses. At which point in the instruction execution do these extra cycles occur?

These instructions cost more than the memory accesses:

* LD SP, HL
* LD HL, (SP+e)
* ADD HL, rr
* ADD SP, e
* JP cc, nn
* JP nn
* JR cc, n
* JR n
* INC rr
* DEC rr
* PUSH rr
* RST
* RET cc
* RET
* RETI

Most of these instructions involve writing a 16-bit register, which could explain the timing.

## Answered questions

### Does BIT b, (HL) take 12 or 16 cycles?

*Answer:* 12 cycles

Blargg's instruction timing ROM confirms that BIT takes 12, and RES/SET take 16 cycles, which makes perfect sense.
Some opcode listings in the internet (e.g. GBCPUman.pdf) are wrong.

### What is the exact behaviour of DI and EI?

These instructions don't change interrupts immediately, but instead have a delay before they take effect. Both instructions simply set an internal flag, which will have take effect after the next instruction is executed. If you rapidly switch between DI/EI right after another, the internal flag has no effect during the switching, and the last instruction wins.

So, assuming interrupts are disabled, and an interrupt has already been requested, this code will cause only one interrupt:

    ei
    di
    ei
    di
    ei
    nop ; <- interrupt is acknowledged between these two
    nop ; <- instructions
