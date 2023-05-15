use crate::Byte;
use crate::Opcode;

/// Conditional branch instructions
pub fn decode_opcode_xx_y_10000(opcode: Byte) -> Opcode {
    match opcode & 0b111_1_0000 {
        0b000_10000 => Opcode::BPL,
        0b001_10000 => Opcode::BMI,
        0b010_10000 => Opcode::BVC,
        0b011_10000 => Opcode::BVS,
        0b100_10000 => Opcode::BCC,
        0b101_10000 => Opcode::BCS,
        0b110_10000 => Opcode::BNE,
        0b111_10000 => Opcode::BEQ,
        op => unreachable!("{op}"),
    }
}
