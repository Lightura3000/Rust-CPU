use std::{collections::HashMap, fmt::Display};
use either::{Either, Left, Right};
use crate::assembler::{
    constructor::{Instruction, U2, U3, U6},
    opcode::Opcode,
    register::Register,
    token::{Token, TokenVariant::*},
    assembly_error::{AssemblyError, AssemblyErrorVariant},
};

pub fn assemble(src: String) -> Result<Vec<u32>, String> {
    let mut instructions = Vec::new();
    let mut labels = HashMap::new();

    let token_lines = tokenize(&src);

    for tokens in token_lines.iter() {
        let first_token = match tokens.first() {
            None => panic!("Empty token line"),
            Some(token) => token
        };

        let line = first_token.line;

        if let Label(name) = &first_token.variant {
            labels.insert(name.to_owned(), line);
            continue;
        }

        match assemble_line(line, tokens, &mut labels) {
            Ok(instruction) => instructions.push(instruction),
            Err(error_variant) => return Err(AssemblyError { line, variant: error_variant }.to_string()),
        }
    }

    Ok(instructions.iter().map(|instr| instr.assemble()).collect::<Vec<_>>())
}

fn assemble_line(line_i: usize, line_tokens: &[Token], labels: &HashMap<String, usize>) -> Result<Instruction, AssemblyErrorVariant> {
    let opcode = match &line_tokens.first().unwrap().variant {
        Opcode(opc) => opc,
        _ => return Err(AssemblyErrorVariant::NeedsStartingOpcode),
    };

    let params = &line_tokens[1..];

    match opcode {
        Opcode::Nop => process_nop(&params),
        Opcode::Add => process_add(&params),
        Opcode::Subtract => process_sub(&params),
        Opcode::Multiply => process_mul(&params),
        Opcode::Divide => process_div(&params),
        Opcode::DivideSigned => process_div_signed(&params),
        Opcode::And => process_and(&params),
        Opcode::Or => process_or(&params),
        Opcode::Xor => process_xor(params),
        Opcode::Nand => process_nand(&params),
        Opcode::Nor => process_nor(&params),
        Opcode::Xnor => process_xnor(&params),
        Opcode::Not => process_not(&params),
        Opcode::RightShift => process_right_shift(&params),
        Opcode::LeftShift => process_left_shift(&params),
        Opcode::RightRoll => process_right_roll(&params),
        Opcode::LeftRoll => process_left_roll(&params),
        Opcode::Move => process_move(&params),
        Opcode::LoadImmediate => process_load_immediate(&params),
        Opcode::LoadRegister => process_load_register(&params),
        Opcode::StoreRegister => process_store_register(&params),
        Opcode::Push => process_push(&params),
        Opcode::Pop => process_pop(&params),
        Opcode::Compare => process_compare(&params),
        Opcode::CompareFloat => process_compare_float(&params),
        Opcode::CompareDouble => process_compare_double(&params),
        Opcode::Branch => process_unconditional_branch(line_i, &params, labels),
        Opcode::BranchGreater => process_branch_greater(line_i, &params, labels),
        Opcode::BranchEqual => process_branch_equal(line_i, &params, labels),
        Opcode::BranchSmaller => process_branch_smaller(line_i, &params, labels),
        Opcode::BranchGreaterEqual => process_branch_greater_equal(line_i, &params, labels),
        Opcode::BranchNotEqual => process_branch_not_equal(line_i, &params, labels),
        Opcode::BranchSmallerEqual => process_branch_smaller_equal(line_i, &params, labels),
        Opcode::ImmediateToFloat => process_immediate_to_float(&params),
        Opcode::ImmediateToDouble => process_immediate_to_double(&params),
        Opcode::IntegerToFloat => process_int_to_float(&params),
        Opcode::IntegerToDouble => process_int_to_double(&params),
        Opcode::FloatToInteger => process_float_to_int(&params),
        Opcode::FloatToDouble => process_float_to_double(&params),
        Opcode::DoubleToInteger => process_double_to_int(&params),
        Opcode::DoubleToFloat => process_double_to_float(&params),
    }.into()
}

