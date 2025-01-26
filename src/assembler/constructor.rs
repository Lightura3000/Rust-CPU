use either::{Either, Left, Right};
use super::register::Register;
use super::unsigned_newtypes::{U2, U3, U6};
use super::nibbles::{pack_nibbles, split_u16_into_nibbles, split_u6_into_nibbles};

// ---------------------------------------------------------------------------------------------

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

impl From<InstrType> for u32 {
    fn from(t: InstrType) -> Self {
        t as u32
    }
}

impl From<Register> for u32 {
    fn from(r: Register) -> Self {
        r as u32
    }
}

// ---------------------------------------------------------------------------------------------

/// Helper for arithmetic instructions that can be register-register or register-immediate.
fn assemble_arithmetic(
    instr_type: InstrType,
    dest: Register,
    a: Either<Register, u16>,
    b: Either<Register, u16>,
    subcode_rr: u32, // subcode if (reg, reg)
    subcode_ri: u32, // subcode if (reg, imm)
    subcode_ir: u32, // subcode if (imm, reg)
) -> u32 {
    match (a, b) {
        (Left(r1), Left(r2)) => pack_nibbles([
            instr_type.into(),  // nibble 0
            dest.into(),        // nibble 1
            r1.into(),          // nibble 2
            r2.into(),          // nibble 3
            0, 0, 0,
            subcode_rr
        ]),
        (Left(r), Right(imm)) => {
            let (n0, n1, n2, n3) = split_u16_into_nibbles(imm);
            pack_nibbles([
                instr_type.into(),
                dest.into(),
                r.into(),
                n0, n1, n2, n3,
                subcode_ri
            ])
        }
        (Right(imm), Left(r)) => {
            let (n0, n1, n2, n3) = split_u16_into_nibbles(imm);
            pack_nibbles([
                instr_type.into(),
                dest.into(),
                r.into(),
                n0, n1, n2, n3,
                subcode_ir
            ])
        }
        // For these instructions, (imm, imm) is disallowed
        (Right(_), Right(_)) => panic!("Invalid combination: immediate, immediate."),
    }
}

/// Helper for shift/rotate instructions that can be register or immediate (U6).
fn assemble_shift_or_rotate(
    instr_type: InstrType,
    dest: Register,
    src: Register,
    amount: Either<Register, U6>,
    subcode_reg: u32,
    subcode_imm: u32
) -> u32 {
    match amount {
        Left(reg) => pack_nibbles([
            instr_type.into(),
            dest.into(),
            src.into(),
            reg.into(),
            0, 0, 0,
            subcode_reg
        ]),
        Right(u6) => {
            let (n0, n1) = split_u6_into_nibbles(u6);
            pack_nibbles([
                instr_type.into(),
                dest.into(),
                src.into(),
                0, 0, n0, n1,
                subcode_imm
            ])
        }
    }
}

// Similar small helpers for branching, bitwise, floating, etc.:
fn assemble_branch(
    instr_type: InstrType,
    offset: Either<Register, i16>,
    subcode_reg: u32,
    subcode_imm: u32
) -> u32 {
    match offset {
        Left(reg) => pack_nibbles([
            instr_type.into(),
            reg.into(),
            0, 0, 0, 0, 0,
            subcode_reg
        ]),
        Right(imm) => {
            let (n0, n1, n2, n3) = split_u16_into_nibbles(imm as u16);
            pack_nibbles([
                instr_type.into(),
                n0, n1, n2, n3,
                0, 0,
                subcode_imm
            ])
        }
    }
}

fn assemble_floating(    instr_type: InstrType,  // either FloatingArithmetic or DoubleArithmetic
    dest: Register,
    a: Register,
    b: Option<Register>,
    subcode_major: u32,
    subcode_minor: u32
) -> u32 {
    pack_nibbles([
        instr_type.into(),
        dest.into(),
        a.into(),
        b.map_or(0, |r| r.into()),
        0, 0,
        subcode_major,
        subcode_minor
    ])
}


