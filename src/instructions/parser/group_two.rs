use crate::{AddressingModeType, Byte};
use crate::Opcode;
pub const MASK: Byte = 0b111_000_10;

/// "Group Two" instructions
pub fn decode_opcode_aaa_xxx_10(opcode: Byte) -> Opcode {
    match opcode & 0b111_000_11 {
        0b000_000_10 => Opcode::ASL,
        0b001_000_10 => Opcode::ROL,
        0b010_000_10 => Opcode::LSR,
        0b011_000_10 => Opcode::ROR,
        0b100_000_10 => Opcode::STX,
        0b101_000_10 => Opcode::LDX,
        0b110_000_10 => Opcode::DEC,
        0b111_000_10 => Opcode::INC,
        op => unreachable!("{op}"),
    }
}

pub fn decode_addressing_mode_xxx_bbb_10(opcode: Byte) -> AddressingModeType {
    match opcode & 0b000_111_11 {
        0b000_000_10 => AddressingModeType::Immediate,
        0b000_001_10 => AddressingModeType::ZeroPage,
        0b000_010_10 => AddressingModeType::Accumulator,
        0b000_011_10 => AddressingModeType::Absolute,
        // With STX and LDX this is ZeroPageIndexedWithX
        0b000_101_10 => AddressingModeType::ZeroPageIndexedWithX,
        // With LDX this is AbsoluteIndexedWithY,
        0b000_111_10 => AddressingModeType::AbsoluteIndexedWithX,
        op => unreachable!("{op}"),
    }
}