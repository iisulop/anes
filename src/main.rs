use std::ops::Range;

use bitvec::prelude::BitArray;
use paste::paste;

use crate::instructions::opcode::Memory;

mod instructions;

// https://llx.com/Neil/a2/opcodes.html

// https://en.wikipedia.org/wiki/MOS_Technology_6502#Addressing
// Addressing modes also include implied (1-byte instructions); absolute (3 bytes); indexed absolute
// (3 bytes); indexed zero-page (2 bytes); relative (2 bytes); accumulator (1); indirect,x and
// indirect,y (2); and immediate (2). Absolute mode is a general-purpose mode. Branch instructions
// use a signed 8-bit offset relative to the instruction after the branch; the numerical range
// −128..127 therefore translates to 128 bytes backward and 127 bytes forward from the instruction
// following the branch (which is 126 bytes backward and 129 bytes forward from the start of the
// branch instruction). Accumulator mode uses the accumulator as an effective address and does not
// need any operand data. Immediate mode uses an 8-bit literal operand.

// 6502 instruction operation codes (opcodes) are 8 bits long and have the general form AAABBBCC,
// where AAA and CC define the opcode.rs, and BBB defines the addressing mode.

// Page size: 256 bytes

const STACK_ADDRESS_SPACE: Range<u16> = Range {
    start: 0x100,
    end: 0x01ff,
};

type Byte = u8;
type Byte2 = u16;

struct StatusFlags(BitArray);

macro_rules! flag {
    ($name:literal, $shorthand:ident, $pos:expr) => {
        paste! {
            #[doc= " get " $name]
            fn [<get_ $shorthand>](&self) -> bool {
                self.0[$pos]
            }

            #[doc=" replace " $name]
            fn [<replace_ $shorthand>](&mut self, val: bool) -> bool {
                self.0.replace($pos, val)
            }
        }
    };
}

impl StatusFlags {
    fn new() -> Self {
        StatusFlags(bitvec::bitarr!(0; 8))
    }
    // from bit 7 to bit 0 these are the negative (N), overflow (V), reserved, break (B),
    // decimal (D), interrupt disable (I), zero (Z) and carry (C) flag

    flag!("negative", n, 7);
    flag!("overflow", v, 6);
    flag!("break", b, 4);
    flag!("decimal", d, 3);
    flag!("interrupt disable", i, 2);
    flag!("zero", z, 1);
    flag!("carry", c, 0);
}

struct Registers {
    /// accumulator
    a: Byte,
    /// Index register X
    x: Byte,
    /// Index register Y
    y: Byte,
    /// Status byte flags
    status: StatusFlags,
    /// Stack pointer
    s: Byte,
    /// Program counter
    p: Byte2,
}

struct Instruction {
    opcode: Opcode,
    addressing_mode: AddressingModeType,
}

