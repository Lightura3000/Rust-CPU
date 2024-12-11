#[derive(Debug, Copy, Clone)]
pub enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,
}

#[derive(Debug, Copy, Clone)]
pub struct U2 { value: u8 }
impl U2 {
    pub fn new(value: u8) -> Option<U2> {
        if value < 4 {
            Some( U2 { value })
        } else {
            None
        }
    }

    pub fn get(&self) -> u8 {
        self.value
    }
}

#[derive(Debug, Copy, Clone)]
pub struct U3 { value: u8 }
impl U3 {
    pub fn new(value: u8) -> Option<U3> {
        if value < 8 {
            Some( U3 { value })
        } else {
            None
        }
    }

    pub fn get(&self) -> u8 {
        self.value
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Nop = 0x00,
    Add { dest: Register, a: Register, b: Register } = 0x01,
    AddImm { dest: Register, a: Register, b: u16 } = 0x02,
    Sub { dest: Register, a: Register, b: Register } = 0x03,
    SubRI { dest: Register, a: Register, b: u16 } = 0x04,
    SubIR { dest: Register, a: u16, b: Register } = 0x05,
    Mul { dest: Register, a: Register, b: Register } = 0x06,
    MulI { dest: Register, a: Register, b: u16 } = 0x07,
    Div { dest: Register, a: Register, b: Register } = 0x08,
    DivRI { dest: Register, a: Register, b: u16 } = 0x09,
    DivIR { dest: Register, a: u16, b: Register } = 0x0A,
    SDiv { dest: Register, a: Register, b: Register } = 0x0B,
    SDivRI { dest: Register, a: Register, b: u16 } = 0x0C,
    SDivIR { dest: Register, a: u16, b: Register } = 0x0D,
    And { dest: Register, a: Register, b: Register } = 0x0E,
    Or { dest: Register, a: Register, b: Register } = 0x0F,
    Xor { dest: Register, a: Register, b: Register } = 0x10,
    Not { dest: Register, src: Register } = 0x11,
    Nand { dest: Register, a: Register, b: Register } = 0x12,
    Nor { dest: Register, a: Register, b: Register } = 0x13,
    Xnor { dest: Register, a: Register, b: Register } = 0x14,
    Mod { dest: Register, a: Register, b: Register } = 0x15,
    ModRI { dest: Register, a: Register, b: u16 } = 0x16,
    ModIR { dest: Register, a: u16, b: Register } = 0x17,
    Smod { dest: Register, a: Register, b: Register } = 0x18,
    SmodRI { dest: Register, a: Register, b: u16 } = 0x19,
    SmodIR { dest: Register, a: u16, b: Register } = 0x1A,
    RshR { reg: Register, amount: Register } = 0x1B,
    RshI { reg: Register, amount: u8 } = 0x1C,
    LshR { reg: Register, amount:  Register } = 0x1D,
    LshI { reg: Register, amount: u8 } = 0x1E,
    Ror { reg: Register } = 0x1F,
    Rol { reg: Register } = 0x20,
    Mov { dest: Register, src: Register } = 0x21,
    Ldi { dest: Register, slice: U2, imm: u16 } = 0x22,
    // Unused opcode 0x23
    LdrR { origin: Register, mem: Register, slice: U3 } = 0x24,
    LdrI { origin: Register, mem: u16, slice: U3 } = 0x25,
    StrR { origin: Register, mem: Register, slice: U3 } = 0x26,
    StrI { origin: Register, mem: u16, slice: U3 } = 0x27,
    Push { reg: Register } = 0x28,
    Pop { reg: Register } = 0x29,
    CmpRR { a: Register, b: Register } = 0x2A,
    CmpRI { a: Register, b: u16 } = 0x2B,
    CmpIR { a: u16, b: Register } = 0x2C,
    ScmpRR { a: Register, b: Register } = 0x2D,
    ScmpRI { a: Register, b: u16 } = 0x2E,
    ScmpIR { a: u16, b: Register } = 0x2F,
    BR { amount: Register } = 0x30,
    BI { amount: i16 } = 0x31,
    BgR { amount: Register } = 0x32,
    BgI { amount: i16 } = 0x33,
    BeR { amount: Register } = 0x34,
    BeI { amount: i16 } = 0x35,
    BsR { amount: Register } = 0x36,
    BsI { amount: u16 } = 0x37,
    BgeR { amount: Register } = 0x38,
    BgeI { amount: u16 } = 0x39,
    BneR { amount: Register } = 0x3A,
    BneI { amount: u16 } = 0x3B,
    BseR { amount: Register } = 0x3C,
    BseI { amount: u16 } = 0x3E,
}


impl Instruction {
    pub fn assemble(self) -> u32 {
        // println!("{:#010x}\n{:#010x}\n{:#010x}\n{:#010x}", 0x01000000, (dest as u32) << 20, (a as u32) << 16, (b as u32) << 12);
        match self {
            Instruction::Nop => 0,
            Instruction::Add { dest, a, b } => 0x01000000 | ((dest as u32) << 20) | ((a as u32) << 16) | ((b as u32) << 12),
            Instruction::AddImm { dest, a, b } => 0x02000000 | ((dest as u32) << 20) | ((a as u32) << 16) | (b as u32),
            Instruction::Sub { dest, a, b } => 0x03000000 | ((dest as u32) << 20) | ((a as u32) << 16) | ((b as u32) << 12),
            Instruction::SubRI { dest, a, b } => 0x04000000 | ((dest as u32) << 20) | ((a as u32) << 16) | (b as u32),
            Instruction::SubIR { dest, a, b } => 0x05000000 | ((dest as u32) << 20) | (a as u32) | ((b as u32) << 16),
            Instruction::Mul { dest, a, b } => 0x06000000 | ((dest as u32) << 20) | ((a as u32) << 16) | ((b as u32) << 12),
            Instruction::MulI { dest, a, b } => 0x07000000 | ((dest as u32) << 20) | ((a as u32) << 16) | (b as u32),
            Instruction::Div { dest, a, b } => 0x08000000 | ((dest as u32) << 20) | ((a as u32) << 16) | ((b as u32) << 12),
            Instruction::DivRI { dest, a, b } => 0x09000000 | ((dest as u32) << 20) | ((a as u32) << 16) | (b as u32),
            Instruction::DivIR { dest, a, b } => 0x0A000000 | ((dest as u32) << 20) | (a as u32) | ((b as u32) << 16),
            Instruction::SDiv { dest, a, b } => 0x0B000000 | ((dest as u32) << 20) | ((a as u32) << 16) | ((b as u32) << 12),
            Instruction::SDivRI { dest, a, b } => 0x0C000000 | ((dest as u32) << 20) | ((a as u32) << 16) | (b as u32),
            Instruction::SDivIR { dest, a, b } => 0x0D000000 | ((dest as u32) << 20) | (a as u32) | ((b as u32) << 16),
            Instruction::And { dest, a, b } => 0x0E000000 | ((dest as u32) << 20) | ((a as u32) << 16) | ((b as u32) << 12),
            Instruction::Or { dest, a, b } => 0x0F000000 | ((dest as u32) << 20) | ((a as u32) << 16) | ((b as u32) << 12),
            Instruction::Xor { dest, a, b } => 0x10000000 | ((dest as u32) << 20) | ((a as u32) << 16) | ((b as u32) << 12),
            Instruction::Not { dest, src } => 0x11000000 | ((dest as u32) << 20) | ((src as u32) << 16),
            Instruction::Nand { dest, a, b } => 0x12000000 | ((dest as u32) << 20) | ((a as u32) << 16) | ((b as u32) << 12),
            Instruction::Nor { dest, a, b } => 0x13000000 | ((dest as u32) << 20) | ((a as u32) << 16) | ((b as u32) << 12),
            Instruction::Xnor { dest, a, b } => 0x14000000 | ((dest as u32) << 20) | ((a as u32) << 16) | ((b as u32) << 12),
            Instruction::Mod { dest, a, b } => 0x15000000 | ((dest as u32) << 20) | ((a as u32) << 16) | ((b as u32) << 12),
            Instruction::ModRI { dest, a, b } => 0x16000000 | ((dest as u32) << 20) | ((a as u32) << 16) | (b as u32),
            Instruction::ModIR { dest, a, b } => 0x17000000 | ((dest as u32) << 20) | (a as u32) | ((b as u32) << 16),
            Instruction::Smod { dest, a, b } => 0x18000000 | ((dest as u32) << 20) | ((a as u32) << 16) | ((b as u32) << 12),
            Instruction::SmodRI { dest, a, b } => 0x19000000 | ((dest as u32) << 20) | ((a as u32) << 16) | (b as u32),
            Instruction::SmodIR { dest, a, b } => 0x1A000000 | ((dest as u32) << 20) | (a as u32) | ((b as u32) << 16),
            Instruction::RshR { reg, amount } => 0x1B000000 | ((reg as u32) << 20) | ((amount as u32) << 16),
            Instruction::RshI { reg, amount } => 0x1C000000 | ((reg as u32) << 20) | (amount as u32),
            Instruction::LshR { reg, amount } => 0x1D000000 | ((reg as u32) << 20) | ((amount as u32) << 16),
            Instruction::LshI { reg, amount } => 0x1E000000 | ((reg as u32) << 20) | (amount as u32),
            Instruction::Ror { reg } => 0x1F000000 | ((reg as u32) << 20),
            Instruction::Rol { reg } => 0x20000000 | ((reg as u32) << 20),
            Instruction::Mov { dest, src } => 0x21000000 | ((dest as u32) << 20) | ((src as u32) << 16),
            Instruction::Ldi { dest, slice, imm } => 0x22000000 | ((dest as u32) << 20) | ((slice.get() as u32) << 16) | (imm as u32),
            Instruction::LdrR { origin, mem, slice } => 0x24000000 | ((origin as u32) << 20) | ((mem as u32) << 16) | ((slice.get() as u32) << 12),
            Instruction::LdrI { origin, mem, slice } => 0x25000000 | ((origin as u32) << 20) | (mem as u32) | ((slice.get() as u32) << 16),
            Instruction::StrR { origin, mem, slice } => 0x26000000 | ((origin as u32) << 20) | ((mem as u32) << 16) | ((slice.get() as u32) << 12),
            Instruction::StrI { origin, mem, slice } => 0x27000000 | ((origin as u32) << 20) | (mem as u32) | ((slice.get() as u32) << 16),
            Instruction::Push { reg } => 0x28000000 | ((reg as u32) << 20),
            Instruction::Pop { reg } => 0x29000000 | ((reg as u32) << 20),
            Instruction::CmpRR { a, b } => 0x2A000000 | ((a as u32) << 20) | ((b as u32) << 16),
            Instruction::CmpRI { a, b } => 0x2B000000 | ((a as u32) << 20) | (b as u32),
            Instruction::CmpIR { a, b } => 0x2C000000 | (a as u32) | ((b as u32) << 16),
            Instruction::ScmpRR { a, b } => 0x2D000000 | ((a as u32) << 20) | ((b as u32) << 16),
            Instruction::ScmpRI { a, b } => 0x2E000000 | ((a as u32) << 20) | (b as u32),
            Instruction::ScmpIR { a, b } => 0x2F000000 | (a as u32) | ((b as u32) << 16),
            Instruction::BR { amount } => 0x30000000 | ((amount as u32) << 20),
            Instruction::BI { amount } => 0x31000000 | (amount as u32),
            Instruction::BgR { amount } => 0x32000000 | ((amount as u32) << 20),
            Instruction::BgI { amount } => 0x33000000 | (amount as u32),
            Instruction::BeR { amount } => 0x34000000 | ((amount as u32) << 20),
            Instruction::BeI { amount } => 0x35000000 | (amount as u32),
            Instruction::BsR { amount } => 0x36000000 | ((amount as u32) << 20),
            Instruction::BsI { amount } => 0x37000000 | (amount as u32),
            Instruction::BgeR { amount } => 0x38000000 | ((amount as u32) << 20),
            Instruction::BgeI { amount } => 0x39000000 | (amount as u32),
            Instruction::BneR { amount } => 0x3A000000 | ((amount as u32) << 20),
            Instruction::BneI { amount } => 0x3B000000 | (amount as u32),
            Instruction::BseR { amount } => 0x3C000000 | ((amount as u32) << 20),
            Instruction::BseI { amount } => 0x3E000000 | (amount as u32),
        }
    }
}
