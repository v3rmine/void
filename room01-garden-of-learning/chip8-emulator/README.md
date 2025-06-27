# chip8 emulator
- MAIN TUTORIAL: https://aquova.net/emudev/chip8/

- https://chip-8.github.io/links/
- https://www.reddit.com/r/EmuDev/
- https://tobiasvl.github.io/blog/write-a-chip-8-emulator/

## Execution Worflow
1. Chip8 ROM => Flat IR
2. Flat IR => AST
3. AST => Target IR (WASM? LLVM?)
4. Target IR => Native Code
5. Execution
