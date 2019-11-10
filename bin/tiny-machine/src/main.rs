use std::{
    io::Read
};
use std::fs::File;

mod executor;
use executor::System;

fn main() {
    let mut args = std::env::args().skip(1);

    let file = File::open(args.next().unwrap()).unwrap();
    let memory_size: usize = args.next().map(|x| x.parse().unwrap()).unwrap_or(1024);
    let mut memory = Vec::with_capacity(memory_size);

    // Zero out the first 256 bits
    memory.extend_from_slice(&[0; 256]);
    // Load the initial program
    memory.extend(file.bytes().map(|x| x.unwrap()));
    // Zero out the rest
    for _ in 0..(memory_size - memory.len()) {
        memory.push(0);
    }

    // Create the machine
    let mut system = System {
        memory,
        program_counter: 0x100
    };

    // Step through execution
    loop {
        if !system.step() {
            break;
        }
    }
}
