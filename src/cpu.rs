use std::cmp::Ordering;

#[derive(Debug)]
pub struct CPU {
    pub regs: [u64; 16],
    pub memory: [u8; 4096],
    privileged: bool,
    halted: bool,
    pub flags: Flags,
}

#[derive(Debug, Default)]
pub struct Flags {
    // Set when an arithmetic operation results in a carry out of the most significant bit
    // Example: 0xFF + 0x01 = 0x100 (carry = true)
    // Useful for:
    // - Multi-precision arithmetic (working with numbers bigger than 64 bits)
    // - Detecting unsigned overflow
    carry: bool,

    // Set when the result of an operation is exactly zero
    // Example: 5 - 5 = 0 (zero = true)
    // Useful for:
    // - Equality comparisons
    // - Loop termination conditions
    zero: bool,

    // Set when the result of an operation has its most significant bit set (is negative)
    // Example: -1 in two's complement (negative = true)
    // Useful for:
    // - Sign detection
    // - Signed number comparisons
    negative: bool,

    // Set when an arithmetic operation exceeds the valid range for signed numbers
    // Example: 0x7FFFFFFF + 1 (overflow = true because it went from positive to negative)
    // Useful for:
    // - Detecting signed arithmetic overflow
    // - Error checking in calculations
    overflow: bool,

    // Set when a comparison shows the first operand is greater than the second
    // Example: CMP 5, 3 (greater = true)
    // Useful for:
    // - Conditional branching
    // - Implementing sorting algorithms
    greater: bool,

    // Set when a comparison shows the operands are equal
    // Example: CMP 4, 4 (equal = true)
    // Useful for:
    // - Conditional branching
    // - Loop termination
    equal: bool,