fn tokenize(src: &String) -> Vec<Vec<Token>> {
    let mut tokens = Vec::new();

    for (line, content) in src.lines().enumerate() {
        let mut line_tokens = Vec::new();

        for split in content.split_whitespace() {
            line_tokens.push(Token::construct(line, split));
        }

        if line_tokens.len() > 0 {
            tokens.push(line_tokens);
        }
    }

    // tokens.iter().for_each(|token| println!("{:?}", token));

    tokens
}

fn expect_param_amount(params: &[Token], expected: usize) -> Result<(), AssemblyErrorVariant> {
    if params.len() != expected {
        return Err(AssemblyErrorVariant::ParamAmount { expected, got: params.len() });
    }
    Ok(())
}

fn process_nop(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 0)?;
    Ok(Instruction::Nop)
}

fn process_add(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, a, b) = process_arithmetic_instruction(params)?;
    Ok(Instruction::Add { dest, a, b })
}

fn process_sub(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, a, b) = process_arithmetic_instruction(params)?;
    Ok(Instruction::Subtract { dest, a , b})
}

fn process_mul(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, a, b) = process_arithmetic_instruction(params)?;
    Ok(Instruction::Multiply { dest, a, b })
}

fn process_div(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, a, b) = process_arithmetic_instruction(params)?;
    Ok(Instruction::Divide { dest, a, b })
}

fn process_div_signed(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, a, b) = process_arithmetic_instruction(params)?;
    Ok(Instruction::DivideSigned { dest, a, b })
}

fn process_arithmetic_instruction(params: &[Token]) -> Result<(Register, Either<Register, u16>, Either<Register, u16>), AssemblyErrorVariant> {
    expect_param_amount(params, 3)?;

    match (&params[0].variant, &params[1].variant, &params[2].variant) {
        (Register(dest), Register(a), Register(b)) => Ok((*dest, Left(*a), Left(*b))),
        (Register(dest), Register(a), Unsigned(b)) => Ok((*dest, Left(*a), Right(*b))),
        (Register(dest), Unsigned(a), Register(b)) => Ok((*dest, Right(*a), Left(*b))),
        _ => Err(AssemblyErrorVariant::ParamTypes),
    }
}

fn process_and(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, a, b) = process_bitwise_instruction(params)?;
    Ok(Instruction::And { dest, a, b })
}

fn process_or(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, a, b) = process_bitwise_instruction(params)?;
    Ok(Instruction::Or { dest, a, b })
}

fn process_xor(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, a, b) = process_bitwise_instruction(params)?;
    Ok(Instruction::Xor { dest, a, b })
}

fn process_nand(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, a, b) = process_bitwise_instruction(params)?;
    Ok(Instruction::Nand { dest, a, b })
}

fn process_nor(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, a, b) = process_bitwise_instruction(params)?;
    Ok(Instruction::Nor { dest, a, b })
}

fn process_xnor(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, a, b) = process_bitwise_instruction(params)?;
    Ok(Instruction::Xnor { dest, a, b })
}

fn process_bitwise_instruction(params: &[Token]) -> Result<(Register, Register, Register), AssemblyErrorVariant> {
    expect_param_amount(params, 3)?;

    match (&params[0].variant, &params[1].variant, &params[2].variant) {
        (Register(dest), Register(a), Register(b)) => Ok((*dest, *a, *b)),
        _ => Err(AssemblyErrorVariant::ParamTypes),
    }
}

fn process_not(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 2)?;

    match (&params[0].variant, &params[1].variant) {
        (Register(dest), Register(src)) =>
            Ok(Instruction::Not {
                dest: *dest,
                src: *src,
            }),

        _ => Err(AssemblyErrorVariant::ParamTypes),
    }
}

