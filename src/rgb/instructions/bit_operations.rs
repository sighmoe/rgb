use super::{InstructionKind, ArgKind};

pub fn decode_bit_instruction(opcode: u16) -> Option<InstructionKind> {
    // CB-prefixed instructions are 16-bit opcodes (0xCBxx)
    match opcode {
        // Rotate left circular (RLC)
        0xCB00 => Some(InstructionKind::RLC(ArgKind::B)),
        0xCB01 => Some(InstructionKind::RLC(ArgKind::C)),
        0xCB02 => Some(InstructionKind::RLC(ArgKind::D)),
        0xCB03 => Some(InstructionKind::RLC(ArgKind::E)),
        0xCB04 => Some(InstructionKind::RLC(ArgKind::H)),
        0xCB05 => Some(InstructionKind::RLC(ArgKind::L)),
        0xCB06 => Some(InstructionKind::RLC(ArgKind::HL)),
        0xCB07 => Some(InstructionKind::RLC(ArgKind::A)),
        
        // Rotate right circular (RRC)
        0xCB08 => Some(InstructionKind::RRC(ArgKind::B)),
        0xCB09 => Some(InstructionKind::RRC(ArgKind::C)),
        0xCB0A => Some(InstructionKind::RRC(ArgKind::D)),
        0xCB0B => Some(InstructionKind::RRC(ArgKind::E)),
        0xCB0C => Some(InstructionKind::RRC(ArgKind::H)),
        0xCB0D => Some(InstructionKind::RRC(ArgKind::L)),
        0xCB0E => Some(InstructionKind::RRC(ArgKind::HL)),
        0xCB0F => Some(InstructionKind::RRC(ArgKind::A)),
        
        // Rotate left through carry (RL)
        0xCB10 => Some(InstructionKind::RL(ArgKind::B)),
        0xCB11 => Some(InstructionKind::RL(ArgKind::C)),
        0xCB12 => Some(InstructionKind::RL(ArgKind::D)),
        0xCB13 => Some(InstructionKind::RL(ArgKind::E)),
        0xCB14 => Some(InstructionKind::RL(ArgKind::H)),
        0xCB15 => Some(InstructionKind::RL(ArgKind::L)),
        0xCB16 => Some(InstructionKind::RL(ArgKind::HL)),
        0xCB17 => Some(InstructionKind::RL(ArgKind::A)),
        
        // Rotate right through carry (RR)
        0xCB18 => Some(InstructionKind::RR(ArgKind::B)),
        0xCB19 => Some(InstructionKind::RR(ArgKind::C)),
        0xCB1A => Some(InstructionKind::RR(ArgKind::D)),
        0xCB1B => Some(InstructionKind::RR(ArgKind::E)),
        0xCB1C => Some(InstructionKind::RR(ArgKind::H)),
        0xCB1D => Some(InstructionKind::RR(ArgKind::L)),
        0xCB1E => Some(InstructionKind::RR(ArgKind::HL)),
        0xCB1F => Some(InstructionKind::RR(ArgKind::A)),
        
        // Shift left arithmetic (SLA)
        0xCB20 => Some(InstructionKind::SLA(ArgKind::B)),
        0xCB21 => Some(InstructionKind::SLA(ArgKind::C)),
        0xCB22 => Some(InstructionKind::SLA(ArgKind::D)),
        0xCB23 => Some(InstructionKind::SLA(ArgKind::E)),
        0xCB24 => Some(InstructionKind::SLA(ArgKind::H)),
        0xCB25 => Some(InstructionKind::SLA(ArgKind::L)),
        0xCB26 => Some(InstructionKind::SLA(ArgKind::HL)),
        0xCB27 => Some(InstructionKind::SLA(ArgKind::A)),
        
        // Shift right arithmetic (SRA)
        0xCB28 => Some(InstructionKind::SRA(ArgKind::B)),
        0xCB29 => Some(InstructionKind::SRA(ArgKind::C)),
        0xCB2A => Some(InstructionKind::SRA(ArgKind::D)),
        0xCB2B => Some(InstructionKind::SRA(ArgKind::E)),
        0xCB2C => Some(InstructionKind::SRA(ArgKind::H)),
        0xCB2D => Some(InstructionKind::SRA(ArgKind::L)),
        0xCB2E => Some(InstructionKind::SRA(ArgKind::HL)),
        0xCB2F => Some(InstructionKind::SRA(ArgKind::A)),
        
        // Swap upper and lower nibbles (SWAP)
        0xCB30 => Some(InstructionKind::SWAP(ArgKind::B)),
        0xCB31 => Some(InstructionKind::SWAP(ArgKind::C)),
        0xCB32 => Some(InstructionKind::SWAP(ArgKind::D)),
        0xCB33 => Some(InstructionKind::SWAP(ArgKind::E)),
        0xCB34 => Some(InstructionKind::SWAP(ArgKind::H)),
        0xCB35 => Some(InstructionKind::SWAP(ArgKind::L)),
        0xCB36 => Some(InstructionKind::SWAP(ArgKind::HL)),
        0xCB37 => Some(InstructionKind::SWAP(ArgKind::A)),
        
        // Shift right logical (SRL)
        0xCB38 => Some(InstructionKind::SRL(ArgKind::B)),
        0xCB39 => Some(InstructionKind::SRL(ArgKind::C)),
        0xCB3A => Some(InstructionKind::SRL(ArgKind::D)),
        0xCB3B => Some(InstructionKind::SRL(ArgKind::E)),
        0xCB3C => Some(InstructionKind::SRL(ArgKind::H)),
        0xCB3D => Some(InstructionKind::SRL(ArgKind::L)),
        0xCB3E => Some(InstructionKind::SRL(ArgKind::HL)),
        0xCB3F => Some(InstructionKind::SRL(ArgKind::A)),
        // BIT 0, r
        0xCB40 => Some(InstructionKind::BIT(0, ArgKind::B)),
        0xCB41 => Some(InstructionKind::BIT(0, ArgKind::C)),
        0xCB42 => Some(InstructionKind::BIT(0, ArgKind::D)),
        0xCB43 => Some(InstructionKind::BIT(0, ArgKind::E)),
        0xCB44 => Some(InstructionKind::BIT(0, ArgKind::H)),
        0xCB45 => Some(InstructionKind::BIT(0, ArgKind::L)),
        0xCB46 => Some(InstructionKind::BIT(0, ArgKind::HL)),
        0xCB47 => Some(InstructionKind::BIT(0, ArgKind::A)),
        
        // BIT 1, r
        0xCB48 => Some(InstructionKind::BIT(1, ArgKind::B)),
        0xCB49 => Some(InstructionKind::BIT(1, ArgKind::C)),
        0xCB4A => Some(InstructionKind::BIT(1, ArgKind::D)),
        0xCB4B => Some(InstructionKind::BIT(1, ArgKind::E)),
        0xCB4C => Some(InstructionKind::BIT(1, ArgKind::H)),
        0xCB4D => Some(InstructionKind::BIT(1, ArgKind::L)),
        0xCB4E => Some(InstructionKind::BIT(1, ArgKind::HL)),
        0xCB4F => Some(InstructionKind::BIT(1, ArgKind::A)),
        
        // BIT 2, r
        0xCB50 => Some(InstructionKind::BIT(2, ArgKind::B)),
        0xCB51 => Some(InstructionKind::BIT(2, ArgKind::C)),
        0xCB52 => Some(InstructionKind::BIT(2, ArgKind::D)),
        0xCB53 => Some(InstructionKind::BIT(2, ArgKind::E)),
        0xCB54 => Some(InstructionKind::BIT(2, ArgKind::H)),
        0xCB55 => Some(InstructionKind::BIT(2, ArgKind::L)),
        0xCB56 => Some(InstructionKind::BIT(2, ArgKind::HL)),
        0xCB57 => Some(InstructionKind::BIT(2, ArgKind::A)),
        
        // BIT 3, r
        0xCB58 => Some(InstructionKind::BIT(3, ArgKind::B)),
        0xCB59 => Some(InstructionKind::BIT(3, ArgKind::C)),
        0xCB5A => Some(InstructionKind::BIT(3, ArgKind::D)),
        0xCB5B => Some(InstructionKind::BIT(3, ArgKind::E)),
        0xCB5C => Some(InstructionKind::BIT(3, ArgKind::H)),
        0xCB5D => Some(InstructionKind::BIT(3, ArgKind::L)),
        0xCB5E => Some(InstructionKind::BIT(3, ArgKind::HL)),
        0xCB5F => Some(InstructionKind::BIT(3, ArgKind::A)),
        
        // BIT 4, r
        0xCB60 => Some(InstructionKind::BIT(4, ArgKind::B)),
        0xCB61 => Some(InstructionKind::BIT(4, ArgKind::C)),
        0xCB62 => Some(InstructionKind::BIT(4, ArgKind::D)),
        0xCB63 => Some(InstructionKind::BIT(4, ArgKind::E)),
        0xCB64 => Some(InstructionKind::BIT(4, ArgKind::H)),
        0xCB65 => Some(InstructionKind::BIT(4, ArgKind::L)),
        0xCB66 => Some(InstructionKind::BIT(4, ArgKind::HL)),
        0xCB67 => Some(InstructionKind::BIT(4, ArgKind::A)),
        
        // BIT 5, r
        0xCB68 => Some(InstructionKind::BIT(5, ArgKind::B)),
        0xCB69 => Some(InstructionKind::BIT(5, ArgKind::C)),
        0xCB6A => Some(InstructionKind::BIT(5, ArgKind::D)),
        0xCB6B => Some(InstructionKind::BIT(5, ArgKind::E)),
        0xCB6C => Some(InstructionKind::BIT(5, ArgKind::H)),
        0xCB6D => Some(InstructionKind::BIT(5, ArgKind::L)),
        0xCB6E => Some(InstructionKind::BIT(5, ArgKind::HL)),
        0xCB6F => Some(InstructionKind::BIT(5, ArgKind::A)),
        
        // BIT 6, r
        0xCB70 => Some(InstructionKind::BIT(6, ArgKind::B)),
        0xCB71 => Some(InstructionKind::BIT(6, ArgKind::C)),
        0xCB72 => Some(InstructionKind::BIT(6, ArgKind::D)),
        0xCB73 => Some(InstructionKind::BIT(6, ArgKind::E)),
        0xCB74 => Some(InstructionKind::BIT(6, ArgKind::H)),
        0xCB75 => Some(InstructionKind::BIT(6, ArgKind::L)),
        0xCB76 => Some(InstructionKind::BIT(6, ArgKind::HL)),
        0xCB77 => Some(InstructionKind::BIT(6, ArgKind::A)),
        
        // BIT 7, r
        0xCB78 => Some(InstructionKind::BIT(7, ArgKind::B)),
        0xCB79 => Some(InstructionKind::BIT(7, ArgKind::C)),
        0xCB7A => Some(InstructionKind::BIT(7, ArgKind::D)),
        0xCB7B => Some(InstructionKind::BIT(7, ArgKind::E)),
        0xCB7C => Some(InstructionKind::BIT(7, ArgKind::H)),
        0xCB7D => Some(InstructionKind::BIT(7, ArgKind::L)),
        0xCB7E => Some(InstructionKind::BIT(7, ArgKind::HL)),
        0xCB7F => Some(InstructionKind::BIT(7, ArgKind::A)),
        
        // RES 0, r (reset bit 0)
        0xCB80 => Some(InstructionKind::RES(0, ArgKind::B)),
        0xCB81 => Some(InstructionKind::RES(0, ArgKind::C)),
        0xCB82 => Some(InstructionKind::RES(0, ArgKind::D)),
        0xCB83 => Some(InstructionKind::RES(0, ArgKind::E)),
        0xCB84 => Some(InstructionKind::RES(0, ArgKind::H)),
        0xCB85 => Some(InstructionKind::RES(0, ArgKind::L)),
        0xCB86 => Some(InstructionKind::RES(0, ArgKind::HL)),
        0xCB87 => Some(InstructionKind::RES(0, ArgKind::A)),
        
        // RES 1, r (reset bit 1)
        0xCB88 => Some(InstructionKind::RES(1, ArgKind::B)),
        0xCB89 => Some(InstructionKind::RES(1, ArgKind::C)),
        0xCB8A => Some(InstructionKind::RES(1, ArgKind::D)),
        0xCB8B => Some(InstructionKind::RES(1, ArgKind::E)),
        0xCB8C => Some(InstructionKind::RES(1, ArgKind::H)),
        0xCB8D => Some(InstructionKind::RES(1, ArgKind::L)),
        0xCB8E => Some(InstructionKind::RES(1, ArgKind::HL)),
        0xCB8F => Some(InstructionKind::RES(1, ArgKind::A)),
        
        // RES 2, r (reset bit 2)
        0xCB90 => Some(InstructionKind::RES(2, ArgKind::B)),
        0xCB91 => Some(InstructionKind::RES(2, ArgKind::C)),
        0xCB92 => Some(InstructionKind::RES(2, ArgKind::D)),
        0xCB93 => Some(InstructionKind::RES(2, ArgKind::E)),
        0xCB94 => Some(InstructionKind::RES(2, ArgKind::H)),
        0xCB95 => Some(InstructionKind::RES(2, ArgKind::L)),
        0xCB96 => Some(InstructionKind::RES(2, ArgKind::HL)),
        0xCB97 => Some(InstructionKind::RES(2, ArgKind::A)),
        
        // RES 3, r (reset bit 3)
        0xCB98 => Some(InstructionKind::RES(3, ArgKind::B)),
        0xCB99 => Some(InstructionKind::RES(3, ArgKind::C)),
        0xCB9A => Some(InstructionKind::RES(3, ArgKind::D)),
        0xCB9B => Some(InstructionKind::RES(3, ArgKind::E)),
        0xCB9C => Some(InstructionKind::RES(3, ArgKind::H)),
        0xCB9D => Some(InstructionKind::RES(3, ArgKind::L)),
        0xCB9E => Some(InstructionKind::RES(3, ArgKind::HL)),
        0xCB9F => Some(InstructionKind::RES(3, ArgKind::A)),
        
        // RES 4, r (reset bit 4)
        0xCBA0 => Some(InstructionKind::RES(4, ArgKind::B)),
        0xCBA1 => Some(InstructionKind::RES(4, ArgKind::C)),
        0xCBA2 => Some(InstructionKind::RES(4, ArgKind::D)),
        0xCBA3 => Some(InstructionKind::RES(4, ArgKind::E)),
        0xCBA4 => Some(InstructionKind::RES(4, ArgKind::H)),
        0xCBA5 => Some(InstructionKind::RES(4, ArgKind::L)),
        0xCBA6 => Some(InstructionKind::RES(4, ArgKind::HL)),
        0xCBA7 => Some(InstructionKind::RES(4, ArgKind::A)),
        
        // RES 5, r (reset bit 5)
        0xCBA8 => Some(InstructionKind::RES(5, ArgKind::B)),
        0xCBA9 => Some(InstructionKind::RES(5, ArgKind::C)),
        0xCBAA => Some(InstructionKind::RES(5, ArgKind::D)),
        0xCBAB => Some(InstructionKind::RES(5, ArgKind::E)),
        0xCBAC => Some(InstructionKind::RES(5, ArgKind::H)),
        0xCBAD => Some(InstructionKind::RES(5, ArgKind::L)),
        0xCBAE => Some(InstructionKind::RES(5, ArgKind::HL)),
        0xCBAF => Some(InstructionKind::RES(5, ArgKind::A)),
        
        // RES 6, r (reset bit 6)
        0xCBB0 => Some(InstructionKind::RES(6, ArgKind::B)),
        0xCBB1 => Some(InstructionKind::RES(6, ArgKind::C)),
        0xCBB2 => Some(InstructionKind::RES(6, ArgKind::D)),
        0xCBB3 => Some(InstructionKind::RES(6, ArgKind::E)),
        0xCBB4 => Some(InstructionKind::RES(6, ArgKind::H)),
        0xCBB5 => Some(InstructionKind::RES(6, ArgKind::L)),
        0xCBB6 => Some(InstructionKind::RES(6, ArgKind::HL)),
        0xCBB7 => Some(InstructionKind::RES(6, ArgKind::A)),
        
        // RES 7, r (reset bit 7)
        0xCBB8 => Some(InstructionKind::RES(7, ArgKind::B)),
        0xCBB9 => Some(InstructionKind::RES(7, ArgKind::C)),
        0xCBBA => Some(InstructionKind::RES(7, ArgKind::D)),
        0xCBBB => Some(InstructionKind::RES(7, ArgKind::E)),
        0xCBBC => Some(InstructionKind::RES(7, ArgKind::H)),
        0xCBBD => Some(InstructionKind::RES(7, ArgKind::L)),
        0xCBBE => Some(InstructionKind::RES(7, ArgKind::HL)),
        0xCBBF => Some(InstructionKind::RES(7, ArgKind::A)),
        
        // SET 0, r (set bit 0)
        0xCBC0 => Some(InstructionKind::SET(0, ArgKind::B)),
        0xCBC1 => Some(InstructionKind::SET(0, ArgKind::C)),
        0xCBC2 => Some(InstructionKind::SET(0, ArgKind::D)),
        0xCBC3 => Some(InstructionKind::SET(0, ArgKind::E)),
        0xCBC4 => Some(InstructionKind::SET(0, ArgKind::H)),
        0xCBC5 => Some(InstructionKind::SET(0, ArgKind::L)),
        0xCBC6 => Some(InstructionKind::SET(0, ArgKind::HL)),
        0xCBC7 => Some(InstructionKind::SET(0, ArgKind::A)),
        
        // SET 1, r (set bit 1)
        0xCBC8 => Some(InstructionKind::SET(1, ArgKind::B)),
        0xCBC9 => Some(InstructionKind::SET(1, ArgKind::C)),
        0xCBCA => Some(InstructionKind::SET(1, ArgKind::D)),
        0xCBCB => Some(InstructionKind::SET(1, ArgKind::E)),
        0xCBCC => Some(InstructionKind::SET(1, ArgKind::H)),
        0xCBCD => Some(InstructionKind::SET(1, ArgKind::L)),
        0xCBCE => Some(InstructionKind::SET(1, ArgKind::HL)),
        0xCBCF => Some(InstructionKind::SET(1, ArgKind::A)),
        
        // SET 2, r (set bit 2)
        0xCBD0 => Some(InstructionKind::SET(2, ArgKind::B)),
        0xCBD1 => Some(InstructionKind::SET(2, ArgKind::C)),
        0xCBD2 => Some(InstructionKind::SET(2, ArgKind::D)),
        0xCBD3 => Some(InstructionKind::SET(2, ArgKind::E)),
        0xCBD4 => Some(InstructionKind::SET(2, ArgKind::H)),
        0xCBD5 => Some(InstructionKind::SET(2, ArgKind::L)),
        0xCBD6 => Some(InstructionKind::SET(2, ArgKind::HL)),
        0xCBD7 => Some(InstructionKind::SET(2, ArgKind::A)),
        
        // SET 3, r (set bit 3)
        0xCBD8 => Some(InstructionKind::SET(3, ArgKind::B)),
        0xCBD9 => Some(InstructionKind::SET(3, ArgKind::C)),
        0xCBDA => Some(InstructionKind::SET(3, ArgKind::D)),
        0xCBDB => Some(InstructionKind::SET(3, ArgKind::E)),
        0xCBDC => Some(InstructionKind::SET(3, ArgKind::H)),
        0xCBDD => Some(InstructionKind::SET(3, ArgKind::L)),
        0xCBDE => Some(InstructionKind::SET(3, ArgKind::HL)),
        0xCBDF => Some(InstructionKind::SET(3, ArgKind::A)),
        
        // SET 4, r (set bit 4)
        0xCBE0 => Some(InstructionKind::SET(4, ArgKind::B)),
        0xCBE1 => Some(InstructionKind::SET(4, ArgKind::C)),
        0xCBE2 => Some(InstructionKind::SET(4, ArgKind::D)),
        0xCBE3 => Some(InstructionKind::SET(4, ArgKind::E)),
        0xCBE4 => Some(InstructionKind::SET(4, ArgKind::H)),
        0xCBE5 => Some(InstructionKind::SET(4, ArgKind::L)),
        0xCBE6 => Some(InstructionKind::SET(4, ArgKind::HL)),
        0xCBE7 => Some(InstructionKind::SET(4, ArgKind::A)),
        
        // SET 5, r (set bit 5)
        0xCBE8 => Some(InstructionKind::SET(5, ArgKind::B)),
        0xCBE9 => Some(InstructionKind::SET(5, ArgKind::C)),
        0xCBEA => Some(InstructionKind::SET(5, ArgKind::D)),
        0xCBEB => Some(InstructionKind::SET(5, ArgKind::E)),
        0xCBEC => Some(InstructionKind::SET(5, ArgKind::H)),
        0xCBED => Some(InstructionKind::SET(5, ArgKind::L)),
        0xCBEE => Some(InstructionKind::SET(5, ArgKind::HL)),
        0xCBEF => Some(InstructionKind::SET(5, ArgKind::A)),
        
        // SET 6, r (set bit 6)
        0xCBF0 => Some(InstructionKind::SET(6, ArgKind::B)),
        0xCBF1 => Some(InstructionKind::SET(6, ArgKind::C)),
        0xCBF2 => Some(InstructionKind::SET(6, ArgKind::D)),
        0xCBF3 => Some(InstructionKind::SET(6, ArgKind::E)),
        0xCBF4 => Some(InstructionKind::SET(6, ArgKind::H)),
        0xCBF5 => Some(InstructionKind::SET(6, ArgKind::L)),
        0xCBF6 => Some(InstructionKind::SET(6, ArgKind::HL)),
        0xCBF7 => Some(InstructionKind::SET(6, ArgKind::A)),
        
        // SET 7, r (set bit 7)
        0xCBF8 => Some(InstructionKind::SET(7, ArgKind::B)),
        0xCBF9 => Some(InstructionKind::SET(7, ArgKind::C)),
        0xCBFA => Some(InstructionKind::SET(7, ArgKind::D)),
        0xCBFB => Some(InstructionKind::SET(7, ArgKind::E)),
        0xCBFC => Some(InstructionKind::SET(7, ArgKind::H)),
        0xCBFD => Some(InstructionKind::SET(7, ArgKind::L)),
        0xCBFE => Some(InstructionKind::SET(7, ArgKind::HL)),
        0xCBFF => Some(InstructionKind::SET(7, ArgKind::A)),
        
        _ => None,
    }
}

