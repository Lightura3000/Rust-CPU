use std::cmp::Ordering;

const INSTRUCTION_POINTER: usize = 15;

#[derive(Debug)]
pub struct CPU {
    pub regs: [u64; 16],
    pub memory: [u8; 4096],
    privileged: bool, /// Indicates if the CPU is running in privileged mode
    halted: bool, /// This field will probably get removed in the future
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
    smaller: bool,
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

    pub fn run(&mut self, cycles: u32) {
        for _ in 0..cycles {
            let address = self.regs[INSTRUCTION_POINTER] as usize;
            let instruction = self.fetch_instruction(address);
            self.exec(instruction);
        }
    }

    pub fn set_instruction_ptr(&mut self, value: u64) {
        self.regs[INSTRUCTION_POINTER] = value;
    }

    fn fetch_instruction(&mut self, address: usize) -> u32 {
        let byte1 = self.memory[address] as u32;
        let byte2 = self.memory[address + 1] as u32;
        let byte3 = self.memory[address + 2] as u32;
        let byte4 = self.memory[address + 3] as u32;
        (byte1 << 24) | (byte2 << 16) | (byte3 << 8) | byte4
    }

    fn compare<T: Ord>(&mut self, a: T, b: T) {
        let cmp = a.cmp(&b);
        self.flags.greater = cmp == Ordering::Greater;
        self.flags.equal = cmp == Ordering::Equal;
        self.flags.smaller = cmp == Ordering::Less;
    }

    /// A helper function to perform an unsigned arithmetic operation and set flags
    fn exec_arith_op(
        &mut self,
        reg_a: usize,
        left_hand_side: u64,
        right_hand_side: u64,
        op_unsigned: fn(u64, u64) -> (u64, bool),
        op_signed: fn(i64, i64) -> (i64, bool),
    ) {
        let (result, carry) = op_unsigned(left_hand_side, right_hand_side);
        // For signed overflow detection, we perform the operation in signed space
        let (signed_result, signed_overflow) = op_signed(left_hand_side as i64, right_hand_side as i64);

        self.regs[reg_a] = result;
        self.flags.carry = carry;
        self.flags.zero = result == 0;
        self.flags.negative = signed_result < 0;
        self.flags.overflow = signed_overflow;
    }

    /// Helper for addition: rA = lhs + rhs
    fn exec_add(&mut self, reg_a: usize, lhs: u64, rhs: u64) {
        self.exec_arith_op(reg_a, lhs, rhs, u64::overflowing_add, i64::overflowing_add);
    }

    /// Helper for subtraction: rA = lhs - rhs
    fn exec_sub(&mut self, reg_a: usize, lhs: u64, rhs: u64) {
        self.exec_arith_op(reg_a, lhs, rhs, u64::overflowing_sub, i64::overflowing_sub);
    }

    /// Helper for multiplication: rA = lhs * rhs
    fn exec_mul(&mut self, reg_a: usize, lhs: u64, rhs: u64) {
        self.exec_arith_op(reg_a, lhs, rhs, u64::overflowing_mul, i64::overflowing_mul);
    }

    /// Helper for unsigned division: rA = lhs / rhs
    fn exec_div_u(&mut self, reg_a: usize, lhs: u64, rhs: u64) {
        if rhs == 0 {
            // Division by zero
            self.regs[reg_a] = 0;
            self.flags.carry = false;
            self.flags.zero = true;
            self.flags.negative = false;
            self.flags.overflow = true;
        } else {
            let result = lhs / rhs;
            self.regs[reg_a] = result;
            self.flags.carry = false;
            self.flags.zero = result == 0;
            self.flags.negative = (result as i64) < 0;
            self.flags.overflow = false; // Unsigned division doesn't typically overflow
        }
    }

    /// Helper for signed division: rA = lhs / rhs (signed)
    fn exec_div_s(&mut self, reg_a: usize, lhs: u64, rhs: u64) {
        let lhs = lhs as i64;
        let rhs = rhs as i64;

        if rhs == 0 {
            // Division by zero
            self.flags.overflow = true;
            self.regs[reg_a] = 0;
            self.flags.zero = true;
            self.flags.negative = false;
            self.flags.carry = false;
        } else {
            // Check the potential overflow case: i64::MIN / -1
            // This would cause an overflow in signed division since the result can't be represented.
            if lhs == i64::MIN && rhs == -1 {
                // In many architectures this causes an arithmetic exception.
                // Here, we treat it as overflow.
                self.regs[reg_a] = lhs.wrapping_div(rhs) as u64;
                self.flags.overflow = true;
            } else {
                self.regs[reg_a] = (lhs / rhs) as u64;
                self.flags.overflow = false;
            }

            let result = self.regs[reg_a] as i64;
            self.flags.carry = false;
            self.flags.zero = result == 0;
            self.flags.negative = result < 0;
        }
    }

