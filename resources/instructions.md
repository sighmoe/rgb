# Game Boy CPU Instructions

This document contains Game Boy CPU opcodes with their complete instruction details.

**Generated Instructions**: 133 (representative sample of the complete ~512 instruction set)

## Instruction Format

Each instruction includes:
- **Mnemonic**: Assembly language representation
- **OpCode**: Hexadecimal opcode value
- **Type**: Instruction category  
- **Flags**: Effect on CPU flags (CY=Carry, H=Half-carry, N=Subtract, Z=Zero)
  - ○ = Flag affected by result
  - 0/1 = Flag set to 0 or 1
  - empty = Flag not affected
- **Cycles**: CPU cycles required (X/Y means X if condition true, Y if false)
- **Bytes**: Instruction size in bytes
- **Description**: Detailed explanation of the operation

## Code Key

- `r` - 8-bit register (A, B, C, D, E, H, L)
- `rr` - 16-bit register pair (BC, DE, HL, SP)
- `d8` - 8-bit immediate data value
- `d16` - 16-bit immediate data value
- `a16` - 16-bit immediate address value
- `s8` - 8-bit signed immediate data value

---

## 16-bit transfer

### LD BC, d16

- **OpCode**: `0x01`
- **Type**: 16-bit transfer
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 3
- **Bytes**: 3
- **Description**: Load 16-bit immediate value d16 into register pair BC.

### LD DE, d16

- **OpCode**: `0x11`
- **Type**: 16-bit transfer
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 3
- **Bytes**: 3
- **Description**: Load 16-bit immediate value d16 into register pair DE.

### LD HL, d16

- **OpCode**: `0x21`
- **Type**: 16-bit transfer
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 3
- **Bytes**: 3
- **Description**: Load 16-bit immediate value d16 into register pair HL.

### LD SP, d16

- **OpCode**: `0x31`
- **Type**: 16-bit transfer
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 3
- **Bytes**: 3
- **Description**: Load 16-bit immediate value d16 into register pair SP.

## 8-bit arithmetic and logical operation

### INC B

- **OpCode**: `0x04`
- **Type**: 8-bit arithmetic and logical operation
- **Flags**: CY=-, H=○, N=0, Z=○
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Increment the contents of register B by 1.

### INC C

- **OpCode**: `0x0C`
- **Type**: 8-bit arithmetic and logical operation
- **Flags**: CY=-, H=○, N=0, Z=○
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Increment the contents of register C by 1.

### INC D

- **OpCode**: `0x14`
- **Type**: 8-bit arithmetic and logical operation
- **Flags**: CY=-, H=○, N=0, Z=○
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Increment the contents of register D by 1.

### INC E

- **OpCode**: `0x1C`
- **Type**: 8-bit arithmetic and logical operation
- **Flags**: CY=-, H=○, N=0, Z=○
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Increment the contents of register E by 1.

### INC H

- **OpCode**: `0x24`
- **Type**: 8-bit arithmetic and logical operation
- **Flags**: CY=-, H=○, N=0, Z=○
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Increment the contents of register H by 1.

### INC L

- **OpCode**: `0x2C`
- **Type**: 8-bit arithmetic and logical operation
- **Flags**: CY=-, H=○, N=0, Z=○
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Increment the contents of register L by 1.

### INC A

- **OpCode**: `0x3C`
- **Type**: 8-bit arithmetic and logical operation
- **Flags**: CY=-, H=○, N=0, Z=○
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Increment the contents of register A by 1.

### ADD A, B

- **OpCode**: `0x80`
- **Type**: 8-bit arithmetic and logical operation
- **Flags**: CY=○, H=○, N=0, Z=○
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Add the contents of register B to register A.

### ADD A, C

- **OpCode**: `0x81`
- **Type**: 8-bit arithmetic and logical operation
- **Flags**: CY=○, H=○, N=0, Z=○
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Add the contents of register C to register A.

### ADD A, D

- **OpCode**: `0x82`
- **Type**: 8-bit arithmetic and logical operation
- **Flags**: CY=○, H=○, N=0, Z=○
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Add the contents of register D to register A.

### ADD A, E

- **OpCode**: `0x83`
- **Type**: 8-bit arithmetic and logical operation
- **Flags**: CY=○, H=○, N=0, Z=○
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Add the contents of register E to register A.

### ADD A, H

- **OpCode**: `0x84`
- **Type**: 8-bit arithmetic and logical operation
- **Flags**: CY=○, H=○, N=0, Z=○
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Add the contents of register H to register A.

