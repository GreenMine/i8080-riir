use crate::{
    ext,
    modules::{
        assembler,
        memory::Memory,
        registers::{Flag, Registers},
    },
};

#[derive(Debug)]
pub struct Cpu {
    pub(crate) memory: Memory,
    pub(crate) registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(65536),
            registers: Registers::new(),
        }
    }

    pub fn execute(&mut self, program: Vec<u8>) -> () {
        self.memory[0..program.len() as u16].clone_from_slice(&program[..]); // maybe set program memory space to 0x100???

        println!(
            "Loaded program: {:x?}",
            &self.memory[0..program.len() as u16]
        );

        while self.registers.pc as usize != program.len() {
            //        for _ in 0..16 {
            let opcode = self.get_w();
            let opcode_h = opcode >> 4; //For opcodes where higher bits its a command
            println!(
                "Current command: {}({:2X})",
                assembler::disassembler(opcode),
                opcode
            );
            //???
            match opcode {
                //LDA
                0x3a => {
                    let var_adress = self.get_dw();
                    self.registers.a = self.memory[var_adress]
                }
                //All kind of jumps
                _ if (opcode_h >> 2) ^ 0b11 == 0 => {
                    println!("{:b}", opcode & 0b11_1111);
                    match opcode & 0b11_1111 {
                        0b11 => self.alu_jmp(true),                                       //JMP
                        0b1010 => self.alu_jmp(self.registers.get_flag(Flag::Zero)),      //JZ
                        0b10 => self.alu_jmp(!self.registers.get_flag(Flag::Zero)),       //JNZ
                        0b110010 => self.alu_jmp(self.registers.get_flag(Flag::Sign)),    //JP
                        0b111010 => self.alu_jmp(!self.registers.get_flag(Flag::Sign)),   //JM
                        0b11010 => self.alu_jmp(self.registers.get_flag(Flag::Carry)),    //JC
                        0b10010 => self.alu_jmp(!self.registers.get_flag(Flag::Carry)),   //JNC
                        0b101010 => self.alu_jmp(self.registers.get_flag(Flag::Parity)),  //JPE
                        0b100010 => self.alu_jmp(!self.registers.get_flag(Flag::Parity)), //JPO
                        _ => unreachable!(),
                    }
                }
                //MOV
                _ if opcode_h ^ 0b0100 == 0 => {
                    let value = *(self.bin_as_register(Cpu::get_second_argument(opcode)));
                    let to = self.bin_as_register(Cpu::get_first_argument(opcode));
                    *to = value;
                }
                //ADD
                _ if opcode_h ^ 0b1000 == 0 => {
                    let value = *(self.bin_as_register(Cpu::get_second_argument(opcode)));
                    self.alu_add(value)
                }
                //SUB
                _ if opcode_h ^ 0b1001 == 0 => {
                    let value = *(self.bin_as_register(Cpu::get_second_argument(opcode)));
                    self.alu_sub(value)
                }
                //CMP
                _ if opcode_h ^ 0b1011 == 0 => {
                    //real opcode is 0b10111, but no matter
                    let temp = self.registers.a;
                    let value = *(self.bin_as_register(Cpu::get_second_argument(opcode)));
                    self.alu_sub(value);
                    self.registers.a = temp;
                }
                //mask 0b0111 for opcodes where command in lower bits.
                //INR
                _ if (opcode & 0b0111) ^ 0b0100 == 0 => {
                    println!("inr??");
                    let to = self.bin_as_register(Cpu::get_first_argument(opcode));
                    *to += 1;
                }
                //MVI
                _ if (opcode & 0b0111) ^ 0b110 == 0 => {
                    let value = self.get_w();
                    let to = self.bin_as_register(Cpu::get_first_argument(opcode));
                    *to = value;
                }

                _ => unreachable!("{:x}", opcode),
            }
            println!("Processor data: {:?}", self);
        }
    }
}

//ALU Operations
impl Cpu {
    fn alu_add(&mut self, value: u8) {
        self.registers.clr();
        self.registers
            .set_flag(Flag::ACarry, (self.registers.a & 0xf) + (value & 0xf) > 0xf);

        self.registers.a = self.registers.a.wrapping_add(value);

        self.registers.set_flag(Flag::Sign, false); //FIXME: change it:D
        self.registers.set_flag(Flag::Zero, self.registers.a == 0);
        self.registers
            .set_flag(Flag::Parity, self.registers.a.count_ones() & 1 == 0);
        self.registers
            .set_flag(Flag::Carry, self.registers.a < value);
    }

    fn alu_sub(&mut self, value: u8) {
        self.registers.clr();
        self.registers.set_flag(Flag::ACarry, false); //(self.registers.a & 0xf) + (value & 0xf) > 0xf

        let temp = self.registers.a;
        self.registers.a = self.registers.a.wrapping_sub(value);

        self.registers.set_flag(Flag::Sign, false); //FIXME: change it:D
        self.registers.set_flag(Flag::Zero, self.registers.a == 0);
        self.registers
            .set_flag(Flag::Parity, self.registers.a.count_ones() & 1 == 0);
        self.registers
            .set_flag(Flag::Carry, self.registers.a > temp);
    }

    fn alu_jmp(&mut self, exp: bool) {
        let to_adress = self.get_dw();
        if exp {
            self.registers.pc = to_adress;
        }
    }
}

//Functions for read/write memory
impl Cpu {
    fn get_w(&mut self) -> u8 {
        let data = self.memory[self.registers.pc];
        self.registers.pc += 1;
        data
    }

    fn get_dw(&mut self) -> u16 {
        let data = ext::split_slice(&self.memory[self.registers.pc..self.registers.pc + 2]);
        self.registers.pc += 2;
        data
    }

    fn _get_slice(&self, start: u16, amount: u16) -> &[u8] {
        &self.memory[start..start + amount + 1]
    }
}

//Binary as register
impl Cpu {
    fn bin_as_register(&mut self, b: u8) -> &mut u8 {
        match b {
            0b000 => &mut self.registers.b,
            0b001 => &mut self.registers.c,
            0b010 => &mut self.registers.d,
            0b011 => &mut self.registers.e,
            0b100 => &mut self.registers.h,
            0b101 => &mut self.registers.l,
            0b110 => unimplemented!(), //&mut self.registers.m,//TODO: M – содержимое ячейки памяти, адресуемое регистровой парой L .
            0b111 => &mut self.registers.a,
            _ => unreachable!(),
        }
    }

    fn get_first_argument(op: u8) -> u8 {
        (op & 0b111000) >> 3
    }
    fn get_second_argument(op: u8) -> u8 {
        op & 0b000111
    }
}
