# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Game Boy emulator written in Rust, currently named "rgb". The project uses the macroquad graphics library for rendering and implements the core components of a Game Boy system.

## Build and Development Commands

### Core Commands
- `cargo build` - Compile the project
- `cargo run` - Build and run the emulator
- `cargo test` - Run tests
- `cargo check` - Check for compilation errors without building
- `cargo clean` - Clean build artifacts

### Development Workflow
- Use `cargo check` for quick compilation feedback during development
- The project currently has a basic test in `src/rgb/cart.rs` - expand tests as needed
- The main executable uses macroquad's async main loop

## Architecture

### Core Components
The emulator is structured with the following main components in `src/rgb/`:

- **CPU** (`cpu.rs`) - Central processing unit with registers, program counter, and stack pointer
  - Contains instruction execution logic and arithmetic operations (e.g., ADD with flag handling)
  - Integrates with memory map and instruction decoder
  
- **Memory** (`memory.rs`) - Memory management system
  - 64KB memory map (`MemoryMap` struct)
  - Bootstrap ROM loading from external file
  - Read/write operations for memory addresses

- **Registers** (`registers.rs`) - CPU register management
  - Individual 8-bit registers (A, B, C, D, E, H, L) 
  - Flags register with zero, subtract, half-carry, and carry flags
  - 16-bit register pair operations (AF, BC, DE, HL)

- **Instructions** (`instructions.rs`) - Instruction set architecture
  - Instruction decoding and representation
  - Currently implements basic instructions (NOP, LD)
  - Arithmetic target enumeration for register operations

- **PPU** (`ppu.rs`) - Picture Processing Unit (graphics) - currently a stub
- **Cart** (`cart.rs`) - Game cartridge ROM loading from file system

### Key Implementation Details
- The CPU implements Game Boy-specific flag handling for arithmetic operations
- Memory map supports bootstrap ROM loading (expects 256-byte bootstrap ROM)
- Register system uses bitwise operations for flag management
- Instruction system is extensible with enum-based instruction kinds

### Current State
The project appears to be in early development:
- Basic CPU arithmetic operations are implemented
- Memory system is functional
- Graphics rendering uses macroquad but currently shows placeholder content
- Instruction set is minimal (only NOP implemented in decoder)

### Dependencies
- **macroquad 0.4.13** - Used for graphics rendering and game loop
- Standard Rust library for file system operations