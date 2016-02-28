# read_timing

This test can be used to check the read timings of different memory areas of
the Game Boy.

A good trigger point is the read from $3FFF. For example:

* A[0..15]=$3FFF
* RD=0

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

* Game Boy keeps RD low at all times except during writes. RD is not pulsed
  on each read!
* If a write preceded the current read, RD is raised high on the rising
  edge of the CPU clock
* Address is undefined until the falling edge of the first cycle
* A15 and MREQ are used as chip output enable signals, and they are asserted one
  edge later than the primary address bits
* Video RAM addresses and data are not visible on the cartridge bus
* Work RAM is physically in the same bus, so all accesses are fully visible
* Echo area accesses are fully visible. The work RAM responds to these accesses
* When accessing $FE00-$FFFF, the addresses are visible but the data is not

This test does not provide any information about the timing of the data.
Different devices in the cartridge bus may have different timings, but in the
end the only thing that is important is when the Game Boy samples the data.
