## CPU module for Velera

These are the goals and to do's for this module

### Short term:

- tests for the **thumb** module (partially done)

- proper CPU pipeline emulation

- **thumb** module execution

- [ ] Finish micro operations
- [X] Use `as` to cast to u8 from bool
- [X] Change val2 and val3 to offset
- [X] Support shifting in ARM
- [ ] Remove all `unimplemented!` from the code
- [X] Accumulate instructions
- [X] ALU Instructions
- [X] Set condition codes on data processing/multiply instructions
- [X] Privilege modes
- [X] Exceptions
- [X] Interrupts
- [X] Load/Store instructions
- [X] Correct arm master decode instruction
- [X] Implement all arm (32-bit) micro operations
- [ ] Implement Thumb decoding
- [ ] Implement Thumb versions of instructions
- [ ] Break down big micro ops into smaller ones (lol)
- [ ] Implement micro ops deque creator w/ pc extra-cycle handling
- [ ] Multiply extra cycles handling
- [ ] Add micro ops testing

- [ ] Implement DMA

# Long term:

- implementation of the Sharp LR CPU for backward compatibility with the Game Boy.
