mod cpu;
mod constructor;

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



    let mut cpu = cpu::CPU::new();

    // Setup
    cpu.regs[0] = 1;
    cpu.regs[1] = 2;

    cpu.run(&program);

    println!("Registers after execution: {:?}", cpu.regs);
}
