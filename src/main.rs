use crate::chip_8::Chip8;
use create::instruction_loader::load_rom;


mod chip_8;
mod instruction_loader;

fn main() {
    let mut chip_8 = Chip8::new(); // loads default memory configuration
    load_rom(chip_8);
    
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