fn process_right_shift(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, src, amount) = process_shift_and_roll(params)?;
    Ok(Instruction::RightShift { dest, src, amount })
}

fn process_left_shift(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, src, amount) = process_shift_and_roll(params)?;
    Ok(Instruction::LeftShift { dest, src, amount })
}

fn process_right_roll(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, src, amount) = process_shift_and_roll(params)?;
    Ok(Instruction::RightRoll { dest, src, amount })
}

fn process_left_roll(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, src, amount) = process_shift_and_roll(params)?;
    Ok(Instruction::LeftRoll { dest, src, amount })
}

fn process_shift_and_roll(params: &[Token]) -> Result<(Register, Register, Either<Register, U6>), AssemblyErrorVariant> {
    expect_param_amount(params, 3)?;

    match (&params[0].variant, &params[1].variant, &params[2].variant) {
        (Register(dest), Register(src), Register(amount)) => Ok((*dest, *src, Left(*amount))),
        (Register(dest), Register(src), Unsigned(amount)) => {
            if *amount > U6::MAX as u16 {
                return Err(AssemblyErrorVariant::ImmediateTooLarge { max: U6::MAX as u16, got: *amount });
            }

            Ok((*dest, *src, Right(U6::new(*amount as u8).unwrap())))
        }
        _ => Err(AssemblyErrorVariant::ParamTypes),
    }
}

fn process_move(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 2)?;

    match (&params[0].variant, &params[1].variant) {
        (Register(dest), Register(src)) =>
            Ok(Instruction::Move {
                dest: *dest,
                src: *src,
            }),

        _ => Err(AssemblyErrorVariant::ParamTypes),
    }
}

fn process_load_immediate(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 3)?;

    match (&params[0].variant, &params[1].variant, &params[2].variant) {
        (Register(dest), Unsigned(slice), Unsigned(imm)) => {
            if *slice > U6::MAX as u16 {
                return Err(AssemblyErrorVariant::ImmediateTooLarge { max: U6::MAX as u16, got: *slice });
            }

            Ok(Instruction::LoadImmediate {
                dest: *dest,
                slice: U2::new(*slice as u8).unwrap(),
                imm: *imm,
            })
        }

        _ => Err(AssemblyErrorVariant::ParamTypes),
    }
}

fn process_load_register(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 3)?;

    match (&params[0].variant, &params[1].variant, &params[2].variant) {
        (Register(dest), Register(mem_ptr), Unsigned(slice)) => {
            if *slice > U3::MAX as u16 {
                return Err(AssemblyErrorVariant::ImmediateTooLarge { max: U3::MAX as u16, got: *slice });
            }

            Ok(Instruction::LoadRegister {
                dest: *dest,
                mem_ptr: Left(*mem_ptr),
                slice: U3::new(*slice as u8).unwrap()
            })
        }

        (Register(dest), Unsigned(mem_ptr), Unsigned(slice)) => {
            if *slice > U3::MAX as u16 {
                return Err(AssemblyErrorVariant::ImmediateTooLarge { max: U3::MAX as u16, got: *slice });
            }

            Ok(Instruction::LoadRegister {
                dest: *dest,
                mem_ptr: Right(*mem_ptr),
                slice: U3::new(*slice as u8).unwrap()
            })
        }

        _ => Err(AssemblyErrorVariant::ParamTypes),
    }
}

fn process_store_register(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 3)?;

    match (&params[0].variant, &params[1].variant, &params[2].variant) {
        (Register(src), Register(mem_ptr), Unsigned(slice)) => {
            if *slice > U3::MAX as u16 {
                return Err(AssemblyErrorVariant::ImmediateTooLarge { max: U3::MAX as u16, got: *slice });
            }

            Ok(Instruction::StoreRegister {
                src: *src,
                mem_ptr: Left(*mem_ptr),
                slice: U3::new(*slice as u8).unwrap(),
            })
        }

        (Register(src), Unsigned(mem_ptr), Unsigned(slice)) => {
            if *slice > U3::MAX as u16 {
                return Err(AssemblyErrorVariant::ImmediateTooLarge { max: U3::MAX as u16, got: *slice });
            }

            Ok(Instruction::StoreRegister {
                src: *src,
                mem_ptr: Right(*mem_ptr),
                slice: U3::new(*slice as u8).unwrap(),
            })
        }

        _ => Err(AssemblyErrorVariant::ParamTypes),
    }
}

