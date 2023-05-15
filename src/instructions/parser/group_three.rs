use crate::{AddressingModeType, Byte};
use crate::Opcode;
pub const MASK: Byte = 0b111_000_00;

/// "Group Three"" instructions
pub fn decode_opcode_aaa_xxx_00(opcode: Byte) -> Opcode {
    match opcode & 0b111_000_11 {
        0b001_000_00 => Opcode::BIT,
        0b010_000_00 => Opcode::JMP,
        0b011_000_00 => Opcode::JMP_ABS,
        0b100_000_00 => Opcode::STY,
        0b101_000_00 => Opcode::LDY,
        0b110_000_00 => Opcode::CPY,
        0b111_000_00 => Opcode::CPX,
        op => unreachable!("{op}"),
    }
}

pub fn decode_addressing_mode_xxx_bbb_00(opcode: Byte) -> AddressingModeType {
    match opcode & 0b000_111_11 {
        0b000_000_00 => AddressingModeType::Immediate,
        0b000_001_00 => AddressingModeType::ZeroPage,
        0b000_011_00 => AddressingModeType::Absolute,
        // With STX and LDX this is ZeroPageIndexedWithX
        0b000_101_00 => AddressingModeType::ZeroPageIndexedWithX,
        // With LDX this is AbsoluteIndexedWithY,
        0b000_111_00 => AddressingModeType::AbsoluteIndexedWithX,
        op => unreachable!("{op}"),
    }
}
