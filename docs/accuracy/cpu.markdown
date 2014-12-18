# CPU emulation accuracy

## Open questions

### What is the exact behaviour of DI and EI?

These instructions don't change interrupts immediately, but with a delay. What happens if you do another DI/EI while a delayed operation is ongoing?

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
