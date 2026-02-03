

fn main() {
    let mem:[i16;4] = [1,2,3,4];
    // counters and registers are 12-bit addressable meaning they cant point to 4096 addresses
    let index_register: i16 = 0;
    let program_counter: i16 = 0;
    
    println!("Hello, world!");
}
