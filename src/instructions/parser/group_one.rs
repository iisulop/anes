use std::collections::HashSet;

use lazy_static::lazy_static;

use crate::{AddressingModeType, Byte, Registers};
use crate::instructions::opcode::Memory;
use crate::Opcode;

pub const MASK: Byte = 0b111_000_01;

/// "Group One" instructions
pub fn decode_opcode_aaa_xxx_01(opcode: Byte) -> Opcode {
    match opcode & 0b111_000_11 {
        0b000_000_01 => Opcode::ORA,
        0b001_000_01 => Opcode::AND,
        0b010_000_01 => Opcode::EOR,
        0b011_000_01 => Opcode::ADC,
        0b100_000_01 => Opcode::STA,
        0b101_000_01 => Opcode::LDA,
        0b110_000_01 => Opcode::CMP,
        0b111_000_01 => Opcode::SBC,
        op => unreachable!("{op}"),
    }
}

pub fn decode_addressing_mode_xxx_bbb_01(opcode: Byte) -> AddressingModeType {
    match opcode & 0b000_111_11 {
        0b000_000_01 => AddressingModeType::IndexedIndirect,
        0b000_001_01 => AddressingModeType::ZeroPage,
        0b000_010_01 => AddressingModeType::Immediate,
        0b000_011_01 => AddressingModeType::Absolute,
        0b000_100_01 => AddressingModeType::IndirectIndexed,
        0b000_101_01 => AddressingModeType::ZeroPageIndexedWithX,
        0b000_110_01 => AddressingModeType::AbsoluteIndexedWithY,
        0b000_111_01 => AddressingModeType::AbsoluteIndexedWithX,
        op => unreachable!("{op}"),
    }
}

lazy_static! {
    static ref ALL_ADDRESSING_MODES: HashSet<AddressingModeType> = HashSet::from_iter(vec![
        AddressingModeType::Absolute,
        AddressingModeType::ZeroPage,
        AddressingModeType::Immediate,
        AddressingModeType::Absolute,
        AddressingModeType::IndirectIndexed,
        AddressingModeType::ZeroPageIndexedWithX,
        AddressingModeType::AbsoluteIndexedWithY,
        AddressingModeType::AbsoluteIndexedWithY,
    ]);
}

/// Add with Carry
fn adc(addressing_mode: AddressingModeType, mut registers: Registers, memory: Memory) {
}

/// Logical AND
fn and(addressing_mode: AddressingModeType, mut registers: Registers, memory: Memory) {

}

/// Compare
fn cmp(addressing_mode: AddressingModeType, mut registers: Registers, memory: Memory) {

}

fn eor(addressing_mode: AddressingModeType, mut registers: Registers, memory: Memory) {

}

fn lda(addressing_mode: AddressingModeType, mut registers: Registers, memory: Memory) {

}

fn ora(addressing_mode: AddressingModeType, mut registers: Registers, memory: Memory) {

}

fn sbc(addressing_mode: AddressingModeType, mut registers: Registers, memory: Memory) {

}

fn sta(addressing_mode: AddressingModeType, mut registers: Registers, memory: Memory) {

}