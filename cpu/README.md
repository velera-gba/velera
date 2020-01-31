## CPU module for Velera

These are the goals and to do's for this module

### Short term:

- tests for the **thumb** module (partially done)

- proper CPU pipeline emulation

- **thumb** module execution

- [ ] Finish micro operations
- [X] Use `as` to cast to u8 from bool
- [X] Implement data transfer instructions (LDR, STR, ...) the right way (supporting addressing)
- [X] Change val2 and val3 to offset
- [X] Support shifting in ARM
- [ ] Remove all `unimplemented!` from the code

### Middle term:

- user stack access

- cycle emulation and interface

# Long term:

- implementation of the Sharp LR CPU for backward compatibility with the Game Boy.
