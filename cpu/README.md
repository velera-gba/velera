## CPU module for Velera

These are the goals and to do's for this module

### Short term:

- tests for the **thumb** module (partially done)

- proper CPU pipeline emulation

- **thumb** module execution

- [ ] Finish micro operations
- [ ] Use `as` to cast to u8 from bool
- [ ] Implement data transfer instructions (LDR, STR, ...) the right way (supporting addressing)
- [ ] Support shifting in ARM
- [ ] Remove all `unimplemented!` from the code
- [ ] Implement a load/store register micro operation that (obviously) takes a single cycle, storing in a temporary register in the CPU.

### Middle term:

- user stack access

- cycle emulation and interface

# Long term:

- implementation of the Sharp LR CPU for backward compatibility with the Game Boy.
