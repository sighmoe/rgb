# RGB - Game Boy Emulator

An experimental Game Boy emulator written entirely in Rust through AI-driven development using [Claude Code](https://claude.ai/code). This project demonstrates the capabilities of AI code agents in implementing complex systems like console emulators through natural language prompts and iterative development.

## Project Overview

This emulator was built entirely through conversational programming with Claude Code, starting from basic CPU instruction execution and progressively implementing:

- Complete Game Boy CPU instruction set (including CB-prefixed instructions)
- Memory management with MBC3 cartridge support
- Picture Processing Unit (PPU) with cycle-accurate timing
- Timer system and interrupt handling
- Joypad input support
- Sound system (basic implementation)

The development process showcased AI-assisted debugging, performance optimization, and incremental feature implementation through natural language descriptions of desired functionality.

## Features

- **CPU Emulation**: Full Game Boy CPU instruction set with proper flag handling
- **Memory Banking**: MBC3 memory bank controller support for larger ROMs
- **Graphics**: PPU implementation with background, window, and sprite rendering
- **Input**: Keyboard controls mapped to Game Boy buttons
- **Performance**: Optimized for 60fps gameplay with configurable timing
- **Debugging**: Comprehensive debug output and execution tracing (debug builds only)

## Building and Running

### Prerequisites

- Rust (latest stable version)
- Game Boy ROM files for testing

### Build Instructions

```bash
# Clone the repository
git clone <repository-url>
cd rgb

# Build the project
cargo build --release

# Run with a ROM file
cargo run --release -- path/to/your/rom.gb

# Skip boot ROM sequence (faster startup)
cargo run --release -- --skip-boot path/to/your/rom.gb
```

### Command Line Options

- `--skip-boot`, `-s`: Skip the Game Boy boot sequence and start directly with the ROM
- `--trace <file>`, `-t <file>`: Write execution trace to specified file (debug builds only)
- `--trace-json`: Format trace output as JSON (requires --trace)
- `--help`, `-h`: Show help message

### Controls

- **Arrow Keys**: D-pad
- **Z**: A button
- **X**: B button
- **Enter**: START button
- **Right Shift**: SELECT button

## ROM Compatibility Status

### Pokemon Blue ✅ Partially Working
- **Status**: Renders graphics (979+ pixels), executes game logic
- **Progress**: Graphics routines active, HRAM variables being written
- **Issues**: Occasional HALT state loops, not fully playable yet
- **Command**: `cargo run --release -- --skip-boot ./test-roms/pkmn.gb`

### Dr. Mario ✅ Working
- **Status**: Playable with proper graphics rendering
- **Progress**: Full menu navigation, gameplay mechanics functional
- **Issues**: Minor timing inconsistencies in some animations
- **Command**: `cargo run --release -- --skip-boot ./test-roms/dr-mario.gb`

### Tetris ✅ Working
- **Status**: Fully playable with smooth block animations
- **Progress**: Complete gameplay, proper piece rotation and line clearing
- **Issues**: None significant, excellent compatibility
- **Command**: `cargo run --release -- --skip-boot ./test-roms/tetris.gb`

## Technical Implementation

### Architecture

The emulator follows a modular design with separate systems for:

- **CPU** (`src/rgb/cpu.rs`): Game Boy CPU with register management and instruction execution
- **Memory** (`src/rgb/memory.rs`): Memory mapping, cartridge loading, and hardware registers
- **PPU** (`src/rgb/ppu.rs`): Graphics processing with scanline rendering and sprite support
- **Cartridge** (`src/rgb/cart.rs`): ROM loading and MBC3 memory bank controller
- **Instructions** (`src/rgb/instructions.rs`): Instruction decoding and execution system

### Resources Folder

The `resources/` folder contains development documentation and reference materials:

- **dmgops.json** & **gb-instructions-db.json**: Game Boy instruction set databases
- **instructions.md** & **updated_instructions.md**: CPU instruction documentation
- **ppu.md**: Picture Processing Unit implementation notes
- **differences.md** & **inconsistencies.md**: Development notes on implementation details
- **prompt**: AI prompts and development guidance

### Performance Optimizations

- **60fps Target**: Optimized timing for smooth gameplay
- **Batch Processing**: PPU operates in 114-cycle chunks for efficiency
- **Debug Conditionals**: Debug output only in debug builds for release performance
- **Memory Efficiency**: Streamlined frame buffer and hash calculations

### Development Process

This project was developed entirely through AI-assisted programming:

1. **Initial Setup**: Basic project structure and CPU foundation
2. **Instruction Implementation**: Incremental addition of Game Boy opcodes
3. **Graphics System**: PPU development with timing accuracy
4. **ROM Support**: MBC3 implementation for Pokemon compatibility
5. **Performance Tuning**: 60fps optimization and debug cleanup
6. **Testing & Debugging**: Iterative fixes based on ROM behavior

## Development Philosophy

This project demonstrates several key aspects of AI-assisted development:

- **Incremental Complexity**: Building from simple concepts to complex systems
- **Natural Language Specifications**: Describing desired behavior in plain English
- **Iterative Debugging**: AI-driven problem solving and optimization
- **Code Quality**: Maintaining clean, documented, and efficient code
- **Performance Awareness**: Balancing accuracy with practical performance

## Development Tools

### Trace Comparison Tool

The `trace_diff.py` script compares Game Boy execution traces to help debug emulator behavior:

```bash
# Compare two trace files
python trace_diff.py trace1.txt trace2.txt

# Compare with options
python trace_diff.py trace1.json trace2.json --no-color --max-lines 50

# Show matching instructions too
python trace_diff.py trace1.txt trace2.txt --show-matching

# Limit comparison to first N instructions
python trace_diff.py trace1.txt trace2.txt --limit-comparison 1000
```

Features:
- Supports both standard text and JSON trace formats
- Colorized diff output highlighting register and memory differences
- Configurable output limits and comparison scope
- Useful for comparing emulator output against reference implementations

## Future Improvements

- Complete Pokemon ROM compatibility
- Sound system implementation
- Additional MBC support (MBC1, MBC5)
- Save state functionality
- Enhanced debugging tools
- More comprehensive test ROM support

## Contributing

This project serves as a demonstration of AI-assisted development. While contributions are welcome, the primary goal is showcasing the capabilities of conversational programming with AI code agents.

## License

This project is provided as-is for educational and experimental purposes. Game Boy is a trademark of Nintendo Co., Ltd.

---

*This emulator was developed entirely through natural language interactions with Claude Code, demonstrating the potential of AI-assisted software development for complex systems programming.*