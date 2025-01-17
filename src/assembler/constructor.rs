use either::Either::{self, Left, Right};
use crate::assembler::register::Register;

#[derive(Debug, Copy, Clone)]
pub struct U2(u8);

#[allow(dead_code)]
impl U2 {
    pub const MAX: u8 = 4;

    pub fn new(value: u8) -> Option<U2> {
        if value < 4 {
            Some( U2(value))
        } else {
            None
        }
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct U3(u8);

#[allow(dead_code)]
impl U3 {
    pub const MAX: u8 = 8;

    pub fn new(value: u8) -> Option<U3> {
        if value < 8 {
            Some( U3(value))
        } else {
            None
        }
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct U6(u8);

#[allow(dead_code)]
impl U6 {
    pub const MAX: u8 = 64;

    pub fn new(value: u8) -> Option<U6> {
        if value < 64 {
            Some( U6(value))
        } else {
            None
        }
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}

#[repr(u32)]
enum InstrType {
    Arithmetic = 0x1,
    Bitwise = 0x2,
    ShiftRotate = 0x3,
    DataMemoryStack = 0x4,
    Comparison = 0x5,
    Branching = 0x6,
    Conversion = 0x7,
    FloatingArithmetic = 0x8,
    DoubleArithmetic = 0x9,
}

pub enum BranchType {
    Unconditional,
    Greater,
    Equal,
    Smaller,
    GreaterEqual,
    NotEqual,
    SmallerEqual,
}

impl BranchType {
    fn construct(&self, offset: Either<Register, i16>) -> Instruction {
        match (self, offset) {
            (Self::Unconditional, offset) => Instruction::Branch { offset },
            (Self::Greater, offset) => Instruction::BranchGreater { offset },
            (Self::Equal, offset) => Instruction::BranchEqual { offset },
            (Self::Smaller, offset) => Instruction::BranchSmaller { offset },
            (Self::GreaterEqual, offset) => Instruction::BranchGreaterEqual { offset },
            (Self::NotEqual, offset) => Instruction::BranchNotEqual { offset },
            (Self::SmallerEqual, offset) => Instruction::BranchSmallerEqual { offset },
        }
    }
}

impl Into<u32> for InstrType {
    fn into(self) -> u32 {
        self as u32
    }
}

impl Into<u32> for Register {
    fn into(self) -> u32 {
        self as u32
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Nop,
    Add { dest: Register, a: Either<Register, u16>, b: Either<Register, u16> },
    Subtract { dest: Register, a: Either<Register, u16>, b: Either<Register, u16> },
    Multiply { dest: Register, a: Either<Register, u16>, b: Either<Register, u16> },
    Divide { dest: Register, a: Either<Register, u16>, b: Either<Register, u16> },
    DivideSigned { dest: Register, a: Either<Register, u16>, b: Either<Register, u16> },
    And { dest: Register, a: Register, b: Register },
    Or { dest: Register, a: Register, b: Register },
    Xor { dest: Register, a: Register, b: Register },
    Nand { dest: Register, a: Register, b: Register },
    Nor { dest: Register, a: Register, b: Register },
    Xnor { dest: Register, a: Register, b: Register },
    Not { dest: Register, src: Register },
    RightShift { dest: Register, src: Register, amount: Either<Register, U6> },
    LeftShift { dest: Register, src: Register, amount: Either<Register, U6> },
    RightRoll { dest: Register, src: Register, amount: Either<Register, U6> },
    LeftRoll { dest: Register, src: Register, amount: Either<Register, U6> },
    Move { dest: Register, src: Register },
    LoadImmediate { dest: Register, slice: U2, imm: u16 },
    LoadRegister { dest: Register, mem_ptr: Either<Register, u16>, slice: U3 },
    StoreRegister { src: Register, mem_ptr: Either<Register, u16>, slice: U3 },
    Push { reg: Register },
    Pop { reg: Register },
    Compare { a: Either<Register, u16>, b: Either<Register, u16>, signed: bool },
    CompareFloat { a: Register, b: Register },
    CompareDouble { a: Register, b: Register },
    Branch { offset: Either<Register, i16> },
    BranchGreater { offset: Either<Register, i16> },
    BranchEqual { offset: Either<Register, i16> },
    BranchSmaller { offset: Either<Register, i16> },
    BranchGreaterEqual { offset: Either<Register, i16> },
    BranchNotEqual { offset: Either<Register, i16> },
    BranchSmallerEqual { offset: Either<Register, i16> },
    ImmediateToFloat { dest: Register, imm: i16 },
    ImmediateToDouble { dest: Register, imm: i16 },
    IntegerToFloat { dest: Register, src: Register },
    IntegerToDouble { dest: Register, src: Register },
    FloatToInteger { dest: Register, src: Register },
    FloatToDouble { dest: Register, src: Register },
    DoubleToInteger { dest: Register, src: Register },
    DoubleToFloat { dest: Register, src: Register },
}


impl Instruction {
    fn denibble(nibbles: [u32; 8]) -> u32 {
        const MASKS: [u32; 8] = [
            0xF0000000,
            0x0F000000,
            0x00F00000,
            0x000F0000,
            0x0000F000,
            0x00000F00,
            0x000000F0,
            0x0000000F,
        ];

        let mut out = 0;

        nibbles
            .iter()
            .enumerate()
            .map(|(i, n)| (n << MASKS[i].trailing_zeros()) & MASKS[i])
            .for_each(|n| out |= n);

        out
    }

    fn nibbles_u16(v: u16) -> (u32, u32, u32, u32) {
        const MASKS: [u32; 4] = [
            0xF000,
            0x0F00,
            0x00F0,
            0x000F,
        ];

        let v = v as u32;

        (
            (v & MASKS[0]) >> MASKS[0].trailing_zeros(),
            (v & MASKS[1]) >> MASKS[1].trailing_zeros(),
            (v & MASKS[2]) >> MASKS[2].trailing_zeros(),
            (v & MASKS[3]) >> MASKS[3].trailing_zeros(),
        )
    }

    fn nibbles_u6(v: U6) -> (u32, u32) {
        const MASKS: [u32; 2] = [
            0xF0,
            0x0F,
        ];

        let v = v.get() as u32;

        (
            (v & MASKS[0]) >> MASKS[0].trailing_zeros(),
            (v & MASKS[1]) >> MASKS[1].trailing_zeros(),
        )
    }

    pub fn assemble(self) -> u32 {
        match self {
            Instruction::Nop => 0,

            // Arithmetic
            Instruction::Add { dest, a, b } => {
                match (a, b) {
                    (Left(r1), Left(r2)) => Self::denibble([InstrType::Arithmetic.into(), dest.into(), r1.into(), r2.into(), 0, 0, 0, 0x0]),
                    (Left(r), Right(imm)) | (Right(imm), Left(r)) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm);
                        Self::denibble([InstrType::Arithmetic.into(), dest.into(), r.into(), n0, n1, n2, n3, 0x1])
                    }
                    _ => panic!("bad code"),
                }
            }
            Instruction::Subtract { dest, a, b } => {
                match (a, b) {
                    (Left(r1), Left(r2)) => Self::denibble([InstrType::Arithmetic.into(), dest.into(), r1.into(), r2.into(), 0, 0, 0, 0x2]),
                    (Left(r), Right(imm)) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm);
                        Self::denibble([InstrType::Arithmetic.into(), dest.into(), r.into(), n0, n1, n2, n3, 0x3])
                    }
                    (Right(imm), Left(r)) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm);
                        Self::denibble([InstrType::Arithmetic.into(), dest.into(), r.into(), n0, n1, n2, n3, 0x4])
                    }
                    _ => panic!("bad code"),
                }
            }
            Instruction::Multiply { dest, a, b } => {
                match (a, b) {
                    (Left(r1), Left(r2)) => Self::denibble([InstrType::Arithmetic.into(), dest.into(), r1.into(), r2.into(), 0, 0, 0, 0x5]),
                    (Left(r), Right(imm)) | (Right(imm), Left(r)) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm);
                        Self::denibble([InstrType::Arithmetic.into(), dest.into(), r.into(), n0, n1, n2, n3, 0x6])
                    }
                    _ => panic!("bad code"),
                }
            }
            Instruction::Divide { dest, a, b } => {
                match (a, b) {
                    (Left(r1), Left(r2)) => Self::denibble([InstrType::Arithmetic.into(), dest.into(), r1.into(), r2.into(), 0, 0, 0, 0x7]),
                    (Left(r), Right(imm)) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm);
                        Self::denibble([InstrType::Arithmetic.into(), dest.into(), r.into(), n0, n1, n2, n3, 0x8])
                    }
                    (Right(imm), Left(r)) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm);
                        Self::denibble([InstrType::Arithmetic.into(), dest.into(), r.into(), n0, n1, n2, n3, 0x9])
                    }
                    (Right(_), Right(_)) => panic!("bad code")
                }
            }
            Instruction::DivideSigned { dest, a, b } => {
                match (a, b) {
                    (Left(r1), Left(r2)) => Self::denibble([InstrType::Arithmetic.into(), dest.into(), r1.into(), r2.into(), 0, 0, 0, 0xA]),
                    (Left(r), Right(imm)) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm);
                        Self::denibble([InstrType::Arithmetic.into(), dest.into(), r.into(), n0, n1, n2, n3, 0xB])
                    }
                    (Right(imm), Left(r)) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm);
                        Self::denibble([InstrType::Arithmetic.into(), dest.into(), r.into(), n0, n1, n2, n3, 0xC])
                    }
                    (Right(_), Right(_)) => panic!("bad code")
                }
            }

            // Bitwise
            Instruction::And { dest, a, b } => Self::denibble([InstrType::Bitwise.into(), dest.into(), a.into(), b.into(), 0, 0, 0, 0x0]),
            Instruction::Or { dest, a, b } => Self::denibble([InstrType::Bitwise.into(), dest.into(), a.into(), b.into(), 0, 0, 0, 0x1]),
            Instruction::Xor { dest, a, b } => Self::denibble([InstrType::Bitwise.into(), dest.into(), a.into(), b.into(), 0, 0, 0, 0x2]),
            Instruction::Nand { dest, a, b } => Self::denibble([InstrType::Bitwise.into(), dest.into(), a.into(), b.into(), 0, 0, 0, 0x3]),
            Instruction::Nor { dest, a, b } => Self::denibble([InstrType::Bitwise.into(), dest.into(), a.into(), b.into(), 0, 0, 0, 0x4]),
            Instruction::Xnor { dest, a, b } => Self::denibble([InstrType::Bitwise.into(), dest.into(), a.into(), b.into(), 0, 0, 0, 0x5]),
            Instruction::Not { dest, src } => Self::denibble([InstrType::Bitwise.into(), dest.into(), src.into(), 0, 0, 0, 0, 0x6]),

            // Shift & Rotate
            Instruction::RightShift { dest, src, amount } => {
                match amount {
                    Left(reg) => Self::denibble([InstrType::ShiftRotate.into(), dest.into(), src.into(), reg.into(), 0, 0, 0, 0x0]),
                    Right(u6) => {
                        let (n0, n1) = Self::nibbles_u6(u6);
                        Self::denibble([InstrType::ShiftRotate.into(), dest.into(), src.into(), 0, 0, n0, n1, 0x1])
                    }
                }
            }
            Instruction::LeftShift { dest, src, amount } => {
                match amount {
                    Left(reg) => Self::denibble([InstrType::ShiftRotate.into(), dest.into(), src.into(), reg.into(), 0, 0, 0, 0x2]),
                    Right(u6) => {
                        let (n0, n1) = Self::nibbles_u6(u6);
                        Self::denibble([InstrType::ShiftRotate.into(), dest.into(), src.into(), 0, 0, n0, n1, 0x3])
                    }
                }
            }
            Instruction::RightRoll { dest, src, amount } => {
                match amount {
                    Left(reg) => Self::denibble([InstrType::ShiftRotate.into(), dest.into(), src.into(), reg.into(), 0, 0, 0, 0x4]),
                    Right(u6) => {
                        let (n0, n1) = Self::nibbles_u6(u6);
                        Self::denibble([InstrType::ShiftRotate.into(), dest.into(), src.into(), 0, 0, n0, n1, 0x5])
                    }
                }
            }
            Instruction::LeftRoll { dest, src, amount } => {
                match amount {
                    Left(reg) => Self::denibble([InstrType::ShiftRotate.into(), dest.into(), src.into(), reg.into(), 0, 0, 0, 0x6]),
                    Right(u6) => {
                        let (n0, n1) = Self::nibbles_u6(u6);
                        Self::denibble([InstrType::ShiftRotate.into(), dest.into(), src.into(), 0, 0, n0, n1, 0x7])
                    }
                }
            }

            // Data movement, Memory, Stack
            Instruction::Move { dest, src } => Self::denibble([InstrType::DataMemoryStack.into(), dest.into(), src.into(), 0, 0, 0, 0, 0x0]),
            Instruction::LoadImmediate { dest, slice, imm } => {
                let (n0, n1, n2, n3) = Self::nibbles_u16(imm);
                Self::denibble([InstrType::DataMemoryStack.into(), dest.into(), n0, n1, n2, n3, slice.get() as u32, 0x1])
            }
            Instruction::LoadRegister { dest, mem_ptr, slice } => {
                match mem_ptr {
                    Left(reg) => Self::denibble([InstrType::DataMemoryStack.into(), dest.into(), reg.into(), 0, 0, 0, slice.get() as u32, 0x2]),
                    Right(imm) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm);
                        Self::denibble([InstrType::DataMemoryStack.into(), dest.into(), n0, n1, n2, n3, slice.get() as u32, 0x3])
                    }
                }
            }
            Instruction::StoreRegister { src, mem_ptr, slice } => {
                match mem_ptr {
                    Left(reg) => Self::denibble([InstrType::DataMemoryStack.into(), src.into(), reg.into(), 0, 0, 0, slice.get() as u32, 0x4]),
                    Right(imm) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm);
                        Self::denibble([InstrType::DataMemoryStack.into(), src.into(), n0, n1, n2, n3, slice.get() as u32, 0x5])
                    }
                }
            }
            Instruction::Push { reg } => Self::denibble([InstrType::DataMemoryStack.into(), reg.into(), 0, 0, 0, 0, 0, 0x6]),
            Instruction::Pop { reg } => Self::denibble([InstrType::DataMemoryStack.into(), reg.into(), 0, 0, 0, 0, 0, 0x7]),

            // Comparison
            Instruction::Compare { a, b, signed } => {
                match (signed, a, b) {
                    (false, Left(reg1), Left(reg2)) => Self::denibble([InstrType::Comparison.into(), reg1.into(), reg2.into(), 0, 0, 0, 0, 0x0]),
                    (false, Left(reg), Right(imm)) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm);
                        Self::denibble([InstrType::Comparison.into(), reg.into(), n0, n1, n2, n3, 0, 0x1])
                    }
                    (false, Right(imm), Left(reg)) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm);
                        Self::denibble([InstrType::Comparison.into(), reg.into(), n0, n1, n2, n3, 0, 0x2])
                    }
                    (false, Right(_), Right(_)) => panic!("bad code"),
                    (true, Left(reg1), Left(reg2)) => Self::denibble([InstrType::Comparison.into(), reg1.into(), reg2.into(), 0, 0, 0, 0, 0x3]),
                    (true, Left(reg), Right(imm)) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm);
                        Self::denibble([InstrType::Comparison.into(), reg.into(), n0, n1, n2, n3, 0, 0x4])
                    }
                    (true, Right(imm), Left(reg)) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm);
                        Self::denibble([InstrType::Comparison.into(), reg.into(), n0, n1, n2, n3, 0, 0x5])
                    }
                    (true, Right(_), Right(_)) => panic!("bad code"),
                }
            }
            Instruction::CompareFloat { a, b } => Self::denibble([InstrType::Comparison.into(), a.into(), b.into(), 0, 0, 0, 0, 0x6]),
            Instruction::CompareDouble { a, b } => Self::denibble([InstrType::Comparison.into(), a.into(), b.into(), 0, 0, 0, 0, 0x7]),

            // Branching
            Instruction::Branch { offset: amount } => {
                match amount {
                    Left(reg) => Self::denibble([InstrType::Branching.into(), reg.into(), 0, 0, 0, 0, 0, 0x0]),
                    Right(imm) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm as u16);
                        Self::denibble([InstrType::Branching.into(), n0, n1, n2, n3, 0, 0, 0x1])
                    }
                }
            }
            Instruction::BranchGreater { offset: amount } => {
                match amount {
                    Left(reg) => Self::denibble([InstrType::Branching.into(), reg.into(), 0, 0, 0, 0, 0, 0x2]),
                    Right(imm) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm as u16);
                        Self::denibble([InstrType::Branching.into(), n0, n1, n2, n3, 0, 0, 0x3])
                    }
                }
            }
            Instruction::BranchEqual { offset: amount } => {
                match amount {
                    Left(reg) => Self::denibble([InstrType::Branching.into(), reg.into(), 0, 0, 0, 0, 0, 0x4]),
                    Right(imm) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm as u16);
                        Self::denibble([InstrType::Branching.into(), n0, n1, n2, n3, 0, 0, 0x5])
                    }
                }
            }
            Instruction::BranchSmaller { offset: amount } => {
                match amount {
                    Left(reg) => Self::denibble([InstrType::Branching.into(), reg.into(), 0, 0, 0, 0, 0, 0x6]),
                    Right(imm) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm as u16);
                        Self::denibble([InstrType::Branching.into(), n0, n1, n2, n3, 0, 0, 0x7])
                    }
                }
            }
            Instruction::BranchGreaterEqual { offset: amount } => {
                match amount {
                    Left(reg) => Self::denibble([InstrType::Branching.into(), reg.into(), 0, 0, 0, 0, 0, 0x8]),
                    Right(imm) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm as u16);
                        Self::denibble([InstrType::Branching.into(), n0, n1, n2, n3, 0, 0, 0x9])
                    }
                }
            }
            Instruction::BranchNotEqual { offset: amount } => {
                match amount {
                    Left(reg) => Self::denibble([InstrType::Branching.into(), reg.into(), 0, 0, 0, 0, 0, 0xA]),
                    Right(imm) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm as u16);
                        Self::denibble([InstrType::Branching.into(), n0, n1, n2, n3, 0, 0, 0xB])
                    }
                }
            }
            Instruction::BranchSmallerEqual { offset: amount } => {
                match amount {
                    Left(reg) => Self::denibble([InstrType::Branching.into(), reg.into(), 0, 0, 0, 0, 0, 0xC]),
                    Right(imm) => {
                        let (n0, n1, n2, n3) = Self::nibbles_u16(imm as u16);
                        Self::denibble([InstrType::Branching.into(), n0, n1, n2, n3, 0, 0, 0xD])
                    }
                }
            }

            // Conversions
            Instruction::ImmediateToFloat { dest, imm } => {
                let (n0, n1, n2, n3) = Self::nibbles_u16(imm as u16);
                Self::denibble([InstrType::Conversion.into(), dest.into(), n0, n1, n2, n3, 0, 0x0])
            }
            Instruction::ImmediateToDouble { dest, imm } => {
                let (n0, n1, n2, n3) = Self::nibbles_u16(imm as u16);
                Self::denibble([InstrType::Conversion.into(), dest.into(), n0, n1, n2, n3, 0, 0x1])
            }
            Instruction::IntegerToFloat { dest, src } => Self::denibble([InstrType::Conversion.into(), dest.into(), src.into(), 0, 0, 0, 0, 0x2]),
            Instruction::IntegerToDouble { dest, src } => Self::denibble([InstrType::Conversion.into(), dest.into(), src.into(), 0, 0, 0, 0, 0x3]),
            Instruction::FloatToInteger { dest, src } => Self::denibble([InstrType::Conversion.into(), dest.into(), src.into(), 0, 0, 0, 0, 0x4]),
            Instruction::FloatToDouble { dest, src } => Self::denibble([InstrType::Conversion.into(), dest.into(), src.into(), 0, 0, 0, 0, 0x5]),
            Instruction::DoubleToInteger { dest, src } => Self::denibble([InstrType::Conversion.into(), dest.into(), src.into(), 0, 0, 0, 0, 0x6]),
            Instruction::DoubleToFloat { dest, src } => Self::denibble([InstrType::Conversion.into(), dest.into(), src.into(), 0, 0, 0, 0, 0x7]),
        }
    }
}