fn process_push(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 1)?;

    match &params[0].variant {
        Register(reg) => Ok(Instruction::Push { reg: *reg }),
        _ => Err(AssemblyErrorVariant::ParamTypes),
    }
}

fn process_pop(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 1)?;

    match &params[0].variant {
        Register(reg) => Ok(Instruction::Pop { reg: *reg }),
        _ => Err(AssemblyErrorVariant::ParamTypes),
    }
}

fn process_compare(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 3)?;

    match (&params[0].variant, &params[1].variant, &params[2].variant) {
        (Unsigned(a), Unsigned(b), Bool(signed)) =>
            Ok(Instruction::Compare {
                a: Right(*a),
                b: Right(*b),
                signed: *signed,
            }),

        (Unsigned(a), Register(b), Bool(signed)) =>
            Ok(Instruction::Compare {
                a: Right(*a),
                b: Left(*b),
                signed: *signed,
            }),

        (Register(a), Unsigned(b), Bool(signed)) =>
            Ok(Instruction::Compare {
                a: Left(*a),
                b: Right(*b),
                signed: *signed,
            }),

        _ => Err(AssemblyErrorVariant::ParamTypes),
    }
}

fn process_compare_float(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 2)?;

    match (&params[0].variant, &params[1].variant) {
        (Register(a), Register(b)) =>
            Ok(Instruction::CompareFloat {
                a: *a,
                b: *b,
            }),

        _ => Err(AssemblyErrorVariant::ParamTypes),
    }
}

fn process_compare_double(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 2)?;

    match (&params[0].variant, &params[1].variant) {
        (Register(a), Register(b)) =>
            Ok(Instruction::CompareDouble {
                a: *a,
                b: *b,
            }),

        _ => Err(AssemblyErrorVariant::ParamTypes),
    }
}

fn process_branch(line_i: usize, token: &Token, labels: &HashMap<String, usize>) -> Result<Either<Register, i16>, AssemblyErrorVariant> {
    match &token.variant {
        Label(name) => {
            let label_i = match labels.get(name) {
                None => return Err(AssemblyErrorVariant::NoLabelFound { name: name.to_owned() }),
                Some(idx) => *idx as i64,
            };

            let offset = label_i - line_i as i64 + 1;

            match offset {
                o if o < i16::MIN as i64 => Err(AssemblyErrorVariant::OffsetTooLarge { limit: i16::MIN as i32, required: o }),
                o if o > i16::MAX as i64 => Err(AssemblyErrorVariant::OffsetTooLarge { limit: i16::MAX as i32, required: o }),
                _ => Ok(Right(offset as i16)),
            }
        }

        Unsigned(offset) => Ok(Right(*offset as i16)),
        Signed(offset) => Ok(Right(*offset)),
        Register(reg) => Ok(Left(*reg)),
        _ => Err(AssemblyErrorVariant::ParamTypes),
    }
}

fn process_unconditional_branch(line_i: usize, params: &[Token], labels: &HashMap<String, usize>) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 1)?;
    let offset = process_branch(line_i, &params[0], labels)?;
    Ok(Instruction::Branch { offset })
}

fn process_branch_greater(line_i: usize, params: &[Token], labels: &HashMap<String, usize>) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 1)?;
    let offset = process_branch(line_i, &params[0], labels)?;
    Ok(Instruction::BranchGreater { offset })
}

