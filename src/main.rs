mod cpu;
mod constructor;

use constructor::{Instruction, Register::*, U2};
use cpu::CPU;
use either::{Left, Right};

fn main() {
    let code = [
        Instruction::LoadImmediate { dest: R0, slice: U2::new(0).unwrap(), imm: 1},
        Instruction::LoadImmediate { dest: R1, slice: U2::new(0).unwrap(), imm: 1},
        Instruction::Add { dest: R2, a: R0, b: Left(R1) },
        Instruction::Move{dest:R0,src:R1},
        Instruction::Move{dest:R1,src:R2},
        Instruction::Branch{amount:Right(-3)}
    ];

    let program = code
        .iter()
        .map(|e| e.assemble())
        .collect::<Vec<u32>>();

    println!("Code:");
    code.iter().for_each(|i| println!("{:?}", i));
    println!("Instructions:");
    program.iter().for_each(|instruction| println!("{:#010x}", instruction));

    let mut cpu = CPU::default();

    load_program(&mut cpu, &program);

    cpu.set_instruction_ptr(0);

    cpu.run(20);

    println!("Registers after execution: {:?}", cpu.regs);
}

fn load_program(cpu: &mut CPU, program: &[u32]) {
    let mut mem = 0;

    // Ensure the program fits into memory
    assert!(program.len() * 4 <= cpu.memory.len(), "Program is too large to fit in memory!");

    for &instruction in program.iter() {
        let (b1, b2, b3, b4) = encode_instruction(instruction);

        // Write the instruction bytes to memory in order
        cpu.memory[mem] = b1;
        cpu.memory[mem + 1] = b2;
        cpu.memory[mem + 2] = b3;
        cpu.memory[mem + 3] = b4;

        mem += 4; // Advance by 4 bytes
    }

    fn encode_instruction(instruction: u32) -> (u8, u8, u8, u8) {
        (
            ((instruction & 0xFF000000) >> 24) as u8,
            ((instruction & 0x00FF0000) >> 16) as u8,
            ((instruction & 0x0000FF00) >> 8) as u8,
            (instruction & 0x000000FF) as u8
        )
    }
}
