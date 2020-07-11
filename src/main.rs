use std::fs::File;
use std::io::prelude::*;

mod cpu;
mod ext;
mod modules;

use cpu::Cpu;
//use modules::memory::Memory;

fn main() {
    //    let memory = Memory::new(65536);
    //    println!("{:?}", memory.get_memory(0, 256))
    let mut processor = Cpu::new();
    let mut file = File::open("data.bin").unwrap();
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data).unwrap();
    processor.execute(data);
    //    for _ in 0..3 {
    //        file.read(std::slice::from_mut(&mut data)).unwrap();
    //        processor.execute(data);
    //        println!("{}", modules::assembler::disassembler(data));
    //    }
    //    println!("{:?}", processor);
}
