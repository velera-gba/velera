# Velera Graphics Module

## Module Goals

- [x] Synchronous interface through boxed slice passing
- [ ] Keypad input events
- [ ] Implement all GBA video modes ([List below](#Video-mode-implementation-progress))
- [ ] Closer emulation of real hardware
- [ ] GUI for configuring the emulator
- [ ] GUI for debugging tools
- [x] SDL backend
- [ ] TUI backend?
- [x] fb backend so it can be used in the vt

### Video mode implementation progress
- [ ] Mode 0:
- [ ] Mode 1: 
- [ ] Mode 2: 
- [x] Mode 3: 1 buffer BGR bitmap mode
- [ ] Mode 4: 
- [ ] Mode 5: 

## fbdev backend

Careful! the fbdev backend uses plenty of unsafe. If it panics or is killed at a poor time, the terminal will be left in a bad state (press `alt-sysrq-r` or `alt-printscreen-r` to recover keyboard). Do not send SIGKILL; Use SIGINT (ie. `pkill -2 velera`) to allow velera to clean up after itself.

Ensure Display is dropped (And only ever have one at a time which should enforce itself) to restore terminal and keyboard state safely. This means do not panic under any circumstance.