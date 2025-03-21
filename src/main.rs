mod cpu;
mod assembler;

use assembler::assemble::assemble;
use cpu::Cpu;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file_path = match args.get(1) {
        None => {
            println!("No file specified");
            return;
        }
        Some(path) => path,
    };

    let file_content = match std::fs::read_to_string(file_path) {
        Ok(file_content) => file_content,
        Err(err) => {
            println!("Error: {:?}", err);
            std::process::exit(1);
        }
    };

    let instructions = match assemble(file_content) {
        Ok(instructions) => instructions,
        Err(err_msg) => {
            println!("{}", err_msg);
            return;
        }
    };

    println!("Assembled into {} instructions:", instructions.len());
    instructions.iter().enumerate().for_each(|(i, instruction)| println!("{}. {:#010x}", i + 1,  instruction));

    let mut cpu = Cpu::default();

    load_program(&mut cpu, &instructions);

    cpu.set_instruction_ptr(0);

    cpu.run(100);

    println!("Registers after execution: {:?}", cpu.regs);
}

fn load_program(cpu: &mut Cpu, program: &[u32]) {
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
