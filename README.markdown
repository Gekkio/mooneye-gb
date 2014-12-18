# Mooneye GB

Another Gameboy emulator written in Rust. 

**Warning**:

* Project is WIP
* Doesn't work properly without a boot ROM

## Running the emulator

1. Acquire a Gameboy bootrom, and put it to ~/.mooneye-gb/boot.bin
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
