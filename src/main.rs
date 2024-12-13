mod cpu;
mod constructor;

use cpu::CPU;
use constructor::{Instruction, Register::*};

fn main() {
    let instructions: Vec<Instruction> = vec![
        Instruction::Add { dest: R2, a: R0, b: R1 },
        Instruction::AddImm { dest: R2, a: R2, b: 5 },
        Instruction::BI { amount: -1 },
    ];

    let program = instructions.iter().map(|instr| instr.assemble()).collect::<Vec<_>>();

    println!("Instructions:");
    instructions.iter().for_each(|instruction| println!("{:?}", instruction));
    println!("Assembled:");
    program.iter().for_each(|instruction| println!("{:#010x}", instruction));


    let mut cpu = CPU::new();

    // Setup
    cpu.regs[0] = 1;
    cpu.regs[1] = 2;

    load_program(&mut cpu, &program);

    cpu.set_instruction_ptr(0);

    cpu.run(10);

    println!("Registers after execution: {:?}", cpu.regs);
}

fn load_program(cpu: &mut CPU, program: &Vec<u32>) {
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