### ADD A, L

- **OpCode**: `0x85`
- **Type**: 8-bit arithmetic and logical operation
- **Flags**: CY=○, H=○, N=0, Z=○
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Add the contents of register L to register A.

### ADD A, A

- **OpCode**: `0x87`
- **Type**: 8-bit arithmetic and logical operation
- **Flags**: CY=○, H=○, N=0, Z=○
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Add the contents of register A to register A.

## 8-bit transfer and I/O

### LD B, d8

- **OpCode**: `0x06`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Load 8-bit immediate value d8 into register B.

### LD C, d8

- **OpCode**: `0x0E`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Load 8-bit immediate value d8 into register C.

### LD D, d8

- **OpCode**: `0x16`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Load 8-bit immediate value d8 into register D.

### LD E, d8

- **OpCode**: `0x1E`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Load 8-bit immediate value d8 into register E.

### LD H, d8

- **OpCode**: `0x26`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Load 8-bit immediate value d8 into register H.

### LD L, d8

- **OpCode**: `0x2E`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Load 8-bit immediate value d8 into register L.

### LD A, d8

- **OpCode**: `0x3E`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Load 8-bit immediate value d8 into register A.

### LD B, C

- **OpCode**: `0x41`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register C into register B.

### LD B, D

- **OpCode**: `0x42`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register D into register B.

### LD B, E

- **OpCode**: `0x43`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register E into register B.

### LD B, H

- **OpCode**: `0x44`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register H into register B.

### LD B, L

- **OpCode**: `0x45`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register L into register B.

### LD B, A

- **OpCode**: `0x47`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register A into register B.

### LD C, B

- **OpCode**: `0x48`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register B into register C.

### LD C, D

- **OpCode**: `0x4A`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register D into register C.

### LD C, E

- **OpCode**: `0x4B`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register E into register C.

### LD C, H

- **OpCode**: `0x4C`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register H into register C.

### LD C, L

- **OpCode**: `0x4D`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register L into register C.

### LD C, A

- **OpCode**: `0x4F`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register A into register C.

### LD D, B

- **OpCode**: `0x50`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register B into register D.

### LD D, C

- **OpCode**: `0x51`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register C into register D.

### LD D, E

- **OpCode**: `0x53`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register E into register D.

### LD D, H

- **OpCode**: `0x54`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register H into register D.

### LD D, L

- **OpCode**: `0x55`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register L into register D.

### LD D, A

- **OpCode**: `0x57`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register A into register D.

### LD E, B

- **OpCode**: `0x58`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register B into register E.

### LD E, C

- **OpCode**: `0x59`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register C into register E.

### LD E, D

- **OpCode**: `0x5A`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register D into register E.

### LD E, H

- **OpCode**: `0x5C`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register H into register E.

### LD E, L

- **OpCode**: `0x5D`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register L into register E.

### LD E, A

- **OpCode**: `0x5F`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register A into register E.

### LD H, B

- **OpCode**: `0x60`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register B into register H.

### LD H, C

- **OpCode**: `0x61`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register C into register H.

### LD H, D

- **OpCode**: `0x62`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register D into register H.

### LD H, E

- **OpCode**: `0x63`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register E into register H.

### LD H, L

- **OpCode**: `0x65`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register L into register H.

### LD H, A

- **OpCode**: `0x67`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register A into register H.

### LD L, B

- **OpCode**: `0x68`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register B into register L.

### LD L, C

- **OpCode**: `0x69`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register C into register L.

### LD L, D

- **OpCode**: `0x6A`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register D into register L.

### LD L, E

- **OpCode**: `0x6B`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register E into register L.

### LD L, H

- **OpCode**: `0x6C`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register H into register L.

### LD L, A

- **OpCode**: `0x6F`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register A into register L.

### LD A, B

- **OpCode**: `0x78`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register B into register A.

### LD A, C

- **OpCode**: `0x79`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register C into register A.

### LD A, D

- **OpCode**: `0x7A`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register D into register A.

### LD A, E

- **OpCode**: `0x7B`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register E into register A.

### LD A, H

- **OpCode**: `0x7C`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register H into register A.

### LD A, L

- **OpCode**: `0x7D`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register L into register A.

### LD A, A

- **OpCode**: `0x7F`
- **Type**: 8-bit transfer and I/O
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Load the contents of register A into register A.

## bit operation

### BIT 0, B

- **OpCode**: `0xCB40`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 0 of register B. Set Z flag if bit is 0.

### BIT 0, C

