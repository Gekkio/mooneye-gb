# write_timing

This test can be used to check the write timings of different memory areas of
the Game Boy.

A good trigger point is the write to $3FFF. For example:

* A[0..15]=$3FFF
* WR=0

The test was executed with the following setup:

* Super Famicom
* Super Game Boy 2
  * 187 500 Hz master clock from Teensy 3.1
* EMS 64M flash cartridge

The slow clock lowers the effect of propagation delays, and makes it much easier
to see which clock transitions correspond to transitions in signals.
Pins used in the VCD file:

* A[0..15]: cartridge bus address
* D[0..7]: cartridge bus data
* CLK: 46 875 Hz cartridge bus clock
* RD: cartridge bus read enable
* WR: cartridge bus write enable
* MREQ: cartridge bus ram enable
* ICLK: 187 500 Hz master clock

## Test results

* Address is undefined until the falling edge of the first cycle
* A15 and MREQ are used as chip output enable signals, and they are asserted one
  edge later than the primary address bits
* The data signals are asserted at the same time as WR
* Video RAM addresses and data are not visible on the cartridge bus
* When accessing $FE00-$FFFF, the addresses are visible but the data is not
