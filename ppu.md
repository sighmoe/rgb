# Game Boy PPU (Picture Processing Unit) Documentation

## Overview

The PPU (Picture Processing Unit) is responsible for rendering graphics on the Game Boy screen. This document provides comprehensive technical details about how the Game Boy's graphics system works.

## Display Specifications

- **Screen Resolution**: 160x144 pixels
- **Color Depth**: 4 shades (2 bits per pixel)
- **Color Palette**: Gray/green monochrome
- **Tile Size**: 8x8 pixels
- **Color Encoding**: 2BPP (2 bits per pixel)

## Graphics Architecture

### Display Layers

The Game Boy PPU renders graphics using three main layers:

#### 1. Background Layer
- **Tile Grid**: 32x32 tiles (256x256 pixels total)
- **Viewport**: Shows 20x18 tiles on screen
- **Scrolling**: Controlled by SCX and SCY registers
- **Purpose**: Static background graphics

#### 2. Window Layer
- **Function**: Similar to background, acts as an overlay
- **Position**: Controlled by WX and WY registers
- **Use Case**: UI elements, status bars, dialog boxes

#### 3. Sprite Layer
- **Count**: Up to 40 sprites maximum
- **Sizes**: 8x8 or 8x16 pixels
- **Storage**: Object Attribute Memory (OAM)
- **Purpose**: Moving objects, characters, projectiles

## Rendering Process

### PPU Modes

The PPU operates in four distinct modes during each frame:

1. **Mode 2 - OAM Scan**
   - Scans Object Attribute Memory
   - Determines which sprites to render on current scanline
   - Duration varies based on sprite count

2. **Mode 3 - Drawing**
   - Active pixel rendering
   - Processes background, window, and sprites
   - Uses Pixel FIFO system

3. **Mode 0 - H-Blank**
   - Horizontal blanking period
   - Occurs after each scanline completion
   - CPU can access VRAM during this time

4. **Mode 1 - V-Blank**
   - Vertical blanking period
   - Occurs after all 144 scanlines rendered
   - Longest period for CPU VRAM access

### Pixel FIFO (First-In-First-Out)

The PPU uses a Pixel FIFO system for rendering:
- Renders pixels scanline by scanline
- Processes pixels sequentially
- Manages pixel priority and transparency
- Handles sprite-background interactions

## Key Registers

### LCDC (LCD Control Register)
Controls various display elements and PPU behavior:
- Display enable/disable
- Window and background enable
- Sprite size selection
- Tile data addressing mode

### STAT (LCD Status Register)
Provides PPU status information and interrupt controls:
- Current PPU mode indication
- Scanline comparison
- Interrupt enable flags
- PPU state monitoring

### Scroll Registers
- **SCX**: Background horizontal scroll
- **SCY**: Background vertical scroll
- **WX**: Window horizontal position
- **WY**: Window vertical position

## Memory Organization

### Video RAM (VRAM)
- **Size**: 8KB
- **Address Range**: 0x8000-0x9FFF
- **Contents**: Tile data and tile maps

### Object Attribute Memory (OAM)
- **Size**: 160 bytes
- **Address Range**: 0xFE00-0xFE9F
- **Contents**: Sprite attribute data

### Tile Data
- **Format**: 2BPP encoding
- **Size**: 16 bytes per tile
- **Organization**: Two bytes per tile row

## Technical Implementation Notes

### 2BPP Color Encoding
- Each pixel uses 2 bits
- 4 possible color values (0-3)
- Colors mapped through palette registers
- Efficient memory usage for monochrome display

### Timing Considerations
- PPU operates on strict timing cycles
- VRAM access restrictions during rendering
- Interrupt-driven programming model
- Critical for accurate emulation

### DMG Specifics
This documentation focuses on the DMG (original Game Boy) implementation:
- No color support
- Single background layer priority
- Limited sprite capabilities
- Foundation for later Game Boy models

## Development Resources

This documentation provides the technical foundation needed for:
- Game Boy emulator development
- Homebrew game programming
- Graphics system understanding
- Hardware-accurate implementations

## Notes for Emulator Implementation

Key considerations for accurate PPU emulation:
- Cycle-accurate timing simulation
- Proper mode transitions
- VRAM access restrictions
- Interrupt generation
- Pixel FIFO implementation
- Sprite priority handling