- **OpCode**: `0xCB41`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 0 of register C. Set Z flag if bit is 0.

### BIT 0, D

- **OpCode**: `0xCB42`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 0 of register D. Set Z flag if bit is 0.

### BIT 0, E

- **OpCode**: `0xCB43`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 0 of register E. Set Z flag if bit is 0.

### BIT 0, H

- **OpCode**: `0xCB44`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 0 of register H. Set Z flag if bit is 0.

### BIT 0, L

- **OpCode**: `0xCB45`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 0 of register L. Set Z flag if bit is 0.

### BIT 0, A

- **OpCode**: `0xCB47`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 0 of register A. Set Z flag if bit is 0.

### BIT 1, B

- **OpCode**: `0xCB48`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 1 of register B. Set Z flag if bit is 0.

### BIT 1, C

- **OpCode**: `0xCB49`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 1 of register C. Set Z flag if bit is 0.

### BIT 1, D

- **OpCode**: `0xCB4A`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 1 of register D. Set Z flag if bit is 0.

### BIT 1, E

- **OpCode**: `0xCB4B`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 1 of register E. Set Z flag if bit is 0.

### BIT 1, H

- **OpCode**: `0xCB4C`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 1 of register H. Set Z flag if bit is 0.

### BIT 1, L

- **OpCode**: `0xCB4D`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 1 of register L. Set Z flag if bit is 0.

### BIT 1, A

- **OpCode**: `0xCB4F`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 1 of register A. Set Z flag if bit is 0.

### BIT 2, B

- **OpCode**: `0xCB50`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 2 of register B. Set Z flag if bit is 0.

### BIT 2, C

- **OpCode**: `0xCB51`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 2 of register C. Set Z flag if bit is 0.

### BIT 2, D

- **OpCode**: `0xCB52`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 2 of register D. Set Z flag if bit is 0.

### BIT 2, E

- **OpCode**: `0xCB53`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 2 of register E. Set Z flag if bit is 0.

### BIT 2, H

- **OpCode**: `0xCB54`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 2 of register H. Set Z flag if bit is 0.

### BIT 2, L

- **OpCode**: `0xCB55`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 2 of register L. Set Z flag if bit is 0.

### BIT 2, A

- **OpCode**: `0xCB57`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 2 of register A. Set Z flag if bit is 0.

### BIT 3, B

- **OpCode**: `0xCB58`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 3 of register B. Set Z flag if bit is 0.

### BIT 3, C

- **OpCode**: `0xCB59`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 3 of register C. Set Z flag if bit is 0.

### BIT 3, D

- **OpCode**: `0xCB5A`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 3 of register D. Set Z flag if bit is 0.

### BIT 3, E

- **OpCode**: `0xCB5B`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 3 of register E. Set Z flag if bit is 0.

### BIT 3, H

- **OpCode**: `0xCB5C`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 3 of register H. Set Z flag if bit is 0.

### BIT 3, L

- **OpCode**: `0xCB5D`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 3 of register L. Set Z flag if bit is 0.

### BIT 3, A

- **OpCode**: `0xCB5F`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 3 of register A. Set Z flag if bit is 0.

### BIT 4, B

- **OpCode**: `0xCB60`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 4 of register B. Set Z flag if bit is 0.

### BIT 4, C

- **OpCode**: `0xCB61`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 4 of register C. Set Z flag if bit is 0.

### BIT 4, D

- **OpCode**: `0xCB62`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 4 of register D. Set Z flag if bit is 0.

### BIT 4, E

- **OpCode**: `0xCB63`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 4 of register E. Set Z flag if bit is 0.

### BIT 4, H

- **OpCode**: `0xCB64`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 4 of register H. Set Z flag if bit is 0.

### BIT 4, L

- **OpCode**: `0xCB65`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 4 of register L. Set Z flag if bit is 0.

### BIT 4, A

- **OpCode**: `0xCB67`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 4 of register A. Set Z flag if bit is 0.

### BIT 5, B

- **OpCode**: `0xCB68`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 5 of register B. Set Z flag if bit is 0.

### BIT 5, C

- **OpCode**: `0xCB69`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 5 of register C. Set Z flag if bit is 0.

### BIT 5, D

- **OpCode**: `0xCB6A`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 5 of register D. Set Z flag if bit is 0.

### BIT 5, E

- **OpCode**: `0xCB6B`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 5 of register E. Set Z flag if bit is 0.

### BIT 5, H

- **OpCode**: `0xCB6C`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 5 of register H. Set Z flag if bit is 0.

