use std::ops::Range;

use bitvec::prelude::BitArray;
use paste::paste;

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
// where AAA and CC define the opcode, and BBB defines the addressing mode.

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
    addressing_mode: AddressingMode,
}

enum AddressingMode {
    /// A
    Accumulator,
    /// i
    Implied,
    /// #
    Immediate,
    /// a
    Absolute,
    /// zp
    ZeroPage,
    /// r
    Relative,
    /// a
    AbsoluteIndirect,
    /// a,x
    AbsoluteIndexedWithX,
    /// a,y
    AbsoluteIndexedWithY,
    /// zp,x
    ZeroPageIndexedWithX,
    /// zp,y
    ZeroPageIndexedWithY,
    /// (zp,x)
    ZeroPageIndexedIndirectWithX,
    /// (zp,y)
    ZeroPageIndexedIndirectWithY,
}

enum Opcode {
    /// Or memory with accumulator
    ORA,
    /// And memory with accumulator
    AND,
    /// Exclusive or memory with accumulator
    EOR,
    /// Add memory to accumulator with carry
    ADC,
    /// Store accumulator in memory
    STA,
    /// Load accumulator with memory
    LDA,
    /// Compare memory and accumulator
    CMP,
    /// Subtract memory from accumulator with borrow
    SBC,
    /// Arithmetic shift left one bit
    ASL,
    /// Rotate left one bit
    ROL,
    /// Logical shift right one bit
    LSR,
    /// Rotate right one bit
    ROR,
    /// Store index X in memory
    STX,
    /// Load index X with memory
    LDX,
    /// Decrement memory by one
    DEC,
    /// Increment memory by one
    INC,
    BIT,
    JMP,
    JMP_ABS,
    STY,
    LDY,
    CPY,
    CPX,

    // Conditional Branches
    /// Branch on result plus
    BPL,
    /// Branch on result minus
    BMI,
    /// Branch on overflow clear
    BVC,
    /// Branch on overflow set
    BVS,
    /// Branch on carry clear
    BCC,
    /// Branch on carry set
    BCS,
    /// Branch on result not zero
    BNE,
    /// Branch on result zero
    BEQ,

    BRK,
    JSR_ABS,
    RTI,
    RTS,
    PHP,
    PLP,
    PHA,
    PLA,
    DEY,
    TAY,
    INY,
    INX,
    CLC,
    SEC,
    CLI,
    SEI,
    TYA,
    CLV,
    CLD,
    SED,
    TXA,
    TXS,
    TAX,
    TSX,
    DEX,
    NOP,
}

/// "Group One" instructions
fn decode_opcode_aaa_xxx_01(opcode: Byte) -> Opcode {
    match opcode & 0b11100011 {
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

fn decode_addressing_mode_xxx_bbb_01(opcode: Byte) -> AddressingMode {
    match opcode & 0b00011111 {
        0b000_000_01 => AddressingMode::ZeroPageIndexedIndirectWithX,
        0b000_001_01 => AddressingMode::ZeroPage,
        0b000_010_01 => AddressingMode::Immediate,
        0b000_011_01 => AddressingMode::Absolute,
        0b000_100_01 => AddressingMode::ZeroPageIndexedIndirectWithY,
        0b000_101_01 => AddressingMode::ZeroPageIndexedWithX,
        0b000_110_01 => AddressingMode::AbsoluteIndexedWithY,
        0b000_111_01 => AddressingMode::AbsoluteIndexedWithX,
        op => unreachable!("{op}"),
    }
}

/// "Group Two" instructions
fn decode_opcode_aaa_xxx_10(opcode: Byte) -> Opcode {
    match opcode & 0b11100011 {
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

fn decode_addressing_mode_xxx_bbb_10(opcode: Byte) -> AddressingMode {
    match opcode & 0b00011111 {
        0b000_000_10 => AddressingMode::Immediate,
        0b000_001_10 => AddressingMode::ZeroPage,
        0b000_010_10 => AddressingMode::Accumulator,
        0b000_011_10 => AddressingMode::Absolute,
        // With STX and LDX this is ZeroPageIndexedWithX
        0b000_101_10 => AddressingMode::ZeroPageIndexedWithX,
        // With LDX this is AbsoluteIndexedWithY,
        0b000_111_10 => AddressingMode::AbsoluteIndexedWithX,
        op => unreachable!("{op}"),
    }
}

/// "Group Three"" instructions
fn decode_opcode_aaa_xxx_00(opcode: Byte) -> Opcode {
    match opcode & 0b11100011 {
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

fn decode_addressing_mode_xxx_bbb_00(opcode: Byte) -> AddressingMode {
    match opcode & 0b00011111 {
        0b000_000_00 => AddressingMode::Immediate,
        0b000_001_00 => AddressingMode::ZeroPage,
        0b000_011_00 => AddressingMode::Absolute,
        // With STX and LDX this is ZeroPageIndexedWithX
        0b000_101_00 => AddressingMode::ZeroPageIndexedWithX,
        // With LDX this is AbsoluteIndexedWithY,
        0b000_111_00 => AddressingMode::AbsoluteIndexedWithX,
        op => unreachable!("{op}"),
    }
}

/// Conditional branch instructions
fn decode_opcode_xx_y_10000(opcode: Byte) -> Opcode {
    match opcode & 0b11100011 {
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

///  Interrupt and subroutine instructionss
fn decode_opcode_interrupt_subroutine(opcode: Byte) -> Opcode {
    match opcode {
        0b0000_0000 => Opcode::BRK,
        0b0001_0000 => Opcode::JSR_ABS,
        0b0010_0000 => Opcode::RTI,
        0b0100_0000 => Opcode::RTS,
        op => unreachable!("{op}"),
    }
}

fn decode_other_instruction(opcode: Byte) -> Opcode {
    match opcode {
        0b0000_0000 => Opcode::PHP,
        0b0000_0000 => Opcode::PLP,
        0b0000_0000 => Opcode::PHA,
        0b0000_0000 => Opcode::PLA,
        0b0000_0000 => Opcode::DEY,
        0b0000_0000 => Opcode::TAY,
        0b0000_0000 => Opcode::INY,
        0b0000_0000 => Opcode::INX,
        0b0000_0000 => Opcode::CLC,
        0b0000_0000 => Opcode::SEC,
        0b0000_0000 => Opcode::CLI,
        0b0000_0000 => Opcode::SEI,
        0b0000_0000 => Opcode::TYA,
        0b0000_0000 => Opcode::CLV,
        0b0000_0000 => Opcode::CLD,
        0b0000_0000 => Opcode::SED,
        0b0000_0000 => Opcode::TXA,
        0b0000_0000 => Opcode::TXS,
        0b0000_0000 => Opcode::TAX,
        0b0000_0000 => Opcode::TSX,
        0b0000_0000 => Opcode::DEX,
        0b0000_0000 => Opcode::NOP,
        op => unreachable!("{op}"),
    }
}

fn main() {
    println!("Hello, world!");
}
