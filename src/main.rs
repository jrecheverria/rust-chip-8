use crate::chip_8::Chip8;
use crate::instruction_manager::load_rom;


mod chip_8;
mod constants;
mod utility;

fn main() {
    let mut chip8 = Chip8::new(); // loads default memory configuration
    load_rom();
    
    loop {
        fetch();
        decode();
        execute();
    }
}

fn fetch() {}

fn decode() {}

fn execute() {}