    /// Helper for unsigned modulo: rA = lhs % rhs
    fn exec_mod_u(&mut self, reg_a: usize, lhs: u64, rhs: u64) {
        if rhs == 0 {
            // Modulo by zero
            self.regs[reg_a] = 0;
            self.flags.carry = false;
            self.flags.zero = true;
            self.flags.negative = false;
            self.flags.overflow = true;
        } else {
            let result = lhs % rhs;
            self.regs[reg_a] = result;
            self.flags.carry = false;
            self.flags.zero = result == 0;
            self.flags.negative = (result as i64) < 0;
            self.flags.overflow = false;
        }
    }

    /// Helper for signed modulo: rA = lhs % rhs (signed)
    fn exec_mod_s(&mut self, reg_a: usize, lhs: u64, rhs: u64) {
        let lhs_i = lhs as i64;
        let rhs_i = rhs as i64;

        if rhs_i == 0 {
            // Modulo by zero
            self.regs[reg_a] = 0;
            self.flags.carry = false;
            self.flags.zero = true;
            self.flags.negative = false;
            self.flags.overflow = true;
        } else {
            // Unlike division, (i64::MIN % -1) = 0, which doesn't overflow.
            let result_i = lhs_i.wrapping_rem(rhs_i);
            self.regs[reg_a] = result_i as u64;
            self.flags.carry = false;
            self.flags.zero = result_i == 0;
            self.flags.negative = result_i < 0;
            self.flags.overflow = false;
        }
    }