### BIT 5, L

- **OpCode**: `0xCB6D`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 5 of register L. Set Z flag if bit is 0.

### BIT 5, A

- **OpCode**: `0xCB6F`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 5 of register A. Set Z flag if bit is 0.

### BIT 6, B

- **OpCode**: `0xCB70`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 6 of register B. Set Z flag if bit is 0.

### BIT 6, C

- **OpCode**: `0xCB71`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 6 of register C. Set Z flag if bit is 0.

### BIT 6, D

- **OpCode**: `0xCB72`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 6 of register D. Set Z flag if bit is 0.

### BIT 6, E

- **OpCode**: `0xCB73`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 6 of register E. Set Z flag if bit is 0.

### BIT 6, H

- **OpCode**: `0xCB74`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 6 of register H. Set Z flag if bit is 0.

### BIT 6, L

- **OpCode**: `0xCB75`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 6 of register L. Set Z flag if bit is 0.

### BIT 6, A

- **OpCode**: `0xCB77`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 6 of register A. Set Z flag if bit is 0.

### BIT 7, B

- **OpCode**: `0xCB78`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 7 of register B. Set Z flag if bit is 0.

### BIT 7, C

- **OpCode**: `0xCB79`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 7 of register C. Set Z flag if bit is 0.

### BIT 7, D

- **OpCode**: `0xCB7A`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 7 of register D. Set Z flag if bit is 0.

### BIT 7, E

- **OpCode**: `0xCB7B`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 7 of register E. Set Z flag if bit is 0.

### BIT 7, H

- **OpCode**: `0xCB7C`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 7 of register H. Set Z flag if bit is 0.

### BIT 7, L

- **OpCode**: `0xCB7D`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 7 of register L. Set Z flag if bit is 0.

### BIT 7, A

- **OpCode**: `0xCB7F`
- **Type**: bit operation
- **Flags**: CY=-, H=1, N=0, Z=○
- **Cycles**: 2
- **Bytes**: 2
- **Description**: Test bit 7 of register A. Set Z flag if bit is 0.

## call and return

### RET

- **OpCode**: `0xC9`
- **Type**: call and return
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 4
- **Bytes**: 1
- **Description**: Pop return address from stack and jump to it.

### CALL a16

- **OpCode**: `0xCD`
- **Type**: call and return
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 6
- **Bytes**: 3
- **Description**: Push current PC onto stack and jump to 16-bit absolute address a16.

## general-purpose arithmetic operations and CPU control

### NOP

- **OpCode**: `0x00`
- **Type**: general-purpose arithmetic operations and CPU control
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: No operation. Do nothing for 1 clock cycle.

### HALT

- **OpCode**: `0x76`
- **Type**: general-purpose arithmetic operations and CPU control
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 1
- **Bytes**: 1
- **Description**: Halt CPU until an interrupt occurs.

## jump

### JP NZ, a16

- **OpCode**: `0x62`
- **Type**: jump
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 4/3
- **Bytes**: 3
- **Description**: Jump to 16-bit absolute address a16 if condition NZ is true. Takes 4 cycles if condition is true, 3 if false.

### JP Z, a16

- **OpCode**: `0x6A`
- **Type**: jump
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 4/3
- **Bytes**: 3
- **Description**: Jump to 16-bit absolute address a16 if condition Z is true. Takes 4 cycles if condition is true, 3 if false.

### JP NC, a16

- **OpCode**: `0x72`
- **Type**: jump
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 4/3
- **Bytes**: 3
- **Description**: Jump to 16-bit absolute address a16 if condition NC is true. Takes 4 cycles if condition is true, 3 if false.

### JP C, a16

- **OpCode**: `0x7A`
- **Type**: jump
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 4/3
- **Bytes**: 3
- **Description**: Jump to 16-bit absolute address a16 if condition C is true. Takes 4 cycles if condition is true, 3 if false.

### JP a16

- **OpCode**: `0xC3`
- **Type**: jump
- **Flags**: CY=-, H=-, N=-, Z=-
- **Cycles**: 4
- **Bytes**: 3
- **Description**: Jump to 16-bit absolute address a16.

---

## Complete Instruction Set

This document shows 133 representative instructions. The complete Game Boy CPU instruction set contains:
- 256 standard opcodes (0x00-0xFF)
- 256 CB-prefixed opcodes (0xCB00-0xCBFF) for bit operations and extended rotate/shift

To generate the complete instruction set, run the web application and use the browser console to execute `generateAllInstructions()`.
