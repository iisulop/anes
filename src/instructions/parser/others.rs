use crate::Byte;
use crate::Opcode;

pub fn decode_other_instruction(opcode: Byte) -> Opcode {
    match opcode {
        0b0000_1000 => Opcode::PHP,
        0b0010_1000 => Opcode::PLP,
        0b0100_1000 => Opcode::PHA,
        0b0110_1000 => Opcode::PLA,
        0b1000_1000 => Opcode::DEY,
        0b1010_1000 => Opcode::TAY,
        0b1100_1000 => Opcode::INY,
        0b1110_1000 => Opcode::INX,
        0b0001_1000 => Opcode::CLC,
        0b0011_1000 => Opcode::SEC,
        0b0101_1000 => Opcode::CLI,
        0b0111_1000 => Opcode::SEI,
        0b1001_1000 => Opcode::TYA,
        0b1011_1000 => Opcode::CLV,
        0b1101_1000 => Opcode::CLD,
        0b1111_1000 => Opcode::SED,
        0b1000_1010 => Opcode::TXA,
        0b1001_1010 => Opcode::TXS,
        0b1010_1010 => Opcode::TAX,
        0b1011_1010 => Opcode::TSX,
        0b1100_1010 => Opcode::DEX,
        0b1110_1010 => Opcode::NOP,
        op => unreachable!("{op}"),
    }
}