    // Set when a comparison shows the first operand is less than the second
    // Example: CMP 3, 5 (less = true)
    // Useful for:
    // - Conditional branching
    // - Implementing sorting algorithms
    less: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            regs: [0; 16],
            memory: [0; 4096],
            privileged: true,
            halted: false,
            flags: Flags::default(),
        }
    }

    fn compare<T: Ord>(&mut self, a: T, b: T) {
        let cmp = a.cmp(&b);
        self.flags.greater = cmp == Ordering::Greater;
        self.flags.equal = cmp == Ordering::Equal;
        self.flags.less = cmp == Ordering::Less;
    }

    fn signed_compare<T: Ord>(&mut self, a: T, b: T) {
        let cmp = a.cmp(&b);
        self.flags.greater = cmp == Ordering::Greater;
        self.flags.equal = cmp == Ordering::Equal;
        self.flags.less = cmp == Ordering::Less;
    }

    pub fn run(&mut self, instructions: &Vec<u32>) {
        instructions.iter().for_each(|instruction| self.exec(*instruction));
    }

    pub fn exec(&mut self, instruction: u32) {
        const OPCODE_MASK: u32    = 0xFF000000; // 8 bits for opcode
        const REG_A_MASK: u32     = 0x00F00000; // 4 bits for first register
        const REG_B_MASK: u32     = 0x000F0000; // 4 bits for second register
        const REG_C_MASK: u32     = 0x0000F000; // 4 bits for third register
        const IMMEDIATE_MASK: u32 = 0x0000FFFF; // 16 bits for immediate

        let opcode = ((instruction & OPCODE_MASK) >> OPCODE_MASK.trailing_zeros()) as u8;
        let reg_a = ((instruction & REG_A_MASK) >> REG_A_MASK.trailing_zeros()) as u8;
        let reg_b = ((instruction & REG_B_MASK) >> REG_B_MASK.trailing_zeros()) as u8;
        let reg_c = ((instruction & REG_C_MASK) >> REG_C_MASK.trailing_zeros()) as u8;
        let imm16 = (instruction & IMMEDIATE_MASK) as u16;
        let imm64 = (instruction & IMMEDIATE_MASK) as u64;

        let reg_a_value = self.regs[reg_a as usize];
        let reg_b_value = self.regs[reg_b as usize];
        let reg_c_value = self.regs[reg_c as usize];

        // println!("Decoded values:");
        // println!("opcode: {}", opcode);
        // println!("reg_a: {}", reg_a);
        // println!("reg_b: {}", reg_b);
        // println!("reg_c: {}", reg_c);
        // println!("reg_a value: {}", reg_a_value);
        // println!("reg_b value: {}", reg_b_value);
        // println!("reg_c value: {}", reg_c_value);

        match opcode {
            0x00 /* nop              */ => {},
            0x01 /* rA = rB + rC     */ => self.regs[reg_a as usize] = reg_b_value + reg_c_value,
            0x02 /* rA = rB + imm    */ => self.regs[reg_a as usize] = reg_b_value + imm64,
            0x03 /* rA = rB - rC     */ => self.regs[reg_a as usize] = reg_b_value - reg_c_value,
            0x04 /* rA = rB - imm    */ => self.regs[reg_a as usize] = reg_b_value - imm64,
            0x05 /* rA = imm - rB    */ => self.regs[reg_a as usize] = imm64 - reg_b_value,
            0x06 /* rA = rB * rC     */ => self.regs[reg_a as usize] = reg_b_value - reg_c_value,
            0x07 /* rA = rB * imm    */ => self.regs[reg_a as usize] = reg_b_value * reg_c_value,
            0x08 /* rA = rB / rC     */ => self.regs[reg_a as usize] = reg_b_value / reg_c_value,
            0x09 /* rA = rB / imm    */ => self.regs[reg_a as usize] = reg_b_value / imm64,
            0x0A /* rA = imm / rB    */ => self.regs[reg_a as usize] = imm64 / reg_b_value,
            0x0B /* rA = rB / rC     */ => self.regs[reg_a as usize] = (reg_b_value as i64 / reg_c_value as i64) as u64,
            0x0C /* rA = imm / rB    */ => self.regs[reg_a as usize] = (imm64 as i64 / reg_b_value as i64) as u64,
            0x0D /* rA = rB / imm    */ => self.regs[reg_a as usize] = (reg_b_value as i64 / imm64 as i64) as u64,
            0x0E /* rA = rB & rC     */ => self.regs[reg_a as usize] = reg_b_value & reg_c_value,
            0x0F /* rA = rB | rC     */ => self.regs[reg_a as usize] = reg_b_value | reg_c_value,
            0x10 /* rA = rB ^ rC     */ => self.regs[reg_a as usize] = reg_b_value ^ reg_c_value,
            0x11 /* rA = !rB         */ => self.regs[reg_a as usize] = !reg_b_value,
            0x12 /* rA = !(rB & rC)  */ => self.regs[reg_a as usize] = !(reg_b_value & reg_c_value),
            0x13 /* rA = !(rB | rC)  */ => self.regs[reg_a as usize] = !(reg_b_value | reg_c_value),
            0x14 /* rA = !(rB ^ rC)  */ => self.regs[reg_a as usize] = !(reg_b_value ^ reg_c_value),
            0x15 /* rA = rB % rC     */ => self.regs[reg_a as usize] = reg_b_value % reg_c_value,
            0x16 /* rA = rB % imm    */ => self.regs[reg_a as usize] = reg_b_value % imm64,
            0x17 /* rA = imm % rB    */ => self.regs[reg_a as usize] = imm64 % reg_b_value,
            0x18 /* rA = rB % rC     */ => self.regs[reg_a as usize] = (reg_b_value as i64 % reg_c_value as i64) as u64,
            0x19 /* rA = rB % imm    */ => self.regs[reg_a as usize] = (reg_b_value as i64 % imm64 as i64) as u64,
            0x1A /* rA = imm % rB    */ => self.regs[reg_a as usize] = (imm64 as i64 % reg_b_value as i64) as u64,
            0x1B /* rA = rB >> rC    */ => self.regs[reg_a as usize] = reg_b_value >> reg_c_value,
            0x1C /* rA = rB >> imm   */ => self.regs[reg_a as usize] = reg_b_value >> imm64,
            0x1D /* rA = rB << rC    */ => self.regs[reg_a as usize] = reg_b_value << reg_c_value,
            0x1E /* rA = rB << imm   */ => self.regs[reg_a as usize] = reg_b_value << imm64,
            0x1F /* ror              */ => self.regs[reg_a as usize] = reg_b_value.rotate_right(1),
            0x20 /* rol              */ => self.regs[reg_a as usize] = reg_b_value.rotate_left(1),
            0x21 /* rA = rB          */ => self.regs[reg_a as usize] = reg_b_value,
            0x22 /* ldi              */ => self.regs[reg_a as usize] = Self::ldi(reg_a_value, (instruction & (0b11 << 16)) >> 16, imm16),
            0x23 /* UNUSED           */ => unimplemented!("This opcode is unused"),
            0x24 /* rA = memory[rB]  */ => self.regs[reg_a as usize] = Self::set_byte(reg_a_value, (instruction & 0b111) as u8, self.memory[reg_b_value as usize]),
            0x25 /* rA = memory[imm] */ => self.regs[reg_a as usize] = Self::set_byte(reg_a_value, (instruction & 0b111) as u8, self.memory[imm16 as usize]),
            0x26 /* memory[rB] = rA  */ => self.memory[reg_b as usize] = Self::get_byte(reg_a_value, (instruction & 0b111) as u8),
            0x27 /* memory[imm] = rA */ => self.memory[imm16 as usize] = Self::get_byte(reg_a_value, (instruction & 0b111) as u8),
            0x28 /* push             */ => unimplemented!("Push not implemented"),
            0x29 /* pop              */ => unimplemented!("Pop not implemented"),
            0x2A /* rA.cmp(&rB)      */ => self.compare(reg_a_value, reg_b_value),
            0x2B /* rA.cmp(&imm)     */ => self.compare(reg_a_value, imm64),
            0x2C /* imm.cmp(&rA)     */ => self.compare(imm64, reg_a_value),
            0x2D /* rA.cmp(&rB)      */ => self.signed_compare(reg_a_value as i64, reg_b_value as i64),
            0x2E /* rA.cmp(&imm)     */ => self.signed_compare(reg_a_value as i64, imm64 as i64),
            0x2F /* imm.cmp(&rA)     */ => self.signed_compare(imm64 as i64, reg_a_value as i64),
            0x30 /* b                */ => unimplemented!("Branch by register not implemented"),
            0x31 /* b                */ => unimplemented!("Branch by immediate not implemented"),
            0x32 /* bg               */ => unimplemented!("Branch if greater by register not implemented"),
            0x33 /* bg               */ => unimplemented!("Branch if greater by immediate not implemented"),
            0x34 /* be               */ => unimplemented!("Branch if equal by register not implemented"),
            0x35 /* be               */ => unimplemented!("Branch if equal by immediate not implemented"),
            0x36 /* bs               */ => unimplemented!("Branch if smaller by register not implemented"),
            0x37 /* bs               */ => unimplemented!("Branch if smaller by immediate not implemented"),
            0x38 /* bng              */ => unimplemented!("Branch if not greater by register not implemented"),
            0x39 /* bng              */ => unimplemented!("Branch if not greater by immediate not implemented"),
            0x3A /* bne              */ => unimplemented!("Branch if not equal by register not implemented"),
            0x3B /* bne              */ => unimplemented!("Branch if not equal by immediate not implemented"),
            0x3C /* bns              */ => unimplemented!("Branch if not smaller by register not implemented"),
            0x3D /* bns              */ => unimplemented!("Branch if not smaller by immediate not implemented"),
            _ =>  println!("Unknown opcode: {:#x}", opcode),
        }
    }

    fn ldi(reg: u64, slice: u32, imm: u16) -> u64 {
        match slice {
            0 => (reg & !(0xFFFF << 0)) | ((imm as u64) << 0), // Clear bits 0-15 and set imm
            1 => (reg & !(0xFFFF << 16)) | ((imm as u64) << 16), // Clear bits 16-31 and set imm
            2 => (reg & !(0xFFFF << 32)) | ((imm as u64) << 32), // Clear bits 32-47 and set imm
            3 => (reg & !(0xFFFF << 48)) | ((imm as u64) << 48), // Clear bits 48-63 and set imm
            _ => unreachable!("If this is reached, something is wrong"),
        }
    }

    fn set_byte(v: u64, byte: u8, new: u8) -> u64 {
        if !(0..8).contains(&byte) {
            panic!("Byte index out of range (must be 0 to 7)");
        }

        let shift = byte * 8;
        let cleared_v = v & !(0xFFu64 << shift); // Clear target byte
        cleared_v | ((new as u64) << shift)
    }

    fn get_byte(v: u64, byte: u8) -> u8 {
        if !(0..8).contains(&byte) {
            panic!("Byte index out of range (must be 0 to 7)");
        }

        let shift = byte * 8;
        ((v >> shift) & 0xFF) as u8
    }
}
