# Timer emulation accuracy

## Open questions

## Answered questions

### Does writing to DIV ($FF04) reset both the internal and the visible register?

*Answer:* Yes

DIV is incremented every 64 t-cycles, so there is an internal counter that counts to 64. If we write any value to the DIV register, it is reset to 0, but we don't know if the internal counter is also reset.

Consider the case where at time t=0 we reset the counter, and at time t=1 the DIV register would have incremented if we didn't do the reset. Do we see the DIV increment at time t=1 or t=64?

A test ROM confirmed that increment happens at t=64, so the internal counter is also reset. See tests/div_timing.