fn construct_floating_instruction(dest: Register, a: Register, b: Option<Register>, byte7: u32, byte8: u32) -> u32 {
    pack_nibbles([
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
    pack_nibbles([
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


#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
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
    pub fn assemble(self) -> u32 {
        use Instruction::*;

        match self {
            Nop => 0,

            // ----------------- Arithmetic -----------------
            Add { dest, a, b } => assemble_arithmetic(InstrType::Arithmetic, dest, a, b, 0x0, 0x1,0x2),
            Subtract { dest, a, b } => assemble_arithmetic(InstrType::Arithmetic, dest, a, b, 0x3, 0x4, 0x5),
            Multiply { dest, a, b } => assemble_arithmetic(InstrType::Arithmetic, dest, a, b, 0x6, 0x7, 0x8),
            Divide { dest, a, b } => assemble_arithmetic(InstrType::Arithmetic, dest, a, b, 0x9, 0xA, 0xB),
            DivideSigned { dest, a, b } => assemble_arithmetic(InstrType::Arithmetic, dest, a, b, 0xC, 0xD, 0xE),

            // ----------------- Bitwise -----------------
            And { dest, a, b } => pack_nibbles([InstrType::Bitwise.into(), dest.into(), a.into(), b.into(), 0, 0, 0, 0x0]),
            Or { dest, a, b } => pack_nibbles([InstrType::Bitwise.into(), dest.into(), a.into(), b.into(), 0, 0, 0, 0x1]),
            Xor { dest, a, b } => pack_nibbles([InstrType::Bitwise.into(), dest.into(), a.into(), b.into(), 0, 0, 0, 0x2]),
            Nand { dest, a, b } => pack_nibbles([InstrType::Bitwise.into(), dest.into(), a.into(), b.into(), 0, 0, 0, 0x3]),
            Nor { dest, a, b } => pack_nibbles([InstrType::Bitwise.into(), dest.into(), a.into(), b.into(), 0, 0, 0, 0x4]),
            Xnor { dest, a, b } => pack_nibbles([InstrType::Bitwise.into(), dest.into(), a.into(), b.into(), 0, 0, 0, 0x5]),
            Not { dest, src } => pack_nibbles([InstrType::Bitwise.into(), dest.into(), src.into(), 0, 0, 0, 0, 0x6]),
            
            // ----------------- Shifts & Rotates -----------------
            RightShift { dest, src, amount } => assemble_shift_or_rotate(InstrType::ShiftRotate, dest, src, amount, 0x0, 0x1,),
            LeftShift { dest, src, amount } => assemble_shift_or_rotate(InstrType::ShiftRotate, dest, src, amount, 0x2, 0x3),
            RightRoll { dest, src, amount } => assemble_shift_or_rotate(InstrType::ShiftRotate, dest, src, amount, 0x4, 0x5),
            LeftRoll { dest, src, amount } => assemble_shift_or_rotate(InstrType::ShiftRotate, dest, src, amount, 0x6, 0x7),

            // ----------------- Data Movement / Stack -----------------
            Move { dest, src } => pack_nibbles([InstrType::DataMemoryStack.into(), dest.into(), src.into(), 0, 0, 0, 0, 0x0]),
            LoadImmediate { dest, slice, imm } => {
                let (n0, n1, n2, n3) = split_u16_into_nibbles(imm);
                pack_nibbles([InstrType::DataMemoryStack.into(), dest.into(), n0, n1, n2, n3, slice.get() as u32, 0x1])
            }
            LoadRegister { dest, mem_ptr, slice } => match mem_ptr {
                Left(reg) => pack_nibbles([InstrType::DataMemoryStack.into(), dest.into(), reg.into(), 0, 0, 0, slice.get() as u32, 0x2]),
                Right(imm) => {
                    let (n0, n1, n2, n3) = split_u16_into_nibbles(imm);
                    pack_nibbles([InstrType::DataMemoryStack.into(), dest.into(), n0, n1, n2, n3, slice.get() as u32, 0x3])
                }
            },
            StoreRegister { src, mem_ptr, slice } => match mem_ptr {
                Left(reg) => pack_nibbles([InstrType::DataMemoryStack.into(), src.into(), reg.into(), 0, 0, 0, slice.get() as u32, 0x4]),
                Right(imm) => {
                    let (n0, n1, n2, n3) = split_u16_into_nibbles(imm);
                    pack_nibbles([InstrType::DataMemoryStack.into(), src.into(), n0, n1, n2, n3, slice.get() as u32, 0x5])
                }
            },
            Push { reg } => pack_nibbles([InstrType::DataMemoryStack.into(), reg.into(), 0, 0, 0, 0, 0, 0x6]),
            Pop { reg } => pack_nibbles([InstrType::DataMemoryStack.into(), reg.into(), 0, 0, 0, 0, 0, 0x7]),

            // ----------------- Comparison -----------------
            Compare { a, b, signed } => {
                use either::Either::{Left, Right};
                match (signed, a, b) {
                    (false, Left(r1), Left(r2)) => pack_nibbles([InstrType::Comparison.into(), r1.into(), r2.into(), 0, 0, 0, 0, 0x0]),
                    (false, Left(r), Right(imm)) => {
                        let (n0, n1, n2, n3) = split_u16_into_nibbles(imm);
                        pack_nibbles([InstrType::Comparison.into(), r.into(), n0, n1, n2, n3, 0, 0x1])
                    }
                    (false, Right(imm), Left(r)) => {
                        let (n0, n1, n2, n3) = split_u16_into_nibbles(imm);
                        pack_nibbles([InstrType::Comparison.into(), r.into(), n0, n1, n2, n3, 0, 0x2])
                    }
                    (true, Left(r1), Left(r2)) => pack_nibbles([InstrType::Comparison.into(), r1.into(), r2.into(), 0, 0, 0, 0, 0x3]),
                    (true, Left(r), Right(imm)) => {
                        let (n0, n1, n2, n3) = split_u16_into_nibbles(imm);
                        pack_nibbles([InstrType::Comparison.into(), r.into(), n0, n1, n2, n3, 0, 0x4])
                    }
                    (true, Right(imm), Left(r)) => {
                        let (n0, n1, n2, n3) = split_u16_into_nibbles(imm);
                        pack_nibbles([InstrType::Comparison.into(), r.into(), n0, n1, n2, n3, 0, 0x5])
                    }
                    (_, Right(_), Right(_)) => panic!("Invalid combination"),
                }
            }
            CompareFloat { a, b } => pack_nibbles([InstrType::Comparison.into(), a.into(), b.into(), 0, 0, 0, 0, 0x6]),
            CompareDouble { a, b } => pack_nibbles([InstrType::Comparison.into(), a.into(), b.into(), 0, 0, 0, 0, 0x7]),

            // ----------------- Branching -----------------
            Branch { offset } => assemble_branch(InstrType::Branching, offset, 0x0, 0x1),
            BranchGreater { offset } => assemble_branch(InstrType::Branching, offset, 0x2, 0x3),
            BranchEqual { offset } => assemble_branch(InstrType::Branching, offset, 0x4, 0x5),
            BranchSmaller { offset } => assemble_branch(InstrType::Branching, offset, 0x6, 0x7),
            BranchGreaterEqual { offset } => assemble_branch(InstrType::Branching, offset, 0x8, 0x9),
            BranchNotEqual { offset } => assemble_branch(InstrType::Branching, offset, 0xA, 0xB),
            BranchSmallerEqual { offset } => assemble_branch(InstrType::Branching, offset, 0xC, 0xD),

            // ----------------- Conversions -----------------
            ImmediateToFloat { dest, imm } => {
                let (n0, n1, n2, n3) = split_u16_into_nibbles(imm as u16);
                pack_nibbles([
                    InstrType::Conversion.into(),
                    dest.into(),
                    n0, n1, n2, n3,
                    0, 0x0
                ])
            }
            ImmediateToDouble { dest, imm } => {
                let (n0, n1, n2, n3) = split_u16_into_nibbles(imm as u16);
                pack_nibbles([
                    InstrType::Conversion.into(),
                    dest.into(),
                    n0, n1, n2, n3,
                    0, 0x1
                ])
            }
            IntegerToFloat { dest, src } => pack_nibbles([InstrType::Conversion.into(), dest.into(), src.into(), 0, 0, 0, 0, 0x2]),
            IntegerToDouble { dest, src } => pack_nibbles([InstrType::Conversion.into(), dest.into(), src.into(), 0, 0, 0, 0, 0x3]),
            FloatToInteger { dest, src } => pack_nibbles([InstrType::Conversion.into(), dest.into(), src.into(), 0, 0, 0, 0, 0x4]),
            FloatToDouble { dest, src } => pack_nibbles([InstrType::Conversion.into(), dest.into(), src.into(), 0, 0, 0, 0, 0x5]),
            DoubleToInteger { dest, src } => pack_nibbles([InstrType::Conversion.into(), dest.into(), src.into(), 0, 0, 0, 0, 0x6]),
            DoubleToFloat { dest, src } => pack_nibbles([InstrType::Conversion.into(), dest.into(), src.into(), 0, 0, 0, 0, 0x7]),

            // ----------------- Floating arithmetic -----------------
            FloatAdd { dest, a, b } => assemble_floating(InstrType::FloatingArithmetic, dest, a, Some(b), 0x0, 0x0),
            FloatSubtract { dest, a, b } => assemble_floating(InstrType::FloatingArithmetic, dest, a, Some(b), 0x0, 0x1),
            FloatMultiply { dest, a, b } => assemble_floating(InstrType::FloatingArithmetic, dest, a, Some(b), 0x0, 0x2),
            FloatDivide { dest, a, b } => assemble_floating(InstrType::FloatingArithmetic, dest, a, Some(b), 0x0, 0x3),
            FloatModulo { dest, a, b } => assemble_floating(InstrType::FloatingArithmetic, dest, a, Some(b), 0x0, 0x4),
            FloatNegate { dest, src } => assemble_floating(InstrType::FloatingArithmetic, dest, src, None, 0x0, 0x5),
            FloatReciprocal { dest, src } => assemble_floating(InstrType::FloatingArithmetic, dest, src, None, 0x0, 0x6),
            FloatPower { dest, a, b } => assemble_floating(InstrType::FloatingArithmetic, dest, a, Some(b), 0x0, 0x7),
            FloatExponential { dest, src } => assemble_floating(InstrType::FloatingArithmetic, dest, src, None, 0x0, 0x8),
            FloatRoot { dest, a, b } => assemble_floating(InstrType::FloatingArithmetic, dest, a, Some(b), 0x0, 0x9),
            FloatSquareRoot { dest, src } => assemble_floating(InstrType::FloatingArithmetic, dest, src, None, 0x0, 0xA),
            FloatCubeRoot { dest, src } => assemble_floating(InstrType::FloatingArithmetic, dest, src, None, 0x0, 0xB),
            FloatSquare { dest, src } => assemble_floating(InstrType::FloatingArithmetic, dest, src, None, 0x0, 0xC),
            FloatCube { dest, src } => assemble_floating(InstrType::FloatingArithmetic, dest, src, None, 0x0, 0xD),
            FloatLogarithm { dest, a, b } => assemble_floating(InstrType::FloatingArithmetic, dest, a, Some(b), 0x0, 0xE),
            FloatNaturalLogarithm { dest, src } => assemble_floating(InstrType::FloatingArithmetic, dest, src, None, 0x0, 0xF),
            FloatAbsolute { dest, src } => assemble_floating(InstrType::FloatingArithmetic, dest, src, None, 0x1, 0x0),
            FloatSine { dest, src } => assemble_floating(InstrType::FloatingArithmetic, dest, src, None, 0x1, 0x1),
            FloatCosine { dest, src } => assemble_floating(InstrType::FloatingArithmetic, dest, src, None, 0x1, 0x2),
            FloatTangent { dest, src } => assemble_floating(InstrType::FloatingArithmetic, dest, src, None, 0x1, 0x3),
            FloatArcsine { dest, src } => assemble_floating(InstrType::FloatingArithmetic, dest, src, None, 0x1, 0x4),
            FloatArccosine { dest, src } => assemble_floating(InstrType::FloatingArithmetic, dest, src, None, 0x1, 0x5),
            FloatArctangent { dest, src } => assemble_floating(InstrType::FloatingArithmetic, dest, src, None, 0x1, 0x6),
            FloatFloor { dest, src } => assemble_floating(InstrType::FloatingArithmetic, dest, src, None, 0x1, 0x7),
            FloatCeil { dest, src } => assemble_floating(InstrType::FloatingArithmetic, dest, src, None, 0x1, 0x8),
            FloatRound { dest, src } => assemble_floating(InstrType::FloatingArithmetic, dest, src, None, 0x1, 0x9),
            FloatMinimum { dest, a, b } => assemble_floating(InstrType::FloatingArithmetic, dest, a, Some(b), 0x1, 0xA),
            FloatMaximum { dest, a, b } => assemble_floating(InstrType::FloatingArithmetic, dest, a, Some(b), 0x1, 0xB),
            FloatSign { dest, src } => assemble_floating(InstrType::FloatingArithmetic, dest, src, None, 0x1, 0xC),
            FloatAbsoluteDifference { dest, a, b } => assemble_floating(InstrType::FloatingArithmetic, dest, a, Some(b), 0x1, 0xD),
            FloatLoadInfinity { dest } => pack_nibbles([InstrType::FloatingArithmetic.into(), dest.into(), 0, 0, 0, 0, 0x1, 0xE]),
            FloatLoadNaN { dest } => pack_nibbles([InstrType::FloatingArithmetic.into(), dest.into(), 0, 0, 0, 0, 0x1, 0xF]),

            // ----------------- Double arithmetic -----------------
            DoubleAdd { dest, a, b } => assemble_floating(InstrType::DoubleArithmetic, dest, a, Some(b), 0x0, 0x0),
            DoubleSubtract { dest, a, b } => assemble_floating(InstrType::DoubleArithmetic, dest, a, Some(b), 0x0, 0x1),
            DoubleMultiply { dest, a, b } => assemble_floating(InstrType::DoubleArithmetic, dest, a, Some(b), 0x0, 0x2),
            DoubleDivide { dest, a, b } => assemble_floating(InstrType::DoubleArithmetic, dest, a, Some(b), 0x0, 0x3),
            DoubleModulo { dest, a, b } => assemble_floating(InstrType::DoubleArithmetic, dest, a, Some(b), 0x0, 0x4),
            DoubleNegate { dest, src } => assemble_floating(InstrType::DoubleArithmetic, dest, src, None, 0x0, 0x5),
            DoubleReciprocal { dest, src } => assemble_floating(InstrType::DoubleArithmetic, dest, src, None, 0x0, 0x6),
            DoublePower { dest, a, b } => assemble_floating(InstrType::DoubleArithmetic, dest, a, Some(b), 0x0, 0x7),
            DoubleExponential { dest, src } => assemble_floating(InstrType::DoubleArithmetic, dest, src, None, 0x0, 0x8),
            DoubleRoot { dest, a, b } => assemble_floating(InstrType::DoubleArithmetic, dest, a, Some(b), 0x0, 0x9),
            DoubleSquareRoot { dest, src } => assemble_floating(InstrType::DoubleArithmetic, dest, src, None, 0x0, 0xA),
            DoubleCubeRoot { dest, src } => assemble_floating(InstrType::DoubleArithmetic, dest, src, None, 0x0, 0xB),
            DoubleSquare { dest, src } => assemble_floating(InstrType::DoubleArithmetic, dest, src, None, 0x0, 0xC),
            DoubleCube { dest, src } => assemble_floating(InstrType::DoubleArithmetic, dest, src, None, 0x0, 0xD),
            DoubleLogarithm { dest, a, b } => assemble_floating(InstrType::DoubleArithmetic, dest, a, Some(b), 0x0, 0xE),
            DoubleNaturalLogarithm { dest, src } => assemble_floating(InstrType::DoubleArithmetic, dest, src, None, 0x0, 0xF),
            DoubleAbsolute { dest, src } => assemble_floating(InstrType::DoubleArithmetic, dest, src, None, 0x1, 0x0),
            DoubleSine { dest, src } => assemble_floating(InstrType::DoubleArithmetic, dest, src, None, 0x1, 0x1),
            DoubleCosine { dest, src } => assemble_floating(InstrType::DoubleArithmetic, dest, src, None, 0x1, 0x2),
            DoubleTangent { dest, src } => assemble_floating(InstrType::DoubleArithmetic, dest, src, None, 0x1, 0x3),
            DoubleArcsine { dest, src } => assemble_floating(InstrType::DoubleArithmetic, dest, src, None, 0x1, 0x4),
            DoubleArccosine { dest, src } => assemble_floating(InstrType::DoubleArithmetic, dest, src, None, 0x1, 0x5),
            DoubleArctangent { dest, src } => assemble_floating(InstrType::DoubleArithmetic, dest, src, None, 0x1, 0x6),
            DoubleFloor { dest, src } => assemble_floating(InstrType::DoubleArithmetic, dest, src, None, 0x1, 0x7),
            DoubleCeil { dest, src } => assemble_floating(InstrType::DoubleArithmetic, dest, src, None, 0x1, 0x8),
            DoubleRound { dest, src } => assemble_floating(InstrType::DoubleArithmetic, dest, src, None, 0x1, 0x9),
            DoubleMinimum { dest, a, b } => assemble_floating(InstrType::DoubleArithmetic, dest, a, Some(b), 0x1, 0xA),
            DoubleMaximum { dest, a, b } => assemble_floating(InstrType::DoubleArithmetic, dest, a, Some(b), 0x1, 0xB),
            DoubleSign { dest, src } => assemble_floating(InstrType::DoubleArithmetic, dest, src, None, 0x1, 0xC),
            DoubleAbsoluteDifference { dest, a, b } => assemble_floating(InstrType::DoubleArithmetic, dest, a, Some(b), 0x1, 0xD),
            DoubleLoadInfinity { dest } => pack_nibbles([InstrType::DoubleArithmetic.into(), dest.into(), 0, 0, 0, 0, 0x1, 0xE]),
            DoubleLoadNaN { dest } => pack_nibbles([InstrType::DoubleArithmetic.into(), dest.into(), 0, 0, 0, 0, 0x1, 0xF]),
        }
    }
}
