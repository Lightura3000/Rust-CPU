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
    FloatAdd { dest: Register, a: Register, b: Register },
    FloatSubtract { dest: Register, a: Register, b: Register },
    FloatMultiply { dest: Register, a: Register, b: Register },
    FloatDivide { dest: Register, a: Register, b: Register },
    FloatModulo { dest: Register, a: Register, b: Register },
    FloatNegate { dest: Register, src: Register },
    FloatReciprocal { dest: Register, src: Register },
    FloatPower { dest: Register, a: Register, b: Register },
    FloatExponential { dest: Register, src: Register },
    FloatRoot { dest: Register, a: Register, b: Register },
    FloatSquareRoot { dest: Register, src: Register },
    FloatCubeRoot { dest: Register, src: Register },
    FloatSquare { dest: Register, src: Register },
    FloatCube { dest: Register, src: Register },
    FloatLogarithm { dest: Register, a: Register, b: Register },
    FloatNaturalLogarithm { dest: Register, src: Register },
    FloatAbsolute { dest: Register, src: Register },
    FloatSine { dest: Register, src: Register },
    FloatCosine { dest: Register, src: Register },
    FloatTangent { dest: Register, src: Register },
    FloatArcsine { dest: Register, src: Register },
    FloatArccosine { dest: Register, src: Register },
    FloatArctangent { dest: Register, src: Register },
    FloatFloor { dest: Register, src: Register },
    FloatCeil { dest: Register, src: Register },
    FloatRound { dest: Register, src: Register },
    FloatMinimum { dest: Register, a: Register, b: Register },
    FloatMaximum { dest: Register, a: Register, b: Register },
    FloatSign { dest: Register, src: Register },
    FloatAbsoluteDifference { dest: Register, a: Register, b: Register },
    FloatLoadInfinity { dest: Register },
    FloatLoadNaN { dest: Register },
    DoubleAdd { dest: Register, a: Register, b: Register },
    DoubleSubtract { dest: Register, a: Register, b: Register },
    DoubleMultiply { dest: Register, a: Register, b: Register },
    DoubleDivide { dest: Register, a: Register, b: Register },
    DoubleModulo { dest: Register, a: Register, b: Register },
    DoubleNegate { dest: Register, src: Register },
    DoubleReciprocal { dest: Register, src: Register },
    DoublePower { dest: Register, a: Register, b: Register },
    DoubleExponential { dest: Register, src: Register },
    DoubleRoot { dest: Register, a: Register, b: Register },
    DoubleSquareRoot { dest: Register, src: Register },
    DoubleCubeRoot { dest: Register, src: Register },
    DoubleSquare { dest: Register, src: Register },
    DoubleCube { dest: Register, src: Register },
    DoubleLogarithm { dest: Register, a: Register, b: Register },
    DoubleNaturalLogarithm { dest: Register, src: Register },
    DoubleAbsolute { dest: Register, src: Register },
    DoubleSine { dest: Register, src: Register },
    DoubleCosine { dest: Register, src: Register },
    DoubleTangent { dest: Register, src: Register },
    DoubleArcsine { dest: Register, src: Register },
    DoubleArccosine { dest: Register, src: Register },
    DoubleArctangent { dest: Register, src: Register },
    DoubleFloor { dest: Register, src: Register },
    DoubleCeil { dest: Register, src: Register },
    DoubleRound { dest: Register, src: Register },
    DoubleMinimum { dest: Register, a: Register, b: Register },
    DoubleMaximum { dest: Register, a: Register, b: Register },
    DoubleSign { dest: Register, src: Register },
    DoubleAbsoluteDifference { dest: Register, a: Register, b: Register },
    DoubleLoadInfinity { dest: Register },
    DoubleLoadNaN { dest: Register },
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

            // Floating point arithmetic
            Instruction::FloatAdd { dest, a, b } => Self::construct_floating_instruction(dest, a, Some(b), 0x0, 0x0),
            Instruction::FloatSubtract { dest, a, b } => Self::construct_floating_instruction(dest, a, Some(b), 0x0, 0x1),
            Instruction::FloatMultiply { dest, a, b } => Self::construct_floating_instruction(dest, a, Some(b), 0x0, 0x2),
            Instruction::FloatDivide { dest, a, b } => Self::construct_floating_instruction(dest, a, Some(b), 0x0, 0x3),
            Instruction::FloatModulo { dest, a, b } => Self::construct_floating_instruction(dest, a, Some(b), 0x0, 0x4),
            Instruction::FloatNegate { dest, src } => Self::construct_floating_instruction(dest, src, None, 0x0, 0x5),
            Instruction::FloatReciprocal { dest, src } => Self::construct_floating_instruction(dest, src, None, 0x0, 0x6),
            Instruction::FloatPower { dest, a, b } => Self::construct_floating_instruction(dest, a, Some(b), 0x0, 0x7),
            Instruction::FloatExponential { dest, src } => Self::construct_floating_instruction(dest, src, None, 0x0, 0x8),
            Instruction::FloatRoot { dest, a, b } => Self::construct_floating_instruction(dest, a, Some(b), 0x0, 0x9),
            Instruction::FloatSquareRoot { dest, src } => Self::construct_floating_instruction(dest, src, None, 0x0, 0xA),
            Instruction::FloatCubeRoot { dest, src } => Self::construct_floating_instruction(dest, src, None, 0x0, 0xB),
            Instruction::FloatSquare { dest, src } => Self::construct_floating_instruction(dest, src, None, 0x0, 0xC),
            Instruction::FloatCube { dest, src } => Self::construct_floating_instruction(dest, src, None, 0x0, 0xD),
            Instruction::FloatLogarithm { dest, a, b } => Self::construct_floating_instruction(dest, a, Some(b), 0x0, 0xE),
            Instruction::FloatNaturalLogarithm { dest, src } => Self::construct_floating_instruction(dest, src, None, 0x0, 0xF),
            Instruction::FloatAbsolute { dest, src } => Self::construct_floating_instruction(dest, src, None, 0x1, 0x0),
            Instruction::FloatSine { dest, src } => Self::construct_floating_instruction(dest, src, None, 0x1, 0x1),
            Instruction::FloatCosine { dest, src } => Self::construct_floating_instruction(dest, src, None, 0x1, 0x2),
            Instruction::FloatTangent { dest, src } => Self::construct_floating_instruction(dest, src, None, 0x1, 0x3),
            Instruction::FloatArcsine { dest, src } => Self::construct_floating_instruction(dest, src, None, 0x1, 0x4),
            Instruction::FloatArccosine { dest, src } => Self::construct_floating_instruction(dest, src, None, 0x1, 0x5),
            Instruction::FloatArctangent { dest, src } => Self::construct_floating_instruction(dest, src, None, 0x1, 0x6),
            Instruction::FloatFloor { dest, src } => Self::construct_floating_instruction(dest, src, None, 0x1, 0x7),
            Instruction::FloatCeil { dest, src } => Self::construct_floating_instruction(dest, src, None, 0x1, 0x8),
            Instruction::FloatRound { dest, src } => Self::construct_floating_instruction(dest, src, None, 0x1, 0x9),
            Instruction::FloatMinimum { dest, a, b } => Self::construct_floating_instruction(dest, a, Some(b), 0x1, 0xA),
            Instruction::FloatMaximum { dest, a, b } => Self::construct_floating_instruction(dest, a, Some(b), 0x1, 0xB),
            Instruction::FloatSign { dest, src } => Self::construct_floating_instruction(dest, src, None, 0x1, 0xC),
            Instruction::FloatAbsoluteDifference { dest, a, b } => Self::construct_floating_instruction(dest, a, Some(b), 0x1, 0xD),
            Instruction::FloatLoadInfinity { dest } => Self::denibble([InstrType::FloatingArithmetic.into(), dest.into(), 0, 0, 0, 0, 0x1, 0xE]),
            Instruction::FloatLoadNaN { dest } => Self::denibble([InstrType::FloatingArithmetic.into(), dest.into(), 0, 0, 0, 0, 0x1, 0xF]),

            // Double precision arithmetic
            Instruction::DoubleAdd { dest, a, b } => Self::construct_double_instruction(dest, a, Some(b), 0x0, 0x0),
            Instruction::DoubleSubtract { dest, a, b } => Self::construct_double_instruction(dest, a, Some(b), 0x0, 0x1),
            Instruction::DoubleMultiply { dest, a, b } => Self::construct_double_instruction(dest, a, Some(b), 0x0, 0x2),
            Instruction::DoubleDivide { dest, a, b } => Self::construct_double_instruction(dest, a, Some(b), 0x0, 0x3),
            Instruction::DoubleModulo { dest, a, b } => Self::construct_double_instruction(dest, a, Some(b), 0x0, 0x4),
            Instruction::DoubleNegate { dest, src } => Self::construct_double_instruction(dest, src, None, 0x0, 0x5),
            Instruction::DoubleReciprocal { dest, src } => Self::construct_double_instruction(dest, src, None, 0x0, 0x6),
            Instruction::DoublePower { dest, a, b } => Self::construct_double_instruction(dest, a, Some(b), 0x0, 0x7),
            Instruction::DoubleExponential { dest, src } => Self::construct_double_instruction(dest, src, None, 0x0, 0x8),
            Instruction::DoubleRoot { dest, a, b } => Self::construct_double_instruction(dest, a, Some(b), 0x0, 0x9),
            Instruction::DoubleSquareRoot { dest, src } => Self::construct_double_instruction(dest, src, None, 0x0, 0xA),
            Instruction::DoubleCubeRoot { dest, src } => Self::construct_double_instruction(dest, src, None, 0x0, 0xB),
            Instruction::DoubleSquare { dest, src } => Self::construct_double_instruction(dest, src, None, 0x0, 0xC),
            Instruction::DoubleCube { dest, src } => Self::construct_double_instruction(dest, src, None, 0x0, 0xD),
            Instruction::DoubleLogarithm { dest, a, b } => Self::construct_double_instruction(dest, a, Some(b), 0x0, 0xE),
            Instruction::DoubleNaturalLogarithm { dest, src } => Self::construct_double_instruction(dest, src, None, 0x0, 0xF),
            Instruction::DoubleAbsolute { dest, src } => Self::construct_double_instruction(dest, src, None, 0x1, 0x0),
            Instruction::DoubleSine { dest, src } => Self::construct_double_instruction(dest, src, None, 0x1, 0x1),
            Instruction::DoubleCosine { dest, src } => Self::construct_double_instruction(dest, src, None, 0x1, 0x2),
            Instruction::DoubleTangent { dest, src } => Self::construct_double_instruction(dest, src, None, 0x1, 0x3),
            Instruction::DoubleArcsine { dest, src } => Self::construct_double_instruction(dest, src, None, 0x1, 0x4),
            Instruction::DoubleArccosine { dest, src } => Self::construct_double_instruction(dest, src, None, 0x1, 0x5),
            Instruction::DoubleArctangent { dest, src } => Self::construct_double_instruction(dest, src, None, 0x1, 0x6),
            Instruction::DoubleFloor { dest, src } => Self::construct_double_instruction(dest, src, None, 0x1, 0x7),
            Instruction::DoubleCeil { dest, src } => Self::construct_double_instruction(dest, src, None, 0x1, 0x8),
            Instruction::DoubleRound { dest, src } => Self::construct_double_instruction(dest, src, None, 0x1, 0x9),
            Instruction::DoubleMinimum { dest, a, b } => Self::construct_double_instruction(dest, a, Some(b), 0x1, 0xA),
            Instruction::DoubleMaximum { dest, a, b } => Self::construct_double_instruction(dest, a, Some(b), 0x1, 0xB),
            Instruction::DoubleSign { dest, src } => Self::construct_double_instruction(dest, src, None, 0x1, 0xC),
            Instruction::DoubleAbsoluteDifference { dest, a, b } => Self::construct_double_instruction(dest, a, Some(b), 0x1, 0xD),
            Instruction::DoubleLoadInfinity { dest } => Self::denibble([InstrType::DoubleArithmetic.into(), dest.into(), 0, 0, 0, 0, 0x1, 0xE]),
            Instruction::DoubleLoadNaN { dest } => Self::denibble([InstrType::DoubleArithmetic.into(), dest.into(), 0, 0, 0, 0, 0x1, 0xF]),
        }
    }

    fn construct_floating_instruction(dest: Register, a: Register, b: Option<Register>, byte7: u32, byte8: u32) -> u32 {
        Self::denibble([
            InstrType::FloatingArithmetic.into(),
            dest.into(),
            a.into(),
            b.map_or(0, Register::into),
            0,
            0,
            byte7,
            byte8
        ])
    }

    fn construct_double_instruction(dest: Register, a: Register, b: Option<Register>, byte7: u32, byte8: u32) -> u32 {
        Self::denibble([
            InstrType::DoubleArithmetic.into(),
            dest.into(),
            a.into(),
            b.map_or(0, Register::into),
            0,
            0,
            byte7,
            byte8
        ])
    }
}
