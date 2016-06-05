# Mooneye GB

Mooneye GB is a Game Boy emulator written in Rust.

[![Build Status](https://travis-ci.org/Gekkio/mooneye-gb.svg?branch=master)](https://travis-ci.org/Gekkio/mooneye-gb)

The main goals of this project are accuracy and documentation. Some existing emulators are very accurate (Gambatte, BGB >= 1.5) but are not documented very clearly, so they are not that good references for emulator developers. I want this project to document as clearly as possible *why* certain behaviour is emulated in a certain way. This also means writing a lot of test ROMs to figure out corner cases and precise behaviour on real hardware.

Non-goals:

* CGB (Game Boy Color) support. It would be nice, but I want to make the normal Game Boy support extremely robust first.
* A good debugger. A primitive debugger exists for development purposes, and it is enough.
* A user interface. Building native UIs with Rust is a bit painful at the moment.

**Warning**:

* Project is WIP
* Doesn't work properly without a boot ROM

## Accuracy

This project already passes Blargg's cpu\_instrs, instr\_timing, and mem\_timing-2 tests.

Things that need significant work:

* GPU emulation accuracy
* APU emulation in general

There's tons of documentation and tons of emulators in the internet, but in the end I only trust real hardware. I follow a fairly "scientific" process when developing emulation for a feature:

1. Think of different ways how it might behave on real hardware
2. Make a hypothesis based on the most probable behaviour
3. Write a test ROM for such behaviour
4. Run the test ROM on real hardware. If the test ROM made an invalid hypothesis, go back to 1.
5. Replicate the behaviour in the emulator

All test ROMs are manually run with these devices:

| Device              | Model    | Mainboard    | CPU         |
| ------------------- | -------- | ------------ | ----------- |
| Game Boy            | DMG-01   | DMG-CPU-04   | DMG CPU B   |
| Game Boy Pocket     | MGB-001  | MGB-ECPU-01  | CPU MGB     |
| Game Boy Pocket     | MGB-001  | MGB-LCPU-01  | CPU MGB     |
| Super Game Boy      | SNSP-027 | SGB-R-10     | SGB-CPU-01  |
| Super Game Boy      | SHVC-027 | SGB-R-10     | SGB-CPU-01  |
| Super Game Boy 2    | SHVC-042 | SHVC-SGB2-01 | CPU SGB2    |
| Game Boy Color      | CGB-001  | CGB-CPU-03   | CPU CGB C   |
| Game Boy Color      | CGB-001  | CGB-CPU-04   | CPU CGB D   |
| Game Boy Color      | CGB-001  | CGB-CPU-05   | CPU CGB D   |
| Game Boy Advance    | AGB-001  | AGB-CPU-10   | CPU AGB A   |
| Game Boy Advance SP | AGS-001  | C/AGS-CPU-21 | CPU AGB B E |
| Game Boy Advance SP | AGS-101  | C/AGT-CPU-01 | CPU AGB B E |

These devices will also be used, but results for old tests have not yet been verified:

| Device              | Model    | Mainboard    | CPU         |
| ------------------- | -------- | ------------ | ----------- |
| Game Boy            | DMG-01   | DMG-CPU-02   | DMG CPU A   |
| Game Boy            | DMG-01   | DMG-CPU-06   | DMG CPU B   |
| Game Boy Color      | CGB-001  | CGB-CPU-02   | CPU CGB B   |
| Game Boy Color      | CGB-001  | CGB-CPU-06   | CPU CGB E   |
| Game Boy Advance    | AGB-001  | AGB-CPU-01   | CPU AGB     |


**For now, the focus is on DMG/MGB/SGB/SGB2 emulation, so not all tests pass on CGB/AGB/AGS or emulators emulating those devices.**

## Performance

**Always compile in release mode if you care about performance!**

On a i7-3770K desktop machine I can usually run ROMs with 2000 - 4000% speed. Without optimizations the speed drops to 150 - 200%, which is still fine for development purposes.

Raspberry Pi with X11 desktop works but is too slow because there is no OpenGL acceleration.

The emulator is runnable on Android, but cross-compiling and packaging is a huge pain and touch controls would have to be implemented, so I'm not supporting Android at the moment.

## Running the emulator

### GUI

1. `cargo run --release`
2. Follow the instructions

### Command-line
1. Acquire a Game Boy bootrom, and put it to `$HOME/.mooneye-gb/dmg_boot.bin`
2. `cargo build --release`
3. `cargo run --release -- PATH_TO_GAMEBOY_ROM`

On Windows, also download an SDL2 package containing SDL2.dll, and put it to `target/debug` and `target/release`.

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

## Accuracy comparison

Versions used:

* mooneye-gb (master)
* BGB 1.5.2
* Gambatte 2015-03-23 (f9fb003)
* GiiBiiAdvance 2015-05-16 (dbf669a)
* Higan v098 (in Game Boy mode, except for SGB/SGB2-specific test ROMs)
* KiGB 2.05
* MESS 0.163

### Blargg's tests

| Test              | mooneye-gb | BGB  | Gambatte | GiiBiiAdvance | Higan | KiGB | MESS |
| ----------------- | ---------- | ---- | -------- | ------------- | ----- | ---- | ---- |
| cpu instrs        | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :x:  | :+1: |
| dmg sound 2       | :x:        | :+1: | :+1:     | :x:           | :x:   | :x:  | :x:  |
| instr timing      | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :x:  | :+1: |
| mem timing 2      | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :x:  | :+1: |
| oam bug 2         | :x:        | :x:  | :x:      | :x:           | :x:   | :x:  | :x:  |

### Mooneye GB acceptance tests

| Test                    | mooneye-gb | BGB  | Gambatte | GiiBiiAdvance | Higan | KiGB | MESS |
| ----------------------- | ---------- | ---- | -------- | ------------- | ------| ---- | ---- |
| add sp e timing         | :+1:       | :x:  | :+1:     | :x:           | :+1:  | :x:  | :x:  |
| boot hwio G             | :+1:       | :+1: | :+1:     | :x:           | :+1:  | :x:  | :x:  |
| boot regs dmg           | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :+1: | :+1: |
| call timing             | :+1:       | :x:  | :+1:     | :x:           | :+1:  | :x:  | :x:  |
| call timing2            | :+1:       | :x:  | :+1:     | :x:           | :+1:  | :x:  | :x:  |
| call cc_timing          | :+1:       | :x:  | :+1:     | :x:           | :+1:  | :x:  | :x:  |
| call cc_timing2         | :+1:       | :x:  | :+1:     | :x:           | :+1:  | :x:  | :x:  |
| di timing GS            | :+1:       | :+1: | :+1:     | :x:           | :+1:  | :+1: | :+1: |
| div timing              | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :x:  | :+1: |
| ei timing               | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :+1: | :+1: |
| halt ime0 ei            | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :+1: | :+1: |
| halt ime0 nointr_timing | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :x:  | :x:  |
| halt ime1 timing        | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :x:  | :+1: |
| halt ime1 timing2 GS    | :+1:       | :+1: | :+1:     | :x:           | :+1:  | :x:  | :+1: |
| if ie registers         | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :x:  | :+1: |
| intr timing             | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :x:  | :+1: |
| jp timing               | :+1:       | :x:  | :+1:     | :x:           | :+1:  | :x:  | :x:  |
| jp cc timing            | :+1:       | :x:  | :+1:     | :x:           | :+1:  | :x:  | :x:  |
| ld hl sp e timing       | :+1:       | :x:  | :+1:     | :x:           | :+1:  | :x:  | :x:  |
| oam dma_restart         | :+1:       | :x:  | :+1:     | :x:           | :+1:  | :x:  | :x:  |
| oam dma start           | :+1:       | :x:  | :+1:     | :x:           | :+1:  | :x:  | :x:  |
| oam dma timing          | :+1:       | :x:  | :+1:     | :x:           | :+1:  | :x:  | :x:  |
| pop timing              | :+1:       | :x:  | :+1:     | :+1:          | :+1:  | :x:  | :+1: |
| push timing             | :+1:       | :x:  | :x:      | :x:           | :+1:  | :x:  | :x:  |
| rapid di ei             | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :+1: | :+1: |
| ret timing              | :+1:       | :x:  | :+1:     | :x:           | :+1:  | :x:  | :x:  |
| ret cc timing           | :+1:       | :x:  | :+1:     | :x:           | :+1:  | :x:  | :x:  |
| reti timing             | :+1:       | :x:  | :+1:     | :x:           | :+1:  | :x:  | :x:  |
| reti intr timing        | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :+1: | :+1: |
| rst timing              | :+1:       | :x:  | :x:      | :x:           | :+1:  | :x:  | :x:  |

#### Bits (unusable bits in memory and registers)

| Test           | mooneye-gb | BGB  | Gambatte | GiiBiiAdvance | Higan | KiGB | MESS |
| -------------- | ---------- | ---- | -------- | ------------- | ------| ---- | ---- |
| mem oam        | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :+1: | :+1: |
| reg f          | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :+1: | :+1: |
| unused_hwio GS | :+1:       | :x:  | :+1:     | :x:           | :x:   | :x:  | :x:  |

#### GPU

| Test                        | mooneye-gb | BGB  | Gambatte | GiiBiiAdvance | Higan | KiGB | MESS |
| --------------------------- | ---------- | ---- | -------- | ------------- | ------| ---- | ---- |
| hblank ly scx timing GS     | :+1:       | :x:  | :x:      | :x:           | :x:   | :x:  | :x:  |
| intr 1 2 timing GS          | :+1:       | :+1: | :+1:     | :x:           | :+1:  | :x:  | :+1: |
| intr 2 0 timing             | :+1:       | :+1: | :x:      | :x:           | :+1:  | :x:  | :x:  |
| intr 2 mode0 timing         | :+1:       | :+1: | :x:      | :x:           | :x:   | :x:  | :x:  |
| intr 2 mode3 timing         | :+1:       | :+1: | :x:      | :x:           | :x:   | :x:  | :x:  |
| intr 2 oam ok timing        | :+1:       | :+1: | :x:      | :x:           | :x:   | :x:  | :x:  |
| intr 2 mode0 timing sprites | :x:        | :x:  | :x:      | :x:           | :x:   | :x:  | :x:  |
| stat irq blocking           | :x:        | :x:  | :+1:     | :x:           | :x:   | :x:  | :x:  |
| vblank stat intr GS         | :+1:       | :+1: | :x:      | :+1:          | :+1:  | :x:  | :x:  |

#### Timer

| Test                 | mooneye-gb | BGB  | Gambatte | GiiBiiAdvance | Higan  | KiGB | MESS |
| -------------------- | ---------- | ---- | -------- | ------------- | ------ | ---- | ---- |
| div write            | :x:        | :+1: | :x:      | :+1:          | :+1:   | :x:  | :x:  |
| rapid toggle         | :x:        | :x:  | :x:      | :+1:          | :x:    | :x:  | :+1: |
| tim00 div trigger    | :+1:       | :x:  | :+1:     | :+1:          | :x:    | :x:  | :x:  |
| tim00                | :x:        | :+1: | :x:      | :+1:          | :+1:   | :x:  | :x:  |
| tim01 div trigger    | :x:        | :+1: | :x:      | :+1:          | :x:    | :x:  | :x:  |
| tim01                | :+1:       | :+1: | :+1:     | :+1:          | :+1:   | :x:  | :x:  |
| tim10 div trigger    | :x:        | :+1: | :x:      | :+1:          | :x:    | :x:  | :+1: |
| tim10                | :x:        | :+1: | :x:      | :+1:          | :+1:   | :x:  | :x:  |
| tim11 div trigger    | :+1:       | :x:  | :x:      | :+1:          | :x:    | :x:  | :x:  |
| tim11                | :x:        | :+1: | :x:      | :+1:          | :+1:   | :x:  | :x:  |
| tima reload          | :x:        | :x:  | :x:      | :+1:          | :x:    | :x:  | :x:  |
| tima write reloading | :x:        | :x:  | :x:      | :+1:          | :x:    | :x:  | :x:  |
| tma write reloading  | :x:        | :x:  | :x:      | :+1:          | :x:    | :x:  | :x:  |

### Mooneye GB emulator-only tests

| Test              | mooneye-gb | BGB  | Gambatte | GiiBiiAdvance | Higan | KiGB | MESS |
| ----------------- | ---------- | ---- | -------- | ------------- | ----- | ---- | ---- |
| mbc1 rom 4banks   | :+1:       | :x:  | :+1:     | :+1:          | :+1:  | :x:  | :+1: |

### Mooneye GB manual tests

| Test            | mooneye-gb | BGB  | Gambatte | GiiBiiAdvance | Higan | KiGB | MESS |
| --------------- | ---------- | ---- | -------- | ------------- | ----- | ---- | ---- |
| sprite priority | :+1:       | :+1: | :+1:     | :x:           | :+1:  | :x:  | :x:  |

### Mooneye GB misc tests

| Test            | mooneye-gb | BGB  | Gambatte | GiiBiiAdvance | Higan | KiGB | MESS |
| --------------- | ---------- | ---- | -------- | ------------- | ----- | ---- | ---- |
| boot hwio C     |            | :+1: |          | :x:           |       | :x:  | :x:  |
| boot hwio S     |            | :x:  |          | :x:           | :+1:  | :x:  | :x:  |
| boot regs A     |            | :x:  |          | :x:           |       | :x:  |      |
| boot regs cgb   |            | :+1: |          | :x:           |       | :x:  | :+1: |
| boot regs mgb   |            | :+1: |          | :+1:          |       |      | :+1: |
| boot regs sgb   |            | :x:  |          | :+1:          | :+1:  | :x:  | :+1: |
| boot regs sgb2  |            | :x:  |          | :+1:          | :x:   | :x:  |      |

#### Bits

| Test          | mooneye-gb | BGB  | Gambatte | GiiBiiAdvance | Higan | KiGB | MESS |
| ------------- | ---------- | ---- | -------- | ------------- | ----- | ---- | ---- |
| unused hwio C |            | :x:  |          | :x:           |       | :x:  | :x:  |

#### GPU

| Test               | mooneye-gb | BGB  | Gambatte | GiiBiiAdvance | Higan | KiGB | MESS |
| ------------------ | ---------- | ---- | -------- | ------------- | ----- | ---- | ---- |
| vblank stat intr C |            | :x:  |          | :x:           |       | :x:  | :x:  |

### Test naming

Some tests are expected to pass only a single type of hardware:

* dmg = Game Boy
* mgb = Game Boy Pocket
* sgb = Super Game Boy
* sgb2 = Super Game Boy 2
* cgb = Game Boy Color
* agb = Game Boy Advance
* ags = Game Boy Advance SP

In general, hardware can be divided on to a couple of groups based on their
behaviour. Some tests are expected to pass on a single or multiple groups:

* G = dmg+mgb
* S = sgb+sgb2
* C = cgb+agb+ags
* A = agb+ags

For example, a test with GS in the name is expected to pass on dmg+mgb +
sgb+sgb2.

# License and copyright

Mooneye GB is licensed under GPLv3+.
Copyright (C) 2014-2016 Joonas Javanainen <joonas.javanainen@gmail.com>
