use crate::Byte;
use crate::Opcode;

///  Interrupt and subroutine instructionss
pub fn decode_opcode_interrupt_subroutine(opcode: Byte) -> Opcode {
    match opcode {
        0b0000_0000 => Opcode::BRK,
        0b0001_0000 => Opcode::JSR,
        0b0010_0000 => Opcode::RTI,
        0b0100_0000 => Opcode::RTS,
        op => unreachable!("{op}"),
    }
}
