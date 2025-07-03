# Game Boy Emulator Timing Comparison: RGB vs rboy

This document compares the instruction timing implementations between our RGB Game Boy emulator and the rboy emulator (https://github.com/mvdnes/rboy).

## Key Architectural Difference

**RGB Emulator**: Uses pre-multiplied T-cycle counts directly (4, 8, 12, 16, etc.)
**rboy Emulator**: Uses base M-cycle counts (1, 2, 3, 4) then multiplies by 4 in `do_cycle()`

Both approaches are mathematically equivalent (1 M-cycle = 4 T-cycles), but represent timing differently in the code.

## Specific Timing Comparisons

### Arithmetic Instructions

| Instruction | RGB Cycles | rboy Base | rboy Actual | Match |
|-------------|------------|-----------|-------------|--------|
| ADD A,B     | 4          | 1         | 1×4 = 4     | ✅ |
| ADD A,d8    | 8          | 2         | 2×4 = 8     | ✅ |
| ADD A,(HL)  | 8          | 2         | 2×4 = 8     | ✅ |
| ADD HL,BC   | 8          | 2         | 2×4 = 8     | ✅ |

### Load Instructions

| Instruction | RGB Cycles | rboy Base | rboy Actual | Match |
|-------------|------------|-----------|-------------|--------|
| LD A,B      | 4          | 1         | 1×4 = 4     | ✅ |
| LD A,d8     | 8          | 2         | 2×4 = 8     | ✅ |
| LD A,(HL)   | 8          | 2         | 2×4 = 8     | ✅ |
| LD (HL),A   | 8          | 2         | 2×4 = 8     | ✅ |
| LD BC,d16   | 12         | 3         | 3×4 = 12    | ✅ |

### Memory Operations

| Instruction | RGB Cycles | rboy Base | rboy Actual | Match |
|-------------|------------|-----------|-------------|--------|
| LD (HL),d8  | 12         | 3         | 3×4 = 12    | ✅ |
| LD A,(nn)   | 16         | 4         | 4×4 = 16    | ✅ |
| LD (nn),A   | 16         | 4         | 4×4 = 16    | ✅ |

### Stack Operations

| Instruction | RGB Cycles | rboy Base | rboy Actual | Match |
|-------------|------------|-----------|-------------|--------|
| PUSH BC     | 16         | 4         | 4×4 = 16    | ✅ |
| POP BC      | 12         | 3         | 3×4 = 12    | ✅ |

### CB-Prefixed Instructions

| Instruction | RGB Cycles | rboy Base | rboy Actual | Match |
|-------------|------------|-----------|-------------|--------|
| BIT 0,A     | 8          | 2         | 2×4 = 8     | ✅ |
| RLC A       | 8          | 2         | 2×4 = 8     | ✅ |
| SET 0,A     | 8          | 2         | 2×4 = 8     | ✅ |

## Implementation Style Differences

### rboy's Approach
```rust
// rboy uses base cycle counts
0x80 => (AddAR8(R8::B), 1), // ADD A,B - 1 M-cycle
0xC6 => (AddAU8, 2),        // ADD A,d8 - 2 M-cycles

// Then multiplies in do_cycle()
fn do_cycle(&mut self, cycles: u8) {
    for _ in 0..cycles {
        self.cycle(); // Each cycle = 4 T-cycles
    }
}
```

### RGB's Approach
```rust
// RGB uses pre-multiplied T-cycle counts
InstructionKind::ADD(ArgKind::A, ArgKind::B) => 4,      // 4 T-cycles
InstructionKind::ADD(ArgKind::A, ArgKind::Immediate(_)) => 8, // 8 T-cycles
```

## Conditional Instruction Timing

Both emulators handle conditional jumps similarly:

| Instruction | Condition | RGB Cycles | rboy Cycles | Match |
|-------------|-----------|------------|-------------|--------|
| JP Z,nn     | Taken     | 16         | 4×4 = 16    | ✅ |
| JP Z,nn     | Not Taken | 12         | 3×4 = 12    | ✅ |
| JR Z,r8     | Taken     | 12         | 3×4 = 12    | ✅ |
| JR Z,r8     | Not Taken | 8          | 2×4 = 8     | ✅ |
| CALL Z,nn   | Taken     | 24         | 6×4 = 24    | ✅ |
| CALL Z,nn   | Not Taken | 12         | 3×4 = 12    | ✅ |

## No Significant Differences Found

After comprehensive analysis, **no significant timing differences** were found between the two emulators. Both implementations appear to use correct Game Boy timing values, just represented differently:

- **rboy**: Uses canonical M-cycle counts from hardware documentation
- **RGB**: Uses T-cycle counts for more direct CPU cycle management

## Architecture Analysis

### rboy's Cycle Management
- Clear separation between M-cycles (instruction timing) and T-cycles (hardware timing)
- More closely matches original Game Boy hardware documentation
- Uses `do_cycle(m_cycles)` to convert M-cycles to actual hardware cycles

### RGB's Cycle Management  
- Direct T-cycle counting simplifies main execution loop
- Pre-calculated values reduce runtime computation
- More straightforward for timing-critical operations

## Recommendations

1. **No Changes Required**: Both timing implementations are correct and equivalent
2. **Documentation**: Consider adding comments to clarify that RGB uses T-cycles directly
3. **Consistency**: Both emulators follow authentic Game Boy timing specifications
4. **Testing**: Both should pass standard Game Boy timing test ROMs

## Conclusion

The timing implementations in both emulators are **functionally identical** and correct. The only difference is representational - rboy uses the more traditional M-cycle approach while RGB uses direct T-cycle counts. Both methods accurately represent authentic Game Boy hardware timing.