# Game Boy (LR35902) Complete Instruction Set Reference

This file contains the complete Game Boy CPU instruction set with opcodes, mnemonics, operands, byte lengths, and cycle timings.

## Regular Instructions (0x00-0xFF)

### 0x00-0x0F
| Opcode | Mnemonic    | Operands | Length | Cycles | Flags | Description |
|--------|-------------|----------|--------|--------|-------|-------------|
| 0x00   | NOP         |          | 1      | 4      | ----  | No operation |
| 0x01   | LD          | BC,d16   | 3      | 12     | ----  | Load 16-bit immediate into BC |
| 0x02   | LD          | (BC),A   | 1      | 8      | ----  | Load A into memory at BC |
| 0x03   | INC         | BC       | 1      | 8      | ----  | Increment BC |
| 0x04   | INC         | B        | 1      | 4      | Z0H-  | Increment B |
| 0x05   | DEC         | B        | 1      | 4      | Z1H-  | Decrement B |
| 0x06   | LD          | B,d8     | 2      | 8      | ----  | Load 8-bit immediate into B |
| 0x07   | RLCA        |          | 1      | 4      | 000C  | Rotate A left circular |
| 0x08   | LD          | (a16),SP | 3      | 20     | ----  | Load SP into memory at 16-bit address |
| 0x09   | ADD         | HL,BC    | 1      | 8      | -0HC  | Add BC to HL |
| 0x0A   | LD          | A,(BC)   | 1      | 8      | ----  | Load memory at BC into A |
| 0x0B   | DEC         | BC       | 1      | 8      | ----  | Decrement BC |
| 0x0C   | INC         | C        | 1      | 4      | Z0H-  | Increment C |
| 0x0D   | DEC         | C        | 1      | 4      | Z1H-  | Decrement C |
| 0x0E   | LD          | C,d8     | 2      | 8      | ----  | Load 8-bit immediate into C |
| 0x0F   | RRCA        |          | 1      | 4      | 000C  | Rotate A right circular |

### 0x10-0x1F
| Opcode | Mnemonic    | Operands | Length | Cycles | Flags | Description |
|--------|-------------|----------|--------|--------|-------|-------------|
| 0x10   | STOP        | 0        | 2      | 4      | ----  | Stop CPU and LCD |
| 0x11   | LD          | DE,d16   | 3      | 12     | ----  | Load 16-bit immediate into DE |
| 0x12   | LD          | (DE),A   | 1      | 8      | ----  | Load A into memory at DE |
| 0x13   | INC         | DE       | 1      | 8      | ----  | Increment DE |
| 0x14   | INC         | D        | 1      | 4      | Z0H-  | Increment D |
| 0x15   | DEC         | D        | 1      | 4      | Z1H-  | Decrement D |
| 0x16   | LD          | D,d8     | 2      | 8      | ----  | Load 8-bit immediate into D |
| 0x17   | RLA         |          | 1      | 4      | 000C  | Rotate A left through carry |
| 0x18   | JR          | r8       | 2      | 12     | ----  | Jump relative |
| 0x19   | ADD         | HL,DE    | 1      | 8      | -0HC  | Add DE to HL |
| 0x1A   | LD          | A,(DE)   | 1      | 8      | ----  | Load memory at DE into A |
| 0x1B   | DEC         | DE       | 1      | 8      | ----  | Decrement DE |
| 0x1C   | INC         | E        | 1      | 4      | Z0H-  | Increment E |
| 0x1D   | DEC         | E        | 1      | 4      | Z1H-  | Decrement E |
| 0x1E   | LD          | E,d8     | 2      | 8      | ----  | Load 8-bit immediate into E |
| 0x1F   | RRA         |          | 1      | 4      | 000C  | Rotate A right through carry |

### Key Missing Instructions Identified
- 0x09: ADD HL,BC (16-bit addition)
- 0x19: ADD HL,DE (16-bit addition)
- 0x29: ADD HL,HL (16-bit addition)
- 0x39: ADD HL,SP (16-bit addition)

### Additional Instructions Needed
- 0x08: LD (a16),SP
- 0x18: JR r8 (relative jump)
- Various rotate/shift instructions (0x07, 0x0F, 0x17, 0x1F)

## CB-Prefixed Instructions (0xCB00-0xCBFF)
These are 2-byte instructions starting with 0xCB prefix for bit operations, rotates, and shifts.

## Instruction Categories
1. **Load Instructions**: LD variants for 8-bit and 16-bit transfers
2. **Arithmetic**: ADD, SUB, INC, DEC operations
3. **Logic**: AND, OR, XOR, CP (compare)
4. **Bit Operations**: BIT, SET, RES (CB-prefixed)
5. **Jumps/Calls**: JP, JR, CALL, RET variants
6. **Stack**: PUSH, POP operations
7. **Misc**: NOP, HALT, STOP, EI, DI

## Flag Legend
- Z: Zero flag
- N: Subtract flag
- H: Half-carry flag  
- C: Carry flag
- -: Flag not affected
- 0: Flag reset
- 1: Flag set

## Timing Notes
- Cycles are for CPU running at 4.19MHz
- Branch instructions take extra cycles when taken
- Memory access instructions may take longer cycles