enum AddressingValue {
    /// Accumulator: A
    /// Like implied addressing but object being the accumulator
    Accumulator,
    /// Implied
    /// No memory reference
    Implied,
    /// Immediate: #aa
    /// The given value is used as is
    Immediate(Byte),
    /// Absolute: aaaa
    /// The given value is the memory address of the 8 bit value to use
    Absolute(Byte2),
    /// Zero Page: aa
    /// Reference to one of the first 256 memory locations
    ZeroPage(Byte),
    /// Relative: aaaa
    Relative(Byte),
    /// Indirect Absolute: (aaaa)
    /// Used by JMP: Uses the given address as a pointer to the low part of a 16-bit address
    AbsoluteIndirect(Byte2),
    /// Absolute Indexed with X: aaaa,X
    /// The given address is used as a base and the value of X register is added as an offset
    AbsoluteIndexedWithX(Byte2),
    /// Absolute Indexed with Y: aaaa,Y
    /// The given address is used as a base and the value of Y register is added as an offset
    AbsoluteIndexedWithY(Byte2),
    /// Zero Page Indexed with X: aa,X
    /// As absolute indexed with X but in the Zero page memory
    ZeroPageIndexedWithX(Byte),
    /// Zero Page Indexed with Y: aa,Y
    /// As absolute indexed with Y but in the Zero page memory
    ZeroPageIndexedWithY(Byte),
    /// Indexed Indirect Addressing: (aa,X)
    /// Given location +X points to the 16-bit address containing a 16-bit pointer to the value
    IndexedIndirect(Byte),
    /// Indirect Indexed Addressing: (aa),Y
    /// Given location points to a 16-bit address containing a 16 bit pointer to the value after Y is added to it
    IndirectIndexed(Byte),
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum AddressingModeType {
    Accumulator,
    Implied,
    Immediate,
    Absolute,
    ZeroPage,
    AbsoluteIndirect,
    AbsoluteIndexedWithX,
    AbsoluteIndexedWithY,
    ZeroPageIndexedWithX,
    ZeroPageIndexedWithY,
    IndexedIndirect,
    IndirectIndexed,
}

pub struct AddressingModeBuilder {
    addressing_mode: AddressingModeType
}

impl AddressingValue {
    pub fn get_value(
        &self,
        registers: Registers,
        memory: &Memory
    ) -> Option<Byte> {
        match self {
            AddressingValue::Accumulator => Some(registers.a),
            AddressingValue::Implied => None,
            AddressingValue::Immediate(value) => Some(*value),
            AddressingValue::Absolute(value) => {
                Some(get_value_from_absolute(*value, memory))
            },
            AddressingValue::ZeroPage(value) => {
                Some(get_value_from_zero_page(*value, memory))
            },
            AddressingValue::Relative(value) => {
                Some(*value)
            },
            AddressingValue::AbsoluteIndirect(value) => {
                Some(get_value_from_absolute_indirect(*value, memory))
            },
            AddressingValue::AbsoluteIndexedWithX(value) => {
                Some(get_value_from_absolute_indexed_with_register(*value, registers.x, memory))
            }
            AddressingValue::AbsoluteIndexedWithY(value) => {
                Some(get_value_from_absolute_indexed_with_register(*value, registers.y, memory))
            },
            AddressingValue::ZeroPageIndexedWithX(value) => {
                Some(get_value_from_zero_page_indexed_with_register(*value, registers.x, memory))
            },
            AddressingValue::ZeroPageIndexedWithY(value) => {
                Some(get_value_from_zero_page_indexed_with_register(*value, registers.y, memory))
            }
            AddressingValue::IndexedIndirect(value) => {
                Some(get_value_from_indexed_indirect(*value, registers, memory))
            }
            AddressingValue::IndirectIndexed(value) => {
                Some(get_value_from_indirect_indexed(*value, registers, memory))
            }
        }
    }
}

fn get_value_from_indirect_indexed(value: Byte, registers: Registers, memory: &Memory) -> Byte {
    let target_low = get_value_from_absolute(value as Byte2, memory);
    let target_high = get_value_from_absolute(value as Byte2 + 1, memory);
    let target: Byte2 = ((target_high as Byte2) << 8) | target_low as Byte2;
    let target = target + registers.y as Byte2;
    get_value_from_absolute(target, memory)
}

fn get_value_from_indexed_indirect(value: Byte, registers: Registers, memory: &Memory) -> Byte {
    let location = value as Byte2 + registers.x as Byte2;
    let target_low = get_value_from_absolute(location, memory);
    let target_high = get_value_from_absolute(location + 1, memory);
    let target: Byte2 = ((target_high as Byte2) << 8) | target_low as Byte2;
    get_value_from_absolute(target, memory)
}

fn get_value_from_zero_page_indexed_with_register(value: Byte, register_contents: Byte, memory: &Memory) -> Byte {
    *memory.0.get(value as usize + (register_contents as usize)).unwrap()
}

fn get_value_from_absolute_indexed_with_register(value: Byte2, register_contents: Byte, memory: &Memory) -> Byte {
    *memory.0.get(value as usize + register_contents as usize).unwrap()
}

fn get_value_from_absolute_indirect(value: Byte2, memory: &Memory) -> Byte {
    let target_low = get_value_from_absolute(value, memory);
    let target_high = get_value_from_absolute(value + 1, memory);
    let target: Byte2 = ((target_high as Byte2) << 8) | target_low as Byte2;
    get_value_from_absolute(target, memory)
}

fn get_value_from_zero_page(value: Byte, memory: &Memory) -> Byte {
    *memory.0.get(value as usize).unwrap()
}

fn get_value_from_absolute(
    value: Byte2,
    memory: &Memory
) -> Byte {
    *memory.0.get(value as usize).unwrap()
}

pub enum Opcode {
    /// Add memory to accumulator with carry
    ADC,
    /// And memory with accumulator
    AND,
    /// Arithmetic shift left one bit
    ASL,
    /// Compare memory and accumulator
    CMP,
    /// Decrement memory by one
    DEC,
    /// Exclusive or memory with accumulator
    EOR,
    /// Increment memory by one
    INC,
    /// Load accumulator with memory
    LDA,
    /// Load index X with memory
    LDX,
    /// Logical shift right one bit
    LSR,
    /// Or memory with accumulator
    ORA,
    /// Rotate left one bit
    ROL,
    /// Rotate right one bit
    ROR,
    /// Subtract memory from accumulator with borrow
    SBC,
    /// Store accumulator in memory
    STA,
    /// Store index X in memory
    STX,

    BIT,
    CPX,
    CPY,
    JMP,
    JMP_ABS,
    LDY,
    STY,

    // Conditional Branches
    /// Branch on result plus
    BPL,
    /// Branch on carry clear
    BCC,
    /// Branch on carry set
    BCS,
    /// Branch on result zero
    BEQ,
    /// Branch on result minus
    BMI,
    /// Branch on result not zero
    BNE,
    /// Branch on overflow clear
    BVC,
    /// Branch on overflow set
    BVS,

    /// Break
    BRK,
    /// Clear carry
    CLC,
    /// Clear decimal
    CLD,
    /// Clear interrupt
    CLI,
    /// Clear overflow
    CLV,
    /// Decrement X
    DEX,
    /// Decrement Y
    DEY,
    /// Increment X
    INX,
    /// Increment Y
    INY,
    /// Shuould this just be JSR with abs?
    JSR,
    ///
    NOP,
    /// Push accumulator
    PHA,
    /// Push processor status
    PHP,
    /// Pull accumulator
    PLA,
    /// Pull processor status
    PLP,
    /// Return from interrupt
    RTI,
    /// Return from subroutine
    RTS,
    /// Set carry
    SEC,
    /// Set decimal
    SED,
    /// Set interrupt
    SEI,
    /// Transfer A to X
    TAX,
    /// Transfer A to Y
    TAY,
    /// Transfer stack pointer to X
    TSX,
    /// Transfer X to A
    TXA,
    /// Transfer X to stack pointer
    TXS,
    /// Transfer Y to A
    TYA,
}

fn read_opcode(memory: &[Byte]) -> () {
}

fn main() {
    println!("Hello, world!");
}
