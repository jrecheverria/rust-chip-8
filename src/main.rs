use crate::{instruction_loader::load_instructions, memory::Memory};

mod memory;
mod instruction_loader;

fn main() {
    let memory = Memory::new(); // loads default memory configuration
    load_instructions();
    
    // Run main CPU loop 
    loop {
        fetch();
        decode();
        execute();
    }
}

fn fetch() {}

fn decode() {}

fn execute() {}