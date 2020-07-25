use crate::{
    ext,
    modules::{
        assembler,
        memory::Memory,
        registers::{DwRegisters, Flag, Registers},
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
            memory: Memory::new(),
            registers: Registers::new(),
        }
    }

    pub fn execute(&mut self, program: Vec<u8>) -> () {
        let program_memory = &mut self.memory.program[0..program.len() as u16];
        program_memory.clone_from_slice(&program[..]);

        println!("Loaded program: {:x?}", program_memory);

        while self.registers.pc as usize != program.len() {
            //        for _ in 0..u32::MAX {
            let opcode = self.get_w();
            let opcode_h = opcode >> 4; //For opcodes where higher bits its a command
            println!(
                "Current command: {}({:2X})",
                assembler::disassembler(opcode),
                opcode
            );
            //???
            match opcode {
                //ADI
                0xC6 => {
                    let value = self.get_w();
                    self.alu_add(value)
                }
                //LDA
                0x3a => {
                    let var_adress = self.get_dw();
                    self.registers.a = self.memory.program[var_adress]
                }
                //All kind of jumps
                0xC3 => self.alu_jmp(true), //JMP
                0xCA => self.alu_jmp(self.registers.get_flag(Flag::Zero)), //JZ
                0xC2 => self.alu_jmp(!self.registers.get_flag(Flag::Zero)), //JNZ
                0xF2 => self.alu_jmp(self.registers.get_flag(Flag::Sign)), //JP
                0xFA => self.alu_jmp(!self.registers.get_flag(Flag::Sign)), //JM
                0xDA => self.alu_jmp(self.registers.get_flag(Flag::Carry)), //JC
                0xD2 => self.alu_jmp(!self.registers.get_flag(Flag::Carry)), //JNC
                0xEA => self.alu_jmp(self.registers.get_flag(Flag::Parity)), //JPE
                0xE2 => self.alu_jmp(!self.registers.get_flag(Flag::Parity)), //JPO
                //CALL
                0xCD => {
                    self.stack_push(self.registers.pc + 1);
                    self.alu_jmp(true)
                }
                0xC9 => self.registers.pc = self.stack_pop(), //RET
                //MOV
                _ if opcode_h ^ 0b0100 == 0 => {
                    let value = *(self.bin_as_register(Cpu::get_second_argument(opcode)));
                    let to = self.bin_as_register(Cpu::get_first_argument(opcode));
                    *to = value;
                }
                0xE6 => {
                    let value = self.get_w();
                    self.alu_and(value)
                }
                //ANA
                _ if opcode_h ^ 0b1010 == 0 => {
                    let value = *(self.bin_as_register(Cpu::get_second_argument(opcode)));
                    self.alu_and(value);
                }
                //                0x0 => println!("NOP goted."), //NOP
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
                    let to = self.bin_as_register(Cpu::get_first_argument(opcode));
                    *to = (*to).wrapping_add(1);
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

//Stack operations
impl Cpu {
    fn stack_push(&mut self, value: u16) -> () {
        self.registers.sp += 1;
        self.memory.stack[self.registers.sp] = value;
    }

    fn stack_pop(&mut self) -> u16 {
        let ret = self.memory.stack[self.registers.sp];
        self.registers.sp -= 1;
        ret
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

    fn alu_and(&mut self, value: u8) -> () {
        self.registers.clr();

        self.registers.a &= value;

        self.registers.set_flag(Flag::ACarry, true);
        self.registers.set_flag(Flag::Sign, false); //FIXME: change it:D
        self.registers.set_flag(Flag::Zero, self.registers.a == 0);
        self.registers
            .set_flag(Flag::Parity, self.registers.a.count_ones() & 1 == 0);
        self.registers.set_flag(Flag::Carry, false);
    }
}

//Functions for read/write memory
impl Cpu {
    fn get_w(&mut self) -> u8 {
        let data = self.memory.program[self.registers.pc];
        self.registers.pc += 1;
        data
    }

    fn get_dw(&mut self) -> u16 {
        let data = ext::split_slice(&self.memory.program[self.registers.pc..self.registers.pc + 2]);
        self.registers.pc += 2;
        data
    }

    fn _get_slice(&self, start: u16, amount: u16) -> &[u8] {
        &self.memory.program[start..start + amount + 1]
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
            0b110 => unimplemented!("M register"), //&mut self.registers.m,//TODO: M – содержимое ячейки памяти, адресуемое регистровой парой HL.
            0b111 => &mut self.registers.a,
            _ => unreachable!("Register? {}", b),
        }
    }

    fn get_first_argument(op: u8) -> u8 {
        (op & 0b111000) >> 3
    }
    fn get_second_argument(op: u8) -> u8 {
        op & 0b000111
    }
}