    pub fn exec(&mut self, instruction: u32) {
        const OPCODE_MASK: u32    = 0xFF000000; // 8 bits for opcode
        const REG_A_MASK: u32     = 0x00F00000; // 4 bits for first register
        const REG_B_MASK: u32     = 0x000F0000; // 4 bits for second register
        const REG_C_MASK: u32     = 0x0000F000; // 4 bits for third register
        const IMMEDIATE_MASK: u32 = 0x0000FFFF; // 16 bits for immediate

        let opcode = ((instruction & OPCODE_MASK) >> OPCODE_MASK.trailing_zeros()) as u8;
        let reg_a = ((instruction & REG_A_MASK) >> REG_A_MASK.trailing_zeros()) as usize;
        let reg_b = ((instruction & REG_B_MASK) >> REG_B_MASK.trailing_zeros()) as u8;
        let reg_c = ((instruction & REG_C_MASK) >> REG_C_MASK.trailing_zeros()) as u8;
        let imm16 = (instruction & IMMEDIATE_MASK) as u16;
        let imm64 = (instruction & IMMEDIATE_MASK) as u64;
        let byte_pos = (instruction & 0b111) as u8;

        let reg_a_value = self.regs[reg_a];
        let reg_b_value = self.regs[reg_b as usize];
        let reg_c_value = self.regs[reg_c as usize];

        let prev_instr_ptr = self.regs[INSTRUCTION_POINTER];

        match opcode {
            0x00 /* nop                        */ => {},
            0x01 /* rA = rB + rC               */ => self.exec_add(reg_a, reg_b_value, reg_c_value),
            0x02 /* rA = rB + imm              */ => self.exec_add(reg_a, reg_b_value, imm64),
            0x03 /* rA = rB - rC               */ => self.exec_sub(reg_a, reg_b_value, reg_c_value),
            0x04 /* rA = rB - imm              */ => self.exec_sub(reg_a, reg_b_value, imm64),
            0x05 /* rA = imm - rB              */ => self.exec_sub(reg_a, imm64, reg_b_value),
            0x06 /* rA = rB * rC               */ => self.exec_mul(reg_a, reg_b_value, reg_c_value),
            0x07 /* rA = rB * imm              */ => self.exec_mul(reg_a, reg_b_value, imm64),
            0x08 /* rA = rB / rC    (unsigned) */ => self.exec_div_u(reg_a, reg_b_value, reg_c_value),
            0x09 /* rA = rB / imm   (unsigned) */ => self.exec_div_u(reg_a, reg_b_value, imm64),
            0x0A /* rA = imm / rB   (unsigned) */ => self.exec_div_u(reg_a, imm64, reg_b_value),
            0x0B /* rA = rB / rC    (signed)   */ => self.exec_div_s(reg_a, reg_b_value, reg_c_value),
            0x0C /* rA = rB / imm   (signed)   */ => self.exec_div_s(reg_a, reg_b_value, imm64),
            0x0D /* rA = imm / rB   (signed)   */ => self.exec_div_s(reg_a, imm64, reg_b_value),
            0x0E /* rA = rB & rC               */ => self.regs[reg_a] = reg_b_value & reg_c_value,
            0x0F /* rA = rB | rC               */ => self.regs[reg_a] = reg_b_value | reg_c_value,
            0x10 /* rA = rB ^ rC               */ => self.regs[reg_a] = reg_b_value ^ reg_c_value,
            0x11 /* rA = !rB                   */ => self.regs[reg_a] = !reg_b_value,
            0x12 /* rA = !(rB & rC)            */ => self.regs[reg_a] = !(reg_b_value & reg_c_value),
            0x13 /* rA = !(rB | rC)            */ => self.regs[reg_a] = !(reg_b_value | reg_c_value),
            0x14 /* rA = !(rB ^ rC)            */ => self.regs[reg_a] = !(reg_b_value ^ reg_c_value),
            0x15 /* rA = rB % rC    (unsigned) */ => self.exec_mod_u(reg_a, reg_b_value, reg_c_value),
            0x16 /* rA = rB % imm   (unsigned) */ => self.exec_mod_u(reg_a, reg_b_value, imm64),
            0x17 /* rA = imm % rB   (unsigned) */ => self.exec_mod_u(reg_a, imm64, reg_b_value),
            0x18 /* rA = rB % rC    (signed)   */ => self.exec_mod_s(reg_a, reg_b_value, reg_c_value),
            0x19 /* rA = rB % imm   (signed)   */ => self.exec_mod_s(reg_a, reg_b_value, imm64),
            0x1A /* rA = imm % rB   (signed)   */ => self.exec_mod_s(reg_a, imm64, reg_b_value),
            0x1B /* rA = rB >> rC              */ => self.regs[reg_a] = reg_b_value.checked_shr(reg_c_value as u32).unwrap_or(0),
            0x1C /* rA = rB >> imm             */ => self.regs[reg_a] = reg_b_value.checked_shr(imm64 as u32).unwrap_or(0),
            0x1D /* rA = rB << rC              */ => self.regs[reg_a] = reg_b_value.checked_shl(reg_c_value as u32).unwrap_or(0),
            0x1E /* rA = rB << imm             */ => self.regs[reg_a] = reg_b_value.checked_shl(imm64 as u32).unwrap_or(0),
            0x1F /* ror                        */ => self.regs[reg_a] = reg_b_value.rotate_right(1),
            0x20 /* rol                        */ => self.regs[reg_a] = reg_b_value.rotate_left(1),
            0x21 /* rA = rB                    */ => self.regs[reg_a] = reg_b_value,
            0x22 /* ldi                        */ => self.regs[reg_a] = Self::ldi(reg_a_value, (instruction & (0b11 << 16)) >> 16, imm16),
            0x23 /* UNUSED                     */ => unimplemented!("This opcode is unused"),
            0x24 /* rA = memory[rB]            */ => self.load_memory_to_register(reg_a, reg_b_value, byte_pos),
            0x25 /* rA = memory[imm]           */ => self.load_memory_to_register_imm(reg_a, imm16, byte_pos),
            0x26 /* memory[rB] = rA            */ => self.store_register_to_memory(self.regs[reg_a], reg_b_value, byte_pos),
            0x27 /* memory[imm] = rA           */ => self.store_register_to_memory_imm(self.regs[reg_a], imm16, byte_pos),
            0x28 /* push                       */ => unimplemented!("Push not implemented"),
            0x29 /* pop                        */ => unimplemented!("Pop not implemented"),
            0x2A /* rA.cmp(rB)      (unsigned) */ => self.compare(reg_a_value, reg_b_value),
            0x2B /* rA.cmp(imm)     (unsigned) */ => self.compare(reg_a_value, imm64),
            0x2C /* imm.cmp(rA)     (unsigned) */ => self.compare(imm64, reg_a_value),
            0x2D /* rA.cmp(rB)      (signed)   */ => self.compare(reg_a_value as i64, reg_b_value as i64),
            0x2E /* rA.cmp(imm)     (signed)   */ => self.compare(reg_a_value as i64, imm64 as i64),
            0x2F /* imm.cmp(rA)     (signed)   */ => self.compare(imm64 as i64, reg_a_value as i64),
            0x30 /* b                          */ => self.branch(true, reg_a_value),
            0x31 /* b                          */ => self.branch(true, imm16),
            0x32 /* bg                         */ => self.branch(self.flags.greater, reg_a_value),
            0x33 /* bg                         */ => self.branch(self.flags.greater, imm16),
            0x34 /* be                         */ => self.branch(self.flags.equal, reg_a_value),
            0x35 /* be                         */ => self.branch(self.flags.greater, imm16),
            0x36 /* bs                         */ => self.branch(self.flags.smaller, reg_a_value),
            0x37 /* bs                         */ => self.branch(self.flags.smaller, imm16),
            0x38 /* bng                        */ => self.branch(!self.flags.greater, reg_a_value),
            0x39 /* bng                        */ => self.branch(!self.flags.greater, imm16),
            0x3A /* bne                        */ => self.branch(!self.flags.equal, reg_a_value),
            0x3B /* bne                        */ => self.branch(!self.flags.equal, imm16),
            0x3C /* bns                        */ => self.branch(!self.flags.smaller, reg_a_value),
            0x3D /* bns                        */ => self.branch(!self.flags.smaller, imm16),
            (0x3E..=0xFF) =>  println!("Unknown opcode: {:#x}", opcode),
        }

        let curr_instr_ptr = self.regs[INSTRUCTION_POINTER];

        if prev_instr_ptr == curr_instr_ptr {
            self.regs[INSTRUCTION_POINTER] = self.regs[INSTRUCTION_POINTER].wrapping_add(4);
        }
    }

