# Joypad emulation accuracy

## Open questions

### Do joypad interrupts depend on the select bits P14-P15, or do we get an interrupt whenever any key is pressed regardless of select bit state?

## Answered questions

### The joypad register (P1) has only 4 inputs (P10-P13). What happens if you enable both key select bits P14-P15 and press overlapping keys?

Both sets of keys are "merged" in the input bits P10-P13. So, if you have both key select bits enabled and press any combination of A and Right, you will see P10 go down (= "set"). Also, if you press A and Right, and then stop pressing Right, P10 should still be down because A is still being pressed.

### What is the initial state of the joypad register (P1)? Does the boot rom write to it?

The DMG/GBP boot rom doesn't write to the joypad register, and the initial value is 0xCF.
This means that key select bits P14-P15 (bits 4-5) are low (= "set").

If GBC is used with old Gameboy games, the boot rom writes and reads from P1, because old games support
palette switches with certain key combinations during boot. After booting, the value is 0xFF.
This means all bits are high (= "unset").
