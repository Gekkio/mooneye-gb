# simple_scx

This test can be used to analyse the effect of SCX on timings using a logic
analyser. A checkerboard pattern consisting of tiles $03 and $07 is written
to VRAM, and the test uses LY=LYC and hblank interrupts to allow precise
triggering.

Pins used in VCD files:

* A[0..15]: cartridge bus address
* CLK: 1 MHz cartridge bus clock
* MOE: VRAM output enable
* MCS: VRAM chip select
* PIXCLK: LCD pixel clock
* ICLK: 4 MHz input/master clock
* MA[0..5], MA12: VRAM address bits

A good trigger point is the falling edge of VRAM CS signal when the CPU
is halted and is waiting for the hblank interrupt. For example:

* A[0..15] = $81AC (at the time of testing, this was the address that is in the bus during HALT waiting for hblank)
* CLK=1
* MOE=0
* MCS=0
* PIXCLK=0
* MA0=0
* MA1=0
* MA2=0
* MA3=0
* MA4=0
* MA5=0
* MA6=0
* MA12=0

## Test results

The test was compiled with several different TEST_SCX values, and a logic
analyser was triggered using the earlier mentioned trigger.

* The 159 pixel clock pulses are delayed by (SCX & $7) cycles
* The tile map numbers are affected by (SCX >> 3)
* There is sometimes noise in MOE signal when it and MCS are pulled high. My
  conclusion is that MOE is actually in high-impedance mode. This might be
  incorrect, but does not significantly affect the results.
* The VRAM address is actually valid for less than the time shown in the
  diagrams. This is a mistake, but does not matter from an emulation point of
  view
* The purpose of the single pixel clock pulse in the beginning is unclear.
  That pulse is noninverted, while during the actual pushing of 159 pixels
  the clock is inverted!
* At SCX=0, the single pulse and 159 later pulses are separated by 8 edges.
  It doesn't matter whether you count rising or falling edges.
