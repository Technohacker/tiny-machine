use byteorder::ByteOrder;
use num_traits::FromPrimitive;

use system::Instruction;

pub struct System {
    pub memory: Vec<u8>,
    pub program_counter: u32
}

impl System {
    pub fn step(&mut self) -> bool {
        let instruction = self.memory[self.program_counter as usize];
        let should_halt = match Instruction::from_u8(instruction).unwrap() {
            Instruction::NOP => true,
            Instruction::HLT => false,
            Instruction::JMP => {
                let jump_target = &self.memory[(self.program_counter + 1) as usize..(self.program_counter + 5) as usize];
                self.program_counter = byteorder::LittleEndian::read_u32(jump_target);
                true
            }
        };
        self.program_counter += 1;

        should_halt
    }
}