fn process_branch_equal(line_i: usize, params: &[Token], labels: &HashMap<String, usize>) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 1)?;
    let offset = process_branch(line_i, &params[0], labels)?;
    Ok(Instruction::BranchEqual { offset })
}

fn process_branch_smaller(line_i: usize, params: &[Token], labels: &HashMap<String, usize>) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 1)?;
    let offset = process_branch(line_i, &params[0], labels)?;
    Ok(Instruction::BranchSmaller { offset })
}

fn process_branch_greater_equal(line_i: usize, params: &[Token], labels: &HashMap<String, usize>) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 1)?;
    let offset = process_branch(line_i, &params[0], labels)?;
    Ok(Instruction::BranchGreaterEqual { offset })
}

fn process_branch_not_equal(line_i: usize, params: &[Token], labels: &HashMap<String, usize>) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 1)?;
    let offset = process_branch(line_i, &params[0], labels)?;
    Ok(Instruction::BranchNotEqual { offset })
}

fn process_branch_smaller_equal(line_i: usize, params: &[Token], labels: &HashMap<String, usize>) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 1)?;
    let offset = process_branch(line_i, &params[0], labels)?;
    Ok(Instruction::BranchSmallerEqual { offset })
}

fn process_immediate_to_float(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 2)?;

    match (&params[0].variant, &params[1].variant) {
        (Register(_), Unsigned(imm)) if *imm > i16::MAX as u16 => {
            Err(AssemblyErrorVariant::ImmediateTooLarge {
                max: i16::MAX as u16,
                got: *imm,
            })
        }

        (Register(reg), Unsigned(imm)) => {
            Ok(Instruction::ImmediateToFloat {
                dest: *reg,
                imm: *imm as i16,
            })
        }

        (Register(reg), Signed(imm)) =>
            Ok(Instruction::ImmediateToFloat {
                dest: *reg,
                imm: *imm,
            }),
        
        _ => Err(AssemblyErrorVariant::ParamTypes),
    }
}

fn process_immediate_to_double(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    expect_param_amount(params, 2)?;

    match (&params[0].variant, &params[1].variant) {
        (Register(_), Unsigned(imm)) if *imm > i16::MAX as u16 => {
            Err(AssemblyErrorVariant::ImmediateTooLarge {
                max: i16::MAX as u16,
                got: *imm,
            })
        }

        (Register(reg), Unsigned(imm)) => {
            Ok(Instruction::ImmediateToDouble {
                dest: *reg,
                imm: *imm as i16,
            })
        }

        (Register(reg), Signed(imm)) =>
            Ok(Instruction::ImmediateToDouble {
                dest: *reg,
                imm: *imm,
            }),

        _ => Err(AssemblyErrorVariant::ParamTypes),
    }
}

fn process_conversion(params: &[Token]) -> Result<(Register, Register), AssemblyErrorVariant> {
    expect_param_amount(params, 2)?;

    match (&params[0].variant, &params[1].variant) {
        (Register(dest), Register(src)) => Ok((*dest, *src)),
        _ => Err(AssemblyErrorVariant::ParamTypes),
    }
}

fn process_int_to_float(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, src) = process_conversion(params)?;
    Ok(Instruction::IntegerToFloat { dest, src })
}

fn process_int_to_double(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, src) = process_conversion(params)?;
    Ok(Instruction::IntegerToDouble { dest, src })
}

fn process_float_to_int(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, src) = process_conversion(params)?;
    Ok(Instruction::FloatToInteger { dest, src })
}

fn process_float_to_double(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, src) = process_conversion(params)?;
    Ok(Instruction::FloatToDouble { dest, src })
}

fn process_double_to_int(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, src) = process_conversion(params)?;
    Ok(Instruction::DoubleToInteger { dest, src })
}

fn process_double_to_float(params: &[Token]) -> Result<Instruction, AssemblyErrorVariant> {
    let (dest, src) = process_conversion(params)?;
    Ok(Instruction::DoubleToFloat { dest, src })
}
