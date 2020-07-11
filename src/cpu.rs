use crate::{
    ext,
    modules::{/*assembler,*/ memory, registers},
};

const PROGRAM_MEMORY_START: u16 = 0x0100;

#[derive(Debug)]
pub struct Cpu {
    memory: memory::Memory,
    registers: registers::Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            memory: memory::Memory::new(65536),
            registers: registers::Registers::new(),
        }
    }

    pub fn execute(&mut self, program: Vec<u8>) -> () {
        self.memory[PROGRAM_MEMORY_START..program.len() as u16 + PROGRAM_MEMORY_START]
            .clone_from_slice(&program[..]);
        self.registers.pc = PROGRAM_MEMORY_START;

        println!(
            "Loaded program: {:x?}",
            self.memory[PROGRAM_MEMORY_START..program.len() as u16 + PROGRAM_MEMORY_START]
        );
        //        while (self.registers.pc) as usize != program.len() + 1 {
        for _ in 0..16 {
            //            println!(
            //                "Current command: {}",
            //                assembler::disassembler(program[self.registers.pc as usize])
            //            );
            println!("Processor data: {:?}", self);
            match self.get_w() {
                0x78 => self.registers.a = self.registers.b,
                0x47 => self.registers.b = self.registers.a,
                0x3c => self.registers.a += 1,
                0x3e => self.registers.a = self.get_w(),
                0xc3 => {
                    self.registers.pc = PROGRAM_MEMORY_START + self.get_dw();
                    continue;
                }
                _ => unreachable!(),
            }
        }
    }
}

//functions for read/write memory
impl Cpu {
    fn get_w(&mut self) -> u8 {
        let data = self.memory[self.registers.pc];
        self.registers.pc += 1;
        data
    }

    fn get_dw(&mut self) -> u16 {
        let data = ext::split_slice(&self.memory[self.registers.pc..self.registers.pc + 2]);
        self.registers.pc += 1;
        data
    }
}
