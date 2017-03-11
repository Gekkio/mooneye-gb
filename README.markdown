# Mooneye GB

Mooneye GB is a Game Boy research project and emulator written in Rust.

[![Build Status](https://travis-ci.org/Gekkio/mooneye-gb.svg?branch=master)](https://travis-ci.org/Gekkio/mooneye-gb)

The main goals of this project are accuracy and documentation. Some existing
emulators are very accurate (Gambatte, BGB >= 1.5) but are not documented very
clearly, so they are not that good references for emulator developers. I want
this project to document as clearly as possible *why* certain behaviour is
emulated in a certain way. This also means writing a lot of test ROMs to figure
out corner cases and precise behaviour on real hardware.

[Binary test ROMs are available here](https://gekkio.fi/files/mooneye-gb/latest/)
in a zip package and also as individual .gb files. They are automatically
built and deployed whenever there's new changes in the master branch.

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

## Hardware testing

There's tons of documentation and tons of emulators in the internet, but in the
end I only trust real hardware. I follow a fairly "scientific" process when
developing emulation for a feature:

1. Think of different ways how it might behave on real hardware
2. Make a hypothesis based on the most probable behaviour
3. Write a test ROM for such behaviour
4. Run the test ROM on real hardware. If the test ROM made an invalid
   hypothesis, go back to 1.
5. Replicate the behaviour in the emulator

All test ROMs are manually run with these devices:

| Device              | Model    | Mainboard    | CPU              | Detailed information                                                                  |
| ------------------- | -------- | ------------ | ---------------- | ---------------                                                                       |
| Game Boy            | DMG-01   | DMG-CPU-01   | DMG-CPU          |                                                                                       |
| Game Boy            | DMG-01   | DMG-CPU-02   | DMG-CPU A        |                                                                                       |
| Game Boy            | DMG-01   | DMG-CPU-04   | DMG-CPU B        |                                                                                       |
| Game Boy            | DMG-01   | DMG-CPU-06   | DMG-CPU C        |                                                                                       |
| Game Boy            | DMG-01   | DMG-CPU-07   | DMG-CPU X (blob) |                                                                                       |
| Super Game Boy      | SHVC-027 | SGB-R-10     | SGB-CPU-01       | [SGB Unit #2 \[gekkio\]](https://gbhwdb.gekkio.fi/consoles/sgb/gekkio-2.html)   |
| Game Boy Pocket     | MGB-001  | MGB-CPU-01   | CPU MGB          |                                                                                       |
| Super Game Boy 2    | SHVC-042 | SHVC-SGB2-01 | CPU SGB2         | [SGB2 Unit #1 \[gekkio\]](https://gbhwdb.gekkio.fi/consoles/sgb2/gekkio-1.html) |
| Game Boy Color      | CGB-001  | CGB-CPU-01   | CPU CGB          |                                                                                       |
| Game Boy Color      | CGB-001  | CGB-CPU-02   | CPU CGB B        |                                                                                       |
| Game Boy Color      | CGB-001  | CGB-CPU-03   | CPU CGB C        |                                                                                       |
| Game Boy Color      | CGB-001  | CGB-CPU-05   | CPU CGB D        |                                                                                       |
| Game Boy Color      | CGB-001  | CGB-CPU-06   | CPU CGB E        |                                                                                       |
| Game Boy Advance    | AGB-001  | AGB-CPU-01   | CPU AGB          |                                                                                       |
| Game Boy Advance    | AGB-001  | AGB-CPU-10   | CPU AGB A        |                                                                                       |
| Game Boy Advance SP | AGS-001  | C/AGS-CPU-01 | CPU AGB B        |                                                                                       |
| Game Boy Advance SP | AGS-001  | C/AGS-CPU-21 | CPU AGB B E      |                                                                                       |

These devices will also be used, but results for old tests have not yet been
verified:

| Device              | Model    | Mainboard    | CPU              | Detailed information                                                                  |
| ------------------- | -------- | ------------ | -----------      | ----                                                                                  |
| Game Boy Color      | CGB-001  | CGB-CPU-01   | CPU CGB A        |                                                                                       |

### Additional devices

I also have access to more devices with different mainboard revisions, but I
think the CPU revision is all that matters if we study the behaviour and not
analog characteristics (e.g. audio filtering). Even if audio sounded different
between two units with the same CPU revision but different mainboard revisions,
I'd expect the difference to be caused by individual device variation or
different revisions of support chips (e.g. RAM/AMP/REG).

The main "test fleet" is already very big, so I will only use these devices if
there's evidence of behaviour that depends on mainboard revision or individual
units.

| Device              | Model    | Mainboard    | CPU              | Detailed information                                                                  |
| ------------------- | -------- | ------------ | -----------      | ----                                                                                  |
| Game Boy            | DMG-01   | DMG-CPU-03   | DMG-CPU B        |                                                                                       |
| Game Boy            | DMG-01   | DMG-CPU-05   | DMG-CPU B        |                                                                                       |
| Game Boy            | DMG-01   | DMG-CPU-06   | DMG-CPU B        |                                                                                       |
| Game Boy            | DMG-01   | DMG-CPU-08   | DMG-CPU X (blob) |                                                                                       |
| Super Game Boy      | SNSP-027 | SGB-R-10     | SGB-CPU-01       | [SGB Submission #7 \[gekkio\]](https://gbhwdb.gekkio.fi/consoles/sgb/gekkio-7.html)   |
| Game Boy Pocket     | MGB-001  | MGB-ECPU-01  | CPU MGB          |                                                                                       |
| Game Boy Pocket     | MGB-001  | MGB-LCPU-01  | CPU MGB          |                                                                                       |
| Game Boy Pocket     | MGB-001  | MGB-LCPU-02  | CPU MGB          |                                                                                       |
| Game Boy Color      | CGB-001  | CGB-CPU-04   | CPU CGB D        |                                                                                       |
| Game Boy Advance    | AGB-001  | AGB-CPU-02   | CPU AGB          |                                                                                       |
| Game Boy Advance    | AGB-001  | AGB-CPU-03   | CPU AGB A        |                                                                                       |
| Game Boy Advance    | AGB-001  | AGB-CPU-04   | CPU AGB A        |                                                                                       |
| Game Boy Advance SP | AGS-001  | C/AGS-CPU-10 | CPU AGB B        |                                                                                       |
| Game Boy Advance SP | AGS-001  | C/AGS-CPU-11 | CPU AGB B        |                                                                                       |
| Game Boy Advance SP | AGS-001  | C/AGS-CPU-30 | CPU AGB B E      |                                                                                       |
| Game Boy Advance SP | AGS-101  | C/AGT-CPU-01 | CPU AGB B E      |                                                                                       |

I'm still looking for the following mainboards, but these are probably not
required for reverse engineering:

* SGB-R-01
* SGB-N-01
* SGB-N-10
* C/AGS-CPU-20

**For now, the focus is on DMG/MGB/SGB/SGB2 emulation, so not all tests pass on
CGB/AGB/AGS or emulators emulating those devices.**

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

### GUI

1. `cargo run --release`
2. Follow the instructions

### Command-line
1. Acquire a Game Boy bootrom, and put it to `$HOME/.mooneye-gb/dmg_boot.bin`
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

## Accuracy comparison

Versions used:

* mooneye-gb (master)
* BGB 1.5.2
* Gambatte 2015-03-23 (f9fb003)
* Higan v098 (in Game Boy mode, except for SGB/SGB2-specific test ROMs)
* MESS 0.179

Symbols:

* :+1: pass
* :x: fail
* :o: pass with caveats, see notes

These emulators are omitted:

* KiGB. This emulator has bold claims about accuracy and compatibility.
  However, version 2.05 was tested and it barely passed any tests at all.
  See the [accuracy table from history](https://github.com/Gekkio/mooneye-gb/blob/401b8b1a4834459df18c8cf781c37a30b2b7848e/README.markdown#accuracy-comparison)
* GiiBiiAdvance. This emulator seems to be unmaintained, but you can check
  the [accuracy table from history](https://github.com/Gekkio/mooneye-gb/blob/6f18d5094fc820b48ac5371c1fb07b78b47e67e8/README.markdown#accuracy-comparison) for old results.

### Blargg's tests

| Test              | mooneye-gb | BGB  | Gambatte | Higan | MESS |
| ----------------- | ---------- | ---- | -------- | ----- | ---- |
| cpu instrs        | :+1:       | :+1: | :+1:     | :+1:  | :+1: |
| dmg sound 2       | :x:        | :+1: | :+1:     | :x:   | :o:  |
| instr timing      | :+1:       | :+1: | :+1:     | :+1:  | :+1: |
| mem timing 2      | :+1:       | :+1: | :+1:     | :+1:  | :+1: |
| oam bug 2         | :x:        | :x:  | :x:      | :x:   | :x:  |

Notes:

* cpu_instrs fails on MGB/SGB2 hardware and emulators emulating them correctly.
  The ROM incorrectly detects the device as CGB, and attempts to perform a CPU
  speed change which causes a freeze (STOP instruction with joypad disabled)
* dmg_sound-2 test #10 fails on DMG-CPU A, DMG-CPU C, and MGB
* MESS incorrectly passes dmg_sound-2 on MGB

### Mooneye GB acceptance tests

| Test                    | mooneye-gb | BGB  | Gambatte | Higan  | MESS |
| ----------------------- | ---------- | ---- | -------- | ------ | ---- |
| add sp e timing         | :+1:       | :x:  | :+1:     | :+1:   | :+1: |
| boot hwio dmg0          | :x:        | :o:  |          |        | :+1: |
| boot hwio dmgABCXmgb    | :x:        | :+1: | :+1:     | :x:    | :+1: |
| boot hwio S             | :+1:       | :o:  |          | :+1:   | :x:  |
| boot regs dmg0          | :+1:       | :o:  |          |        | :+1: |
| boot regs dmgABCX       | :+1:       | :+1: | :+1:     | :+1:   | :+1: |
| boot regs mgb           | :+1:       | :o:  |          |        | :+1: |
| boot regs sgb           | :+1:       | :o:  |          | :+1:   | :+1: |
| boot regs sgb2          | :+1:       | :o:  |          | :x:    | :+1: |
| call timing             | :+1:       | :x:  | :+1:     | :+1:   | :+1: |
| call timing2            | :+1:       | :x:  | :+1:     | :+1:   | :+1: |
| call cc_timing          | :+1:       | :x:  | :+1:     | :+1:   | :+1: |
| call cc_timing2         | :+1:       | :x:  | :+1:     | :+1:   | :+1: |
| di timing GS            | :+1:       | :+1: | :+1:     | :+1:   | :+1: |
| div timing              | :+1:       | :+1: | :+1:     | :+1:   | :+1: |
| ei timing               | :+1:       | :+1: | :+1:     | :+1:   | :+1: |
| halt ime0 ei            | :+1:       | :+1: | :+1:     | :+1:   | :+1: |
| halt ime0 nointr_timing | :+1:       | :+1: | :+1:     | :+1:   | :+1: |
| halt ime1 timing        | :+1:       | :+1: | :+1:     | :+1:   | :+1: |
| halt ime1 timing2 GS    | :+1:       | :+1: | :+1:     | :+1:   | :+1: |
| if ie registers         | :+1:       | :+1: | :+1:     | :+1:   | :+1: |
| intr timing             | :+1:       | :+1: | :+1:     | :+1:   | :+1: |
| jp timing               | :+1:       | :x:  | :+1:     | :+1:   | :+1: |
| jp cc timing            | :+1:       | :x:  | :+1:     | :+1:   | :+1: |
| ld hl sp e timing       | :+1:       | :x:  | :+1:     | :+1:   | :+1: |
| oam dma_restart         | :+1:       | :x:  | :+1:     | :+1:   | :+1: |
| oam dma start           | :+1:       | :x:  | :+1:     | :+1:   | :+1: |
| oam dma timing          | :+1:       | :x:  | :+1:     | :+1:   | :+1: |
| pop timing              | :+1:       | :x:  | :+1:     | :+1:   | :+1: |
| push timing             | :+1:       | :x:  | :x:      | :+1:   | :+1: |
| rapid di ei             | :+1:       | :+1: | :+1:     | :+1:   | :+1: |
| ret timing              | :+1:       | :x:  | :+1:     | :+1:   | :+1: |
| ret cc timing           | :+1:       | :x:  | :+1:     | :+1:   | :+1: |
| reti timing             | :+1:       | :x:  | :+1:     | :+1:   | :+1: |
| reti intr timing        | :+1:       | :+1: | :+1:     | :+1:   | :+1: |
| rst timing              | :+1:       | :x:  | :x:      | :+1:   | :+1: |

Notes:

* BGB passes most boot tests only if you explicitly enable boot ROMs and give it the right one.
  This makes sense for DMG0, MGB, and SGB2 because they are not selectable, but SGB should work
  without boot ROMs out of the box.

#### Bits (unusable bits in memory and registers)

| Test           | mooneye-gb | BGB  | Gambatte | Higan | MESS |
| -------------- | ---------- | ---- | -------- | ------| ---- |
| mem oam        | :+1:       | :+1: | :+1:     | :+1:  | :+1: |
| reg f          | :+1:       | :+1: | :+1:     | :+1:  | :+1: |
| unused_hwio GS | :+1:       | :x:  | :+1:     | :x:   | :+1: |

#### GPU

| Test                        | mooneye-gb | BGB  | Gambatte | Higan | MESS |
| --------------------------- | ---------- | ---- | -------- | ------| ---- |
| hblank ly scx timing GS     | :+1:       | :x:  | :x:      | :x:   | :+1: |
| intr 1 2 timing GS          | :+1:       | :+1: | :+1:     | :+1:  | :+1: |
| intr 2 0 timing             | :+1:       | :+1: | :x:      | :+1:  | :+1: |
| intr 2 mode0 timing         | :+1:       | :+1: | :x:      | :x:   | :+1: |
| intr 2 mode3 timing         | :+1:       | :+1: | :x:      | :x:   | :+1: |
| intr 2 oam ok timing        | :+1:       | :+1: | :x:      | :x:   | :+1: |
| intr 2 mode0 timing sprites | :x:        | :x:  | :x:      | :x:   | :+1: |
| stat irq blocking           | :x:        | :x:  | :+1:     | :x:   | :+1: |
| vblank stat intr GS         | :+1:       | :+1: | :x:      | :+1:  | :+1: |

#### Timer

| Test                 | mooneye-gb | BGB  | Gambatte | Higan  | MESS |
| -------------------- | ---------- | ---- | -------- | ------ | ---- |
| div write            | :x:        | :+1: | :x:      | :+1:   | :+1: |
| rapid toggle         | :x:        | :x:  | :x:      | :x:    | :+1: |
| tim00 div trigger    | :+1:       | :x:  | :+1:     | :x:    | :+1: |
| tim00                | :x:        | :+1: | :x:      | :+1:   | :+1: |
| tim01 div trigger    | :x:        | :+1: | :x:      | :x:    | :+1: |
| tim01                | :+1:       | :+1: | :+1:     | :+1:   | :+1: |
| tim10 div trigger    | :x:        | :+1: | :x:      | :x:    | :+1: |
| tim10                | :x:        | :+1: | :x:      | :+1:   | :+1: |
| tim11 div trigger    | :+1:       | :x:  | :x:      | :x:    | :+1: |
| tim11                | :x:        | :+1: | :x:      | :+1:   | :+1: |
| tima reload          | :x:        | :x:  | :x:      | :x:    | :+1: |
| tima write reloading | :x:        | :x:  | :x:      | :x:    | :+1: |
| tma write reloading  | :x:        | :x:  | :x:      | :x:    | :+1: |

### Mooneye GB emulator-only tests

#### MBC1

| Test              | mooneye-gb | BGB  | Gambatte | Higan | MESS |
| ----------------- | ---------- | ---- | -------- | ----- | ---- |
| rom 512Kb         | :+1:       | :x:  | :+1:     | :+1:  | :+1: |
| rom 1Mb           | :+1:       | :x:  | :+1:     | :+1:  | :+1: |
| rom 2Mb           | :+1:       | :x:  | :+1:     | :+1:  | :+1: |
| rom 4Mb           | :+1:       | :+1: | :+1:     | :+1:  | :+1: |
| rom 8Mb           | :+1:       | :x:  | :x:      | :x:   | :+1: |
| rom 16Mb          | :+1:       | :x:  | :x:      | :x:   | :+1: |
| ram 64Kb          | :+1:       | :+1: | :+1:     | :+1:  | :+1: |
| ram 256Kb         | :+1:       | :+1: | :x:      | :x:   | :+1: |
| multicart rom 8Mb | :+1:       |      |          | :x:   | :+1: |

Notes:

* Most emulators don't support MBC1 multicart ROMs at all
* Higan requires manual manifest file creation to trigger MBC1 multicart mode,
  but doesn't pass the test.

### Mooneye GB manual tests

| Test            | mooneye-gb | BGB  | Gambatte | Higan | MESS |
| --------------- | ---------- | ---- | -------- | ----- | ---- |
| sprite priority | :+1:       | :+1: | :+1:     | :+1:  | :x:  |

### Mooneye GB misc tests

| Test            | mooneye-gb | BGB  | Gambatte | Higan | MESS |
| --------------- | ---------- | ---- | -------- | ----- | ---- |
| boot hwio C     |            | :+1: |          |       | :x:  |
| boot regs A     |            | :x:  |          |       |      |
| boot regs cgb   |            | :+1: |          |       | :+1: |

#### Bits

| Test          | mooneye-gb | BGB  | Gambatte | Higan | MESS |
| ------------- | ---------- | ---- | -------- | ----- | ---- |
| unused hwio C |            | :x:  |          |       | :x:  |

#### GPU

| Test               | mooneye-gb | BGB  | Gambatte | Higan | MESS |
| ------------------ | ---------- | ---- | -------- | ----- | ---- |
| vblank stat intr C |            | :x:  |          |       | :+1: |

### Test naming

Some tests are expected to pass only a single model:

* dmg = Game Boy
* mgb = Game Boy Pocket
* sgb = Super Game Boy
* sgb2 = Super Game Boy 2
* cgb = Game Boy Color
* agb = Game Boy Advance
* ags = Game Boy Advance SP

In addition to model differences, CPU revisions can affect the behaviour.
Revision 0 refers always to the initial version of a CPU (e.g. CPU CGB). AGB
and AGS use the same CPU models.  The following CPU models have several
revisions:

* DMG: 0, A, B, C, X (blob)
* CGB: 0, A, B, C, D, E
* AGB: 0, A, B, B E. Revision E also exists, but only in Game Boy Micro (OXY)
  so it is out of this project's scope.

In general, hardware can be divided to a couple of groups based on their
behaviour. Some tests are expected to pass on a single or multiple groups:

* G = dmg+mgb
* S = sgb+sgb2
* C = cgb+agb+ags
* A = agb+ags

For example, a test with GS in the name is expected to pass on dmg+mgb +
sgb+sgb2.

# License and copyright

Mooneye GB is licensed under GPLv3+.
Copyright (C) 2014-2017 Joonas Javanainen <joonas.javanainen@gmail.com>

The test framework and hardware tests under `tests/` are licensed
under MIT.
Copyright (C) 2014-2017 Joonas Javanainen <joonas.javanainen@gmail.com>
