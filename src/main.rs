pub mod memory;

fn main() {
    let index_register: i16 = 0; // points to locations in memory
    let program_counter: i16 = 0; // points to current instruction in memoru
    let stack:Vec<i16> = Vec::new();
    
    println!("Hello, world!");
}

fn load_memory() {}

fn fetch() {}

fn decode() {}

fn execute {}