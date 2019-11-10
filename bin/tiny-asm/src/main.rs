use std::{
    fs::File,
    io::{
        Read,
        Write
    }
};

use system::Instruction;

fn main() {
    let mut args = std::env::args().skip(1);

    let mut in_file = File::open(args.next().unwrap()).unwrap();
    let mut program = String::new();

    in_file.read_to_string(&mut program).unwrap();

    let mut bin_program = Vec::new();

    for token in program.split_whitespace() {
        match token {
            "nop" => bin_program.push(Instruction::NOP as u8),
            "hlt" => bin_program.push(Instruction::HLT as u8),
            _ => panic!("Invalid opcode!")
        }
    }

    let mut out_file = File::create(args.next().unwrap_or_else(|| "tiny.bin".to_string())).unwrap();

    out_file.write_all(&bin_program).unwrap();
}