    fn branch<T: UsableForBranch>(&mut self, condition: bool, offset: T) {
        if condition {
            let current_ip = self.regs[INSTRUCTION_POINTER] as i64;
            self.regs[INSTRUCTION_POINTER] = offset.to_i64().wrapping_mul(4).wrapping_add(current_ip) as u64;
        }
    }

    /// Load a byte from memory at the address in `reg_b`, modify the specified byte in `reg_a`.
    fn load_memory_to_register(&mut self, reg_a: usize, reg_b_value: u64, byte_pos: u8) {
        let address = reg_b_value as usize;
        if address >= self.memory.len() {
            println!("Memory access out of bounds: address {}", address);
            return;
        }
        self.regs[reg_a] = Self::set_byte(self.regs[reg_a], byte_pos, self.memory[address]);
    }

    /// Load a byte from memory at the immediate address, modify the specified byte in `reg_a`.
    fn load_memory_to_register_imm(&mut self, reg_a: usize, imm16: u16, byte_pos: u8) {
        let address = imm16 as usize;
        if address >= self.memory.len() {
            println!("Memory access out of bounds: address {}", address);
            return;
        }
        self.regs[reg_a] = Self::set_byte(self.regs[reg_a], byte_pos, self.memory[address]);
    }

    /// Store a byte from `reg_a` into memory at the address in `reg_b`.
    fn store_register_to_memory(&mut self, reg_a_value: u64, reg_b_value: u64, byte_pos: u8) {
        let address = reg_b_value as usize;
        if address >= self.memory.len() {
            println!("Memory access out of bounds: address {}", address);
            return;
        }
        self.memory[address] = Self::get_byte(reg_a_value, byte_pos);
    }

    /// Store a byte from `reg_a` into memory at the immediate address.
    fn store_register_to_memory_imm(&mut self, reg_a_value: u64, imm16: u16, byte_pos: u8) {
        let address = imm16 as usize;
        if address >= self.memory.len() {
            println!("Memory access out of bounds: address {}", address);
            return;
        }
        self.memory[address] = Self::get_byte(reg_a_value, byte_pos);
    }

    fn ldi(reg: u64, slice: u32, imm: u16) -> u64 {
        assert!(slice <= 3); // TODO: This might panic for now, in the future this would trigger an interrupt
        let shift = slice * 16;
        let mask = !(0xFFFF << shift);
        (reg & mask) | ((imm as u64) << shift)
    }

    fn set_byte(v: u64, byte: u8, new: u8) -> u64 {
        assert!(byte <= 7); // TODO: This might panic for now, in the future this would trigger an interrupt
        let shift = byte * 8;
        let cleared_v = v & !(0xFFu64 << shift); // Clear target byte
        cleared_v | ((new as u64) << shift)
    }

    fn get_byte(v: u64, byte: u8) -> u8 {
        assert!(byte <= 7); // TODO: This might panic for now, in the future this would trigger an interrupt
        let shift = byte * 8;
        ((v >> shift) & 0xFF) as u8
    }
}

trait UsableForBranch {
    fn to_i64(self) -> i64;
}

impl UsableForBranch for u64 {
    fn to_i64(self) -> i64 {
        self as i64
    }
}

impl UsableForBranch for u16 {
    fn to_i64(self) -> i64 {
        self as i16 as i64
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand;
    use rand::Rng;

    #[test]
    fn stress_test() {
        let mut cpu = CPU::new();

        for i in 0..4000 {
            let random_instr = rand::thread_rng().gen_range(0x00000000..=0x3D000000);
            println!("{}. {:#8x}", i+1, random_instr);
            cpu.exec(random_instr);
            println!("regs: {:?}", cpu.regs);
            println!();
        }

        // Print CPU state after running random instructions
        println!("Registers after random stress test: {:?}", cpu.regs);
        println!("Flags after random stress test: {:?}", cpu.flags);
    }
}