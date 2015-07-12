# Mooneye GB

Mooneye GB is a Gameboy emulator written in Rust.

The main goals of this project are accuracy and documentation. Some existing emulators are very accurate (Gambatte, Gameboy Online, BGB >= 1.5) but are not documented very clearly, so they are not that good references for emulator developers. I want this project to document as clearly as possible *why* certain behaviour is emulated in a certain way. This also means writing a lot of test ROMs to figure out corner cases and precise behaviour on real hardware.

Non-goals:

* CGB (Color Gameboy) support. It would be nice, but I want to make the normal Gameboy support extremely robust first.
* A good debugger. A primitive debugger exists for development purposes, and it is enough.
* A user interface. Building native UIs with Rust is a bit painful at the moment.

**Warning**:

* Project is WIP
* Doesn't work properly without a boot ROM

## Accuracy

This project already passes Blargg's cpu\_instrs, instr\_timing, and mem\_timing-2 tests.

Things that need significant work:

* GPU emulation accuracy
* APU emulation in general (Blargg's dmg_sound-2 works fairly well, but that's just the beginning)

There's tons of documentation and tons of emulators in the internet, but in the end I only trust real hardware. I follow a fairly "scientific" process when developing emulation for a feature:

1. Think of different ways how it might behave on real hardware
2. Make a hypothesis based on the most probable behaviour
3. Write a test ROM for such behaviour
4. Run the test ROM on real hardware. If the test ROM made an invalid hypothesis, go back to 1.
5. Replicate the behaviour in the emulator

All test ROMs are manually run with a Gameboy Pocket (model MGB-001), Gameboy Color (model CGB-001) and a Gameboy Advance SP (model AGS-101).

## Performance

**Always compile in release mode if you care about performance!**

On a i7-3770K desktop machine I can usually run ROMs with 2000 - 4000% speed. Without optimizations the speed drops to 150 - 200%, which is still fine for development purposes.

Raspberry Pi with X11 desktop works but is too slow because there is no OpenGL acceleration.

The emulator is runnable on Android, but cross-compiling and packaging is a huge pain and touch controls would have to be implemented, so I'm not supporting Android at the moment.

## Running the emulator

1. Acquire a Gameboy bootrom, and put it to `~/.mooneye-gb/boot.bin`
2. `cargo build --release`
3. `cargo run --release -- PATH_TO_GAMEBOY_ROM`

### Gameboy keys

| Gameboy | Key        |
| ------- | ---------- |
| Dpad    | Arrow keys |
| A       | Z          |
| B       | X          |
| Start   | Return     |
| Select  | Backspace  |

### Other keys

| Function     | Key       |
| ------------ | --------- |
| Fast forward | Shift     |
| Debug break  | Home      |
| Debug step   | Page Down |
| Debug run    | End       |

## Accuracy comparison

Versions used:

* mooneye-gb (master)
* BGB 1.5.1
* Gambatte 2015-03-23 (f9fb003)
* GiiBiiAdvance 2015-05-16 (dbf669a)
* Higan 0.94
* KiGB 2.05

### Blargg's tests

| Test              | mooneye-gb | BGB  | Gambatte | GiiBiiAdvance | Higan | KiGB |
| ----------------- | ---------- | ---- | -------- | ------------- | ----- | ---- |
| cpu instrs        | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :x:  |
| dmg sound 2       | :x:        | :+1: | :+1:     | :x:           | :x:   | :x:  |
| instr timing      | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :x:  |
| mem timing 2      | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :x:  |
| oam bug 2         | :x:        | :x:  | :x:      | :x:           | :x:   | :x:  |

### Mooneye GB acceptance tests

| Test                     | mooneye-gb | BGB  | Gambatte | GiiBiiAdvance | Higan | KiGB |
| ------------------------ | ---------- | ---- | -------- | ------------- | ------| ---- |
| add sp e timing          | :+1:       | :x:  | :+1:     | :x:           | :x:   | :x:  |
| call timing              | :+1:       | :x:  | :+1:     | :x:           | :x:   | :x:  |
| call timing2             | :+1:       | :x:  | :+1:     | :x:           | :x:   | :x:  |
| call cc_timing           | :+1:       | :x:  | :+1:     | :x:           | :x:   | :x:  |
| call cc_timing2          | :+1:       | :x:  | :+1:     | :x:           | :x:   | :x:  |
| di timing                | :+1:       | :+1: | :+1:     | :x:           | :x:   | :+1: |
| div timing               | :+1:       | :+1: | :+1:     | :+1:          | :x:   | :x:  |
| ei timing                | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :+1: |
| halt ime0 ei             | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :+1: |
| halt ime0 nointr_timing  | :+1:       | :+1: | :+1:     | :+1:          | :x:   | :x:  |
| halt ime1 timing         | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :x:  |
| halt ime1 timing2        | :+1:       | :+1: | :+1:     | :x:           | :x:   | :x:  |
| if ie registers          | :+1:       | :+1: | :+1:     | :+1:          | :x:   | :x:  |
| intr timing              | :+1:       | :+1: | :+1:     | :+1:          | :x:   | :x:  |
| jp timing                | :+1:       | :x:  | :+1:     | :x:           | :x:   | :x:  |
| jp cc timing             | :+1:       | :x:  | :+1:     | :x:           | :x:   | :x:  |
| ld hl sp e timing        | :+1:       | :x:  | :+1:     | :x:           | :x:   | :x:  |
| oam bits                 | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :+1: |
| oam dma_restart          | :+1:       | :x:  | :+1:     | :x:           | :x:   | :x:  |
| oam dma timing           | :+1:       | :x:  | :+1:     | :x:           | :x:   | :x:  |
| pop timing               | :+1:       | :x:  | :+1:     | :+1:          | :x:   | :x:  |
| push timing              | :+1:       | :x:  | :x:      | :x:           | :x:   | :x:  |
| rapid di ei              | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :+1: |
| ret timing               | :+1:       | :x:  | :+1:     | :x:           | :x:   | :x:  |
| ret cc timing            | :+1:       | :x:  | :+1:     | :x:           | :x:   | :x:  |
| reti timing              | :+1:       | :x:  | :+1:     | :x:           | :x:   | :x:  |
| reti intr timing         | :+1:       | :+1: | :+1:     | :+1:          | :+1:  | :+1: |
| rst timing               | :+1:       | :x:  | :x:      | :x:           | :x:   | :x:  |
| gpu hblank ly scx timing | :+1:       | :x:  | :x:      | :x:           | :x:   | :x:  |

### Mooneye GB emulator-only tests

| Test              | mooneye-gb | BGB  | Gambatte | GiiBiiAdvance | Higan | KiGB |
| ----------------- | ---------- | ---- | -------- | ------------- | ----- | ---- |
| mbc1 rom 4banks   | :+1:       | :x:  | :+1:     | :+1:          | :+1:  | :x:  |

### Mooneye GB manual tests

| Test              | mooneye-gb | BGB  | Gambatte | GiiBiiAdvance | Higan | KiGB |
| ----------------- | ---------- | ---- | -------- | ------------- | ----- | ---- |
| sprite priority   | :+1:       | :+1: | :+1:     | :x:           | :+1:  | :x:  |
