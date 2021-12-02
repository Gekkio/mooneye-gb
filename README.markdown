# Mooneye GB

Mooneye GB is a Game Boy research project and emulator written in Rust.

[![Build Status](https://github.com/Gekkio/mooneye-gb/workflows/ci/badge.svg)](https://github.com/Gekkio/mooneye-gb/actions)

The main goals of this project are accuracy and documentation. Some existing
emulators are very accurate (Gambatte, BGB >= 1.5) but are not documented very
clearly, so they are not that good references for emulator developers. I want
this project to document as clearly as possible *why* certain behaviour is
emulated in a certain way. This also means writing a lot of test ROMs to figure
out corner cases and precise behaviour on real hardware.

For documentation about known behaviour, see [Game Boy: Complete Technical Reference](https://github.com/Gekkio/gb-ctr)

*Looking for the mooneye-gb test ROMs?* They are now part of [Mooneye Test Suite](https://github.com/Gekkio/mooneye-test-suite/).

Non-goals:

* CGB (Game Boy Color) support. It would be nice, but I want to make the normal
  Game Boy support extremely robust first.
* A debugger
* A good user interface. Building native UIs with Rust is a bit painful at the
  moment.

**Warning**:

* Project is WIP
* Doesn't work properly without a boot ROM
* The emulator is lagging behind hardware research. I don't want to spend time
  making short-lived and probably incorrect fixes to the emulator if I'm not
  sure about the hardware behaviour.

## Performance

**Always compile in release mode if you care about performance!**

On a i7-3770K desktop machine I can usually run ROMs with 2000 - 4000% speed.
Without optimizations the speed drops to 150 - 200%, which is still fine for
development purposes.

Raspberry Pi with X11 desktop works but is too slow because there is no OpenGL
acceleration.

The emulator is runnable on Android, but cross-compiling and packaging is a
huge pain and touch controls would have to be implemented, so I'm not
supporting Android at the moment.

## Running the emulator

Requirements:

* Rust 1.26
* SDL2 development libraries for your platform must be installed

### GUI

1. `cargo run --release`
2. Follow the instructions

### Command-line
1. Acquire a Game Boy bootrom, and put it to `$HOME/.local/share/mooneye-gb/bootroms/dmg_boot.bin`
2. `cargo build --release`
3. `cargo run --release -- PATH_TO_GAMEBOY_ROM`

On Windows, also download an SDL2 package containing SDL2.dll, and put it to
`target/debug` and `target/release`.

### Game Boy keys

| Game Boy | Key        |
| -------- | ---------- |
| Dpad     | Arrow keys |
| A        | Z          |
| B        | X          |
| Start    | Return     |
| Select   | Backspace  |

### Other keys

| Function                   | Key       |
| -------------------------- | --------- |
| Fast forward               | Shift     |
| Toggle performance overlay | F2        |

## Test suite

### Blargg's tests

| Test              | mooneye-gb |
| ----------------- | ---------- |
| cpu instrs        | :+1:       |
| dmg sound 2       | :x:        |
| instr timing      | :+1:       |
| mem timing 2      | :+1:       |
| oam bug 2         | :x:        |
| cgb sound 2       |            |

Notes:

* cpu_instrs fails on MGB/SGB2 hardware and emulators emulating them correctly.
  The ROM incorrectly detects the device as CGB, and attempts to perform a CPU
  speed change which causes a freeze (STOP instruction with joypad disabled)
* dmg_sound-2 test #10 can fail randomly on real hardware and seems to depend
  on non-deterministic behaviour.
* oam_bug-2 fails on all CGB, AGB, and AGS devices
* cgb_sound-2 test #03 fails on CPU CGB, CPU CGB A, and CPU CGB B

### Mooneye GB acceptance tests

| Test                    | mooneye-gb |
| ----------------------- | ---------- |
| add sp e timing         | :+1:       |
| boot div dmg0           | :x:        |
| boot div dmgABCmgb      | :x:        |
| boot div S              | :x:        |
| boot div2 S             | :x:        |
| boot hwio dmg0          | :x:        |
| boot hwio dmgABCmgb     | :x:        |
| boot hwio S             | :+1:       |
| boot regs dmg0          | :+1:       |
| boot regs dmgABC        | :+1:       |
| boot regs mgb           | :+1:       |
| boot regs sgb           | :+1:       |
| boot regs sgb2          | :+1:       |
| call timing             | :+1:       |
| call timing2            | :+1:       |
| call cc_timing          | :+1:       |
| call cc_timing2         | :+1:       |
| di timing GS            | :+1:       |
| div timing              | :+1:       |
| ei sequence             | :+1:       |
| ei timing               | :+1:       |
| halt ime0 ei            | :+1:       |
| halt ime0 nointr_timing | :+1:       |
| halt ime1 timing        | :+1:       |
| halt ime1 timing2 GS    | :+1:       |
| if ie registers         | :+1:       |
| intr timing             | :+1:       |
| jp timing               | :+1:       |
| jp cc timing            | :+1:       |
| ld hl sp e timing       | :+1:       |
| oam dma_restart         | :+1:       |
| oam dma start           | :+1:       |
| oam dma timing          | :+1:       |
| pop timing              | :+1:       |
| push timing             | :+1:       |
| rapid di ei             | :+1:       |
| ret timing              | :+1:       |
| ret cc timing           | :+1:       |
| reti timing             | :+1:       |
| reti intr timing        | :+1:       |
| rst timing              | :+1:       |

#### Bits (unusable bits in memory and registers)

| Test           | mooneye-gb |
| -------------- | ---------- |
| mem oam        | :+1:       |
| reg f          | :+1:       |
| unused_hwio GS | :+1:       |

#### Instructions

| Test                        | mooneye-gb |
| --------------------------- | ---------- |
| daa                         | :+1:       |

#### Interrupt handling

| Test                        | mooneye-gb |
| --------------------------- | ---------- |
| ie push                     | :+1:       |

#### OAM DMA

| Test                        | mooneye-gb |
| --------------------------- | ---------- |
| basic                       | :+1:       |
| reg_read                    | :+1:       |
| sources GS                  | :+1:       |

#### PPU

| Test                        | mooneye-gb |
| --------------------------- | ---------- |
| hblank ly scx timing GS     | :+1:       |
| intr 1 2 timing GS          | :+1:       |
| intr 2 0 timing             | :+1:       |
| intr 2 mode0 timing         | :+1:       |
| intr 2 mode3 timing         | :+1:       |
| intr 2 oam ok timing        | :+1:       |
| intr 2 mode0 timing sprites | :x:        |
| lcdon timing GS             | :x:        |
| lcdon write timing GS       | :x:        |
| stat irq blocking           | :x:        |
| stat lyc onoff              | :x:        |
| vblank stat intr GS         | :+1:       |

#### Serial

| Test                        | mooneye-gb |
| --------------------------- | ---------- |
| boot sclk align dmgABCmgb   | :x:        |

#### Timer

| Test                 | mooneye-gb |
| -------------------- | ---------- |
| div write            | :+1:       |
| rapid toggle         | :+1:       |
| tim00 div trigger    | :+1:       |
| tim00                | :+1:       |
| tim01 div trigger    | :+1:       |
| tim01                | :+1:       |
| tim10 div trigger    | :+1:       |
| tim10                | :+1:       |
| tim11 div trigger    | :+1:       |
| tim11                | :+1:       |
| tima reload          | :+1:       |
| tima write reloading | :+1:       |
| tma write reloading  | :+1:       |

### Mooneye GB emulator-only tests

#### MBC1

| Test              | mooneye-gb |
| ----------------- | ---------- |
| bits bank1        | :+1:       |
| bits bank2        | :+1:       |
| bits mode         | :+1:       |
| bits ramg         | :+1:       |
| rom 512kb         | :+1:       |
| rom 1Mb           | :+1:       |
| rom 2Mb           | :+1:       |
| rom 4Mb           | :+1:       |
| rom 8Mb           | :+1:       |
| rom 16Mb          | :+1:       |
| ram 64kb          | :+1:       |
| ram 256kb         | :+1:       |
| multicart rom 8Mb | :+1:       |

#### MBC2

| Test              | mooneye-gb |
| ----------------- | ---------- |
| bits ramg         | :+1:       |
| bits romb         | :+1:       |
| bits unused       | :+1:       |
| rom 512kb         | :+1:       |
| rom 1Mb           | :+1:       |
| rom 2Mb           | :+1:       |
| ram               | :+1:       |

#### MBC5

| Test              | mooneye-gb |
| ----------------- | ---------- |
| rom 512kb         | :+1:       |
| rom 1Mb           | :+1:       |
| rom 2Mb           | :+1:       |
| rom 4Mb           | :+1:       |
| rom 8Mb           | :+1:       |
| rom 16Mb          | :+1:       |
| rom 32Mb          | :+1:       |
| rom 64Mb          | :+1:       |

### Mooneye GB manual tests

| Test            | mooneye-gb |
| --------------- | ---------- |
| sprite priority | :+1:       |

### Mooneye GB misc tests

| Test              | mooneye-gb |
| ---------------   | ---------- |
| boot div A        |            |
| boot div cgb0     |            |
| boot div cgbABCDE |            |
| boot hwio C       |            |
| boot regs A       |            |
| boot regs cgb     |            |

#### Bits

| Test          | mooneye-gb |
| ------------- | ---------- |
| unused hwio C |            |

#### PPU

| Test               | mooneye-gb |
| ------------------ | ---------- |
| vblank stat intr C |            |

# License and copyright

Mooneye GB is licensed under GPLv3+.
Copyright (C) 2014-2020 Joonas Javanainen <joonas.javanainen@gmail.com>
