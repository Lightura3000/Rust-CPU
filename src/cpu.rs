use std::cmp::Ordering;
use std::fmt::Display;

type InstrFn = fn(&mut CPU, u32);
const MEMORY_SIZE: usize = 4096;
const INSTR_PTR: usize = 15;

#[derive(Debug, Eq, PartialEq)]
pub struct CPU {
    pub regs: [u64; 16],
    pub memory: [u8; 4096],
    pub privileged: bool,
    pub flags: Flags,
}

#[derive(Debug, Default, Eq, PartialEq)]
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
    #[allow(dead_code)]
    pub fn new(regs: [u64; 16], memory: [u8; MEMORY_SIZE], privileged: bool, flags: Flags) -> Self {
        Self {
            regs,
            memory,
            privileged,
            flags,
        }
    }

    pub fn default() -> Self {
        Self {
            regs: [0; 16],
            memory: [0; MEMORY_SIZE],
            privileged: true,
            flags: Flags::default(),
        }
    }

    pub fn run(&mut self, cycles: u64) {
        for _ in 0..cycles {
            let address = self.regs[INSTR_PTR] as usize;
            let instruction = self.fetch_instruction(address);
            self.exec(instruction);
        }
    }

    pub fn set_instruction_ptr(&mut self, value: u64) {
        self.regs[INSTR_PTR] = value;
    }

    pub fn exec(&mut self, instruction: u32) {
        // Using a lookup table for opcodes instead of a match is probably faster
        const INSTRUCTION_TABLE: [InstrFn; 10] = [
            |_, _| { }, // nop
            CPU::execute_arithmetic_operations,
            CPU::execute_bitwise_operations,
            CPU::execute_shift_and_rotate,
            CPU::execute_data_movement_memory_stack,
            CPU::execute_comparison,
            CPU::execute_branching,
            CPU::execute_conversion,
            CPU::execute_floating,
            CPU::execute_double,
        ];

        const OPCODE_MASK: u32 = 0xF0000000;

        let opcode = ((instruction & OPCODE_MASK) >> OPCODE_MASK.trailing_zeros()) as usize;

        let prev_instr_ptr = self.regs[INSTR_PTR];

        match INSTRUCTION_TABLE.get(opcode) {
            None => Self::complain(format!("Invalid instruction: {:#010x}", instruction)),
            Some(function) => function(self, instruction),
        }

        if self.regs[INSTR_PTR] == prev_instr_ptr {
            self.regs[INSTR_PTR] += 4;
        }
    }

    fn execute_arithmetic_operations(&mut self, instruction: u32) {
        // Type for an arithmetic function: (self, dest, lhs, rhs)
        type ArithmeticOperationFn = fn(&mut CPU, usize, u64, u64);

        const OPERATION_MASK: u32 = 0x0000_000F;
        const DEST_REG_MASK: u32  = 0x0F00_0000;
        const SRC1_REG_MASK: u32  = 0x00F0_0000;
        const SRC2_REG_MASK: u32  = 0x000F_0000;
        const IMMEDIATE_MASK: u32 = 0x000F_FFF0;

        let operation = instruction & OPERATION_MASK;
        let dest = ((instruction & DEST_REG_MASK) >> DEST_REG_MASK.trailing_zeros()) as usize;
        let src1 = ((instruction & SRC1_REG_MASK) >> SRC1_REG_MASK.trailing_zeros()) as usize;
        let src2 = ((instruction & SRC2_REG_MASK) >> SRC2_REG_MASK.trailing_zeros()) as usize;

        let b = self.regs[src1];
        let c = self.regs[src2];
        let imm = ((instruction & IMMEDIATE_MASK) >> IMMEDIATE_MASK.trailing_zeros()) as u64;

        // decide which CPU method we’ll call (add, sub, mul, etc.)
        let arith_fn: ArithmeticOperationFn = match operation {
            0x0 | 0x1       => Self::addition,
            0x2 | 0x3 | 0x4 => Self::subtraction,
            0x5 | 0x6       => Self::multiplication,
            0x7 | 0x8 | 0x9 => Self::unsigned_division,
            0xA | 0xB | 0xC => Self::signed_division,
            _ => {
                Self::complain(format!("Invalid arithmetic operation code: {operation:#010X}"));
                return;
            },
        };

        // decide which operands (lhs, rhs) to pass
        let (lhs, rhs) = match operation {
            0x0 => (b,   c),
            0x1 => (b,   imm),
            0x2 => (b,   c),
            0x3 => (b,   imm),
            0x4 => (imm, b),
            0x5 => (b,   c),
            0x6 => (b,   imm),
            0x7 => (b,   c),
            0x8 => (b,   imm),
            0x9 => (imm, b),
            0xA => (b,   c),
            0xB => (b,   imm),
            0xC => (imm, b),
            _ => unreachable!("Invalid arithmetic operation code: {operation:#010X}. This should not happen."),
        };

        // call the chosen arithmetic function with the decoded operands
        arith_fn(self, dest, lhs, rhs);
    }

    fn execute_bitwise_operations(&mut self, instruction: u32) {
        const OPERATION_MASK: u32 = 0b111;
        const DEST_REG_MASK: u32  = 0x0F00_0000;
        const SRC1_REG_MASK: u32  = 0x00F0_0000;
        const SRC2_REG_MASK: u32  = 0x000F_0000;

        let operation = instruction & OPERATION_MASK;
        let dest = ((instruction & DEST_REG_MASK) >> DEST_REG_MASK.trailing_zeros()) as usize;
        let src1 = ((instruction & SRC1_REG_MASK) >> SRC1_REG_MASK.trailing_zeros()) as usize;
        let src2 = ((instruction & SRC2_REG_MASK) >> SRC2_REG_MASK.trailing_zeros()) as usize;

        let b = self.regs[src1];
        let c = self.regs[src2];

        self.regs[dest] = match operation {
            0 => b & c,
            1 => b | c,
            2 => b ^ c,
            3 => !(b & c),
            4 => !(b | c),
            5 => !(b ^ c),
            6 => !b,
            _ => {
                Self::complain(format!("Invalid arithmetic operation code: {operation:#010X}"));
                return;
            },
        }
    }

    fn execute_shift_and_rotate(&mut self, instruction: u32) {
        const OPERATION_MASK: u32 = 0b111;
        const DEST_REG_MASK: u32  = 0x0F00_0000;
        const SRC1_REG_MASK: u32  = 0x00F0_0000;
        const SRC2_REG_MASK: u32  = 0x000F_0000;
        const IMMEDIATE_MASK: u32 = 0b11_1111_0000;

        let operation = instruction & OPERATION_MASK;
        let dest = ((instruction & DEST_REG_MASK) >> DEST_REG_MASK.trailing_zeros()) as usize;
        let src1 = ((instruction & SRC1_REG_MASK) >> SRC1_REG_MASK.trailing_zeros()) as usize;
        let src2 = ((instruction & SRC2_REG_MASK) >> SRC2_REG_MASK.trailing_zeros()) as usize;

        let b = self.regs[src1];
        let c = self.regs[src2];
        let imm = ((instruction & IMMEDIATE_MASK) >> IMMEDIATE_MASK.trailing_zeros()) as u64;

        self.regs[dest] = match operation {
            0 => b >> c,
            1 => b >> imm,
            2 => b << c,
            3 => b << imm,
            4 => b.rotate_right(c as u32),
            5 => b.rotate_right(imm as u32),
            6 => b.rotate_left(c as u32),
            7 => b.rotate_left(imm as u32),
            _ => unreachable!("Invalid operation code: {operation:#010x}"),
        }
    }

    fn execute_data_movement_memory_stack(&mut self, instruction: u32) {
        const OPERATION_MASK: u32 = 0b111;
        const DEST_REG_MASK: u32  = 0x0F00_0000;
        const SRC1_REG_MASK: u32  = 0x00F0_0000;
        const IMMEDIATE_MASK: u32 = 0x00FF_FF00;
        const SECTION_MASK: u32 = 0b0111_0000;

        let operation = instruction & OPERATION_MASK;
        let dest = ((instruction & DEST_REG_MASK) >> DEST_REG_MASK.trailing_zeros()) as usize;
        let src = ((instruction & SRC1_REG_MASK) >> SRC1_REG_MASK.trailing_zeros()) as usize;
        let imm = ((instruction & IMMEDIATE_MASK) >> IMMEDIATE_MASK.trailing_zeros()) as u16;
        let section = ((instruction & SECTION_MASK) >> SECTION_MASK.trailing_zeros()) as u8;

        let b = self.regs[src];

        match operation {
            0 => self.regs[dest] = self.regs[src],
            1 => {
                const CHUNK_MASK: u32 = 0b0011_0000;
                let chunk = ((instruction & CHUNK_MASK) >> CHUNK_MASK.trailing_zeros()) as u8;
                self.regs[dest] = Self::set_chunk(self.regs[dest], imm, chunk);
            }
            2 => self.regs[dest] = Self::set_byte(self.regs[dest], self.memory[b as usize], section),
            3 => self.regs[dest] = Self::set_byte(self.regs[dest], self.memory[imm as usize], section),
            4 => self.memory[b as usize] = Self::get_byte(self.regs[dest], section),
            5 => self.memory[imm as usize] = Self::get_byte(self.regs[dest], section),
            6 => Self::complain("Push not implemented yet"),
            7 => Self::complain("Pop not implemented yet"),
            _ => unreachable!("Invalid operation code: {operation:#04x}"),
        }
    }

    fn execute_comparison(&mut self, instruction: u32) {
        const REG1_MASK: u32      = 0x0F00_0000;
        const REG2_MASK: u32      = 0x00F0_0000;
        const IMMEDIATE_MASK: u32 = 0x00FF_FF00;
        const COMPARISON_MASK: u32 = 0b111;

        let reg1 = ((instruction & REG1_MASK) >> REG1_MASK.trailing_zeros()) as usize;
        let reg2 = ((instruction & REG2_MASK) >> REG2_MASK.trailing_zeros()) as usize;
        let imm = ((instruction & IMMEDIATE_MASK) >> IMMEDIATE_MASK.trailing_zeros()) as u64;
        let comparison = instruction & COMPARISON_MASK;

        let reg1 = self.regs[reg1];
        let reg2 = self.regs[reg2];

        fn compare<T: Ord>(cpu: &mut CPU, a: T, b: T) {
            let cmp = a.cmp(&b);
            cpu.flags.greater = cmp == Ordering::Greater;
            cpu.flags.equal = cmp == Ordering::Equal;
            cpu.flags.smaller = cmp == Ordering::Less;
        }

        fn partial_compare<T: PartialOrd>(cpu: &mut CPU, a: T, b: T) {
            match a.partial_cmp(&b) {
                None => {
                    cpu.flags.greater = false;
                    cpu.flags.equal = false;
                    cpu.flags.smaller = false;
                }
                Some(cmp) => {
                    cpu.flags.greater = cmp == Ordering::Greater;
                    cpu.flags.equal = cmp == Ordering::Equal;
                    cpu.flags.smaller = cmp == Ordering::Less;
                }
            }
        }

        match instruction {
            0 => compare(self, reg1, reg2),
            1 => compare(self, reg1, imm),
            2 => compare(self, imm, reg1),
            3 => compare(self, reg1 as i64, reg2 as i64),
            4 => compare(self, reg1 as i64, imm as i64),
            5 => compare(self, imm as i64, reg1 as i64),
            6 => partial_compare(self, f32::from_bits(reg1 as u32), f32::from_bits(reg2 as u32)),
            7 => partial_compare(self, f64::from_bits(reg1),        f64::from_bits(reg2)),
            _ => Self::complain(format!("Invalid comparison code: {comparison:#04x}")),
        }
    }

    fn execute_branching(&mut self, instruction: u32) {
        const BRANCH_CONDITION: u32 = 0b1111;
        const BRANCH_AMOUNT_MASK: u32 = 0x0F00_0000;
        const IMMEDIATE_MASK: u32     = 0x0FFF_F000;

        let branch_condition = instruction & BRANCH_CONDITION;
        let branch_amount = ((instruction & BRANCH_AMOUNT_MASK) >> BRANCH_AMOUNT_MASK.trailing_zeros()) as usize;
        let imm_offset = ((instruction & IMMEDIATE_MASK) >> IMMEDIATE_MASK.trailing_zeros()) as u16;

        let reg_offset = self.regs[branch_amount];

        fn branch_u16(cpu: &mut CPU, condition: bool, offset: u16) {
            if condition {
                let current_ip = cpu.regs[INSTR_PTR] as i64;
                cpu.regs[INSTR_PTR] = (offset as i16 as i64 * 4 + current_ip) as u64;
            }
        }

        fn branch_u64(cpu: &mut CPU, condition: bool, offset: u64) {
            if condition {
                let current_ip = cpu.regs[INSTR_PTR] as i64;
                cpu.regs[INSTR_PTR] = (offset as i64 * 4 + current_ip) as u64;
            }
        }

        match branch_condition {
            0x0 => branch_u64(self, true, reg_offset),
            0x1 => branch_u16(self, true, imm_offset),
            0x2 => branch_u64(self, self.flags.greater, reg_offset),
            0x3 => branch_u16(self, self.flags.greater, imm_offset),
            0x4 => branch_u64(self, self.flags.equal, reg_offset),
            0x5 => branch_u16(self, self.flags.equal, imm_offset),
            0x6 => branch_u64(self, self.flags.smaller, reg_offset),
            0x7 => branch_u16(self, self.flags.smaller, imm_offset),
            0x8 => branch_u64(self, self.flags.greater || self.flags.equal, reg_offset),
            0x9 => branch_u16(self, self.flags.greater || self.flags.equal, imm_offset),
            0xA => branch_u64(self, !self.flags.equal, reg_offset),
            0xB => branch_u16(self, !self.flags.equal, imm_offset),
            0xC => branch_u64(self, self.flags.smaller || self.flags.equal, reg_offset),
            0xD => branch_u16(self, self.flags.smaller || self.flags.equal, imm_offset),
            0xE | 0xF => Self::complain(format!("Using unassigned branching condition {:#x}", branch_condition)),
            _ => unreachable!("Invalid branching code: {branch_condition:#04x}"),
        }
    }

    fn execute_conversion(&mut self, instruction: u32) {
        const CONVERSION_MASK: u32 = 0b111;
        const REG_MASK: u32 = 0x0F000000;
        const IMMEDIATE_MASK: u32 = 0x00FFFF00;

        let conversion = instruction & CONVERSION_MASK;
        let reg = ((instruction & REG_MASK) >> REG_MASK.trailing_zeros()) as usize;
        let imm = ((instruction & IMMEDIATE_MASK) >> IMMEDIATE_MASK.trailing_zeros()) as u16;

        let a = self.regs[reg];

        match conversion {
            0 => self.regs[reg] = (imm as i16 as f32).to_bits() as u64, // 0 => Immediate to float
            1 => self.regs[reg] = (imm as i16 as f64).to_bits(), // 1 => Immediate to double
            2 => self.regs[reg] = (a as i64 as f32).to_bits() as u64, // 2 => Int to float
            3 => self.regs[reg] = (a as i64 as f64).to_bits(), // 3 => Int to double
            4 => self.regs[reg] = f32::from_bits(a as u32) as i64 as u64, // 4 => Float to int
            5 => self.regs[reg] = (f32::from_bits(a as u32) as f64).to_bits(), // 5 => Float to double
            6 => self.regs[reg] = f64::from_bits(a) as i64 as u64, // 6 => Double to int
            7 => self.regs[reg] = (f64::from_bits(a) as f32).to_bits() as u64, // 7 => Double to float
            _ => Self::complain(format!("Invalid conversion instruction: {conversion:#04x}")),
        }
    }

    fn execute_floating(&mut self, instruction: u32) {
        const DEST_REG_MASK: u32 = 0x0F000000;
        const SRC1_REG_MASK: u32 = 0x00F00000;
        const SRC2_REG_MASK: u32 = 0x000F0000;
        const COMPARISON_MASK: u32 = 0b0111_0000_0000;
        const OPERATION_MASK: u32 = 0b1_1111;

        let dest = ((instruction & DEST_REG_MASK) >> DEST_REG_MASK.trailing_zeros()) as usize;
        let src1 = ((instruction & SRC1_REG_MASK) >> SRC1_REG_MASK.trailing_zeros()) as usize;
        let src2 = ((instruction & SRC2_REG_MASK) >> SRC2_REG_MASK.trailing_zeros()) as usize;
        let operation = instruction & OPERATION_MASK;

        let a = f32::from_bits(self.regs[dest] as u32);
        let b = f32::from_bits(self.regs[src1] as u32);
        let c = f32::from_bits(self.regs[src2] as u32);

        self.regs[dest] = match operation {
            0x00 => b + c,
            0x01 => b - c,
            0x02 => b * c,
            0x03 => b / c,
            0x04 => b % c,
            0x05 => -a,
            0x06 => 1.0 / b,
            0x07 => b.powf(c),
            0x08 => f32::exp(b),
            0x09 => b.nth_root(c),
            0x0A => b.sqrt(),
            0x0B => b.nth_root(3.0),
            0x0C => b * b,
            0x0D => b * b * b,
            0x0E => b.log(c),
            0x0F => f32::ln(b),
            0x10 => b.abs(),
            0x11 => b.sin(),
            0x12 => b.cos(),
            0x13 => b.tan(),
            0x14 => b.asin(),
            0x15 => b.acos(),
            0x16 => b.atan(),
            0x17 => b.floor(),
            0x18 => b.ceil(),
            0x19 => b.round(),
            0x1A => a.min(b),
            0x1B => a.max(b),
            0x1C => a.signum(),
            0x1D => (a - b).abs(),
            0x1E => f32::INFINITY,
            0x1F => f32::NAN,
            _ => unreachable!("Invalid operation: {comparison:#04x}"),
        }.to_bits() as u64
    }

    fn execute_double(&mut self, instruction: u32) {
        const DEST_REG_MASK: u32 = 0x0F000000;
        const SRC1_REG_MASK: u32 = 0x00F00000;
        const SRC2_REG_MASK: u32 = 0x000F0000;
        const COMPARISON_MASK: u32 = 0b0111_0000_0000;
        const OPERATION_MASK: u32 = 0b1_1111;

        let dest = ((instruction & DEST_REG_MASK) >> DEST_REG_MASK.trailing_zeros()) as usize;
        let src1 = ((instruction & SRC1_REG_MASK) >> SRC1_REG_MASK.trailing_zeros()) as usize;
        let src2 = ((instruction & SRC2_REG_MASK) >> SRC2_REG_MASK.trailing_zeros()) as usize;
        let comparison = instruction & COMPARISON_MASK;
        let operation = instruction & OPERATION_MASK;

        let a = f64::from_bits(self.regs[dest]);
        let b = f64::from_bits(self.regs[src1]);
        let c = f64::from_bits(self.regs[src2]);

        self.regs[dest] = match operation {
            0x00 => b + c,
            0x01 => b - c,
            0x02 => b * c,
            0x03 => b / c,
            0x04 => b % c,
            0x05 => -a,
            0x06 => 1.0 / b,
            0x07 => b.powf(c),
            0x08 => f64::exp(b),
            0x09 => b.nth_root(c),
            0x0A => b.sqrt(),
            0x0B => b.nth_root(3.0),
            0x0C => b * b,
            0x0D => b * b * b,
            0x0E => b.log(c),
            0x0F => f64::ln(b),
            0x10 => b.abs(),
            0x11 => b.sin(),
            0x12 => b.cos(),
            0x13 => b.tan(),
            0x14 => b.asin(),
            0x15 => b.acos(),
            0x16 => b.atan(),
            0x17 => b.floor(),
            0x18 => b.ceil(),
            0x19 => b.round(),
            0x1A => a.min(b),
            0x1B => a.max(b),
            0x1C => a.signum(),
            0x1D => (a - b).abs(),
            0x1E => f64::INFINITY,
            0x1F => f64::NAN,
            _ => {
                Self::complain(format!("Invalid operation: {comparison:#04x}"));
                return;
            }
        }.to_bits() as u64
    }

    /// Helper function to perform an unsigned arithmetic operation and set flags
    fn exec_arithmetic_operation(&mut self, reg_a: usize, left_hand_side: u64, right_hand_side: u64, op_unsigned: fn(u64, u64) -> (u64, bool), op_signed: fn(i64, i64) -> (i64, bool)) {
        let (result, carry) = op_unsigned(left_hand_side, right_hand_side);
        // For signed overflow detection, we perform the operation in signed space
        let (signed_result, signed_overflow) = op_signed(left_hand_side as i64, right_hand_side as i64);

        self.regs[reg_a] = result;
        self.flags.carry = carry;
        self.flags.zero = result == 0;
        self.flags.negative = signed_result < 0;
        self.flags.overflow = signed_overflow;
    }

    fn addition(&mut self, dest_reg: usize, lhs: u64, rhs: u64) {
        self.exec_arithmetic_operation(dest_reg, lhs, rhs, u64::overflowing_add, i64::overflowing_add);
    }

    fn subtraction(&mut self, dest_reg: usize, lhs: u64, rhs: u64) {
        self.exec_arithmetic_operation(dest_reg, lhs, rhs, u64::overflowing_sub, i64::overflowing_sub);
    }

    fn multiplication(&mut self, dest_reg: usize, lhs: u64, rhs: u64) {
        self.exec_arithmetic_operation(dest_reg, lhs, rhs, u64::overflowing_mul, i64::overflowing_mul);
    }

    fn unsigned_division(&mut self, dest_reg: usize, lhs: u64, rhs: u64) {
        if rhs == 0 {
            // Division by zero
            self.regs[dest_reg] = 0;
            self.flags.carry = false;
            self.flags.zero = true;
            self.flags.negative = false;
            self.flags.overflow = true;
        } else {
            let result = lhs / rhs;
            self.regs[dest_reg] = result;
            self.flags.carry = false;
            self.flags.zero = result == 0;
            self.flags.negative = (result as i64) < 0;
            self.flags.overflow = false; // Unsigned division doesn't typically overflow
        }
    }

    fn signed_division(&mut self, dest_reg: usize, lhs: u64, rhs: u64) {
        let lhs = lhs as i64;
        let rhs = rhs as i64;

        if rhs == 0 {
            // Division by zero
            self.flags.overflow = true;
            self.regs[dest_reg] = 0;
            self.flags.zero = true;
            self.flags.negative = false;
            self.flags.carry = false;
        } else {
            // Check the potential overflow case: i64::MIN / -1
            // This would cause an overflow in signed division since the result can't be represented.
            if lhs == i64::MIN && rhs == -1 {
                // In many architectures this causes an arithmetic exception.
                // Here, we treat it as overflow.
                self.regs[dest_reg] = lhs.wrapping_div(rhs) as u64;
                self.flags.overflow = true;
            } else {
                self.regs[dest_reg] = (lhs / rhs) as u64;
                self.flags.overflow = false;
            }

            let result = self.regs[dest_reg] as i64;
            self.flags.carry = false;
            self.flags.zero = result == 0;
            self.flags.negative = result < 0;
        }
    }

    fn set_chunk(reg: u64, data: u16, chunk: u8) -> u64 {
        assert!(chunk < 4);
        let shift = chunk * 16;
        let mask: u64 = 0xFFFF << shift;
        (reg & !mask) | ((data as u64) << shift)
    }

    fn set_byte(reg: u64, data: u8, byte: u8) -> u64 {
        assert!(byte < 8);
        let shift = byte * 8;
        let mask: u64 = 0xFF << shift;
        (reg & !mask) | ((data as u64) << shift)
    }

    fn get_byte(reg: u64, byte: u8) -> u8 {
        assert!(byte < 8);
        let shift = byte * 8;
        ((reg & (0xFF << shift)) >> shift) as u8
    }

    /// This prints the message when cargo is using the test profile, otherwise it panics
    fn complain(msg: impl Display) {
        #[cfg(test)] {
            println!("{}", msg);
        }

        #[cfg(not(test))] {
            panic!("{}", msg);
        }
    }

    fn fetch_instruction(&mut self, address: usize) -> u32 {
        let byte1 = self.memory[address] as u32;
        let byte2 = self.memory[address + 1] as u32;
        let byte3 = self.memory[address + 2] as u32;
        let byte4 = self.memory[address + 3] as u32;
        (byte1 << 24) | (byte2 << 16) | (byte3 << 8) | byte4
    }
}

trait NthRoot {
    fn nth_root(self, n: Self) -> Self;
}

impl NthRoot for f32 {
    fn nth_root(self, n: f32) -> Self {
        // If n is zero or negative, return NaN.
        if n <= 0.0 {
            return f32::NAN;
        }

        // Calculate the nth root as value^(1/n).
        self.powf(1.0 / n)
    }
}

impl NthRoot for f64 {
    fn nth_root(self, n: f64) -> Self {
        // If n is zero or negative, return NaN.
        if n <= 0.0 {
            return f64::NAN;
        }

        // Calculate the nth root as value^(1/n).
        self.powf(1.0 / n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    #[ignore]
    fn stress_test() {
        let iterations = 10_000_000;

        let mut cpu = CPU::default();
        let fill = iterations.to_string().len();

        for i in 0..iterations {
            let random_instr = rand::thread_rng().gen_range(0x00000000..=0x3D000000);
            cpu.exec(random_instr);

            println!("{:0fill$}. {:#010x}", i+1, random_instr);
        }

        println!("Registers after stress test: {:?}", cpu.regs);
        println!("Flags after stress test: {:?}", cpu.flags);
    }
}
