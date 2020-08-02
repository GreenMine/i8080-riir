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

/* Macro for command with registers and immediate data adressing
 * If command have a two types: register and immediate data, it can be implement one time:
 * For example MOV and MVI:
 * MOV: 01TTTFFF
 * MVI: 00TTT110, where
 * TTT - register where data will be moves
 * FFF - register from which data will be moves
 * Means, if its immediate type of command, it xor second bite of command, and set "FFF" to 110
 */
macro_rules! check_cmd {
    ($f:stmt, $cmd:expr, $opcode:ident, $needed_opcode:literal, $is_a:literal, $self:ident, $to:ident, $from:ident) => (
        if $cmd ^ $needed_opcode == 0 {
            let $from = *$self.registers.bin_as_register(Cpu::get_second_argument($opcode));
            let $to: &mut u8 = if $is_a {&mut $self.registers.a} else {$self.registers.bin_as_register(Cpu::get_first_argument($opcode))};
            $f
            continue;
        }
		if $cmd ^ ($needed_opcode ^ 0b0100) == 0 && ($opcode & 0b111) == 0b110 {
            let $from = $self.get_w();
            let $to: &mut u8 = if $is_a {&mut $self.registers.a} else {$self.registers.bin_as_register(Cpu::get_first_argument($opcode))};
            $f
            continue;
        }
    )
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
            let opcode_l = opcode & 0b0111; //For opcodes where command in lower bits.
			#[cfg(debug_assertions)] {
				println!("Processor data: {:?}", self);
				println!(
					"Current command: {}({:2X})",
					assembler::disassembler(opcode),
					opcode
				);
			}

            //SUB and SUI
            check_cmd!(self.alu_sub(value), opcode_h, opcode, 0b1001, true, self, _to, value);
            //CMP and CPI
            check_cmd!({
                    let temp = self.registers.a;
                    self.alu_sub(value);
                    self.registers.a = temp;
                },
				opcode_h,opcode, 0b1011, true, self, _to, value);
            //ANA and ANI
            check_cmd!(self.alu_and(value), opcode_h, opcode, 0b1010, true, self, _to, value);
            //ADD and ADI
            check_cmd!(self.alu_add(value), opcode_h, opcode, 0b1000, true, self, _to, value);
            //MOV and MVI
            check_cmd!(*to = value, opcode_h & 0b0, opcode, 0b0100, false, self, to, value);
            //???
            match opcode {
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
                0x76 => return,                               //HLT
                //NOTE: PUSH and POP have a special 11 bit in height half of opcode
                //PUSH
                _ if ((opcode_h >> 2) ^ 0b11 == 0) && (opcode_l ^ 0b101 == 0) => {
                    let value = self.registers.get_dw_reg(Cpu::get_first_argument(opcode));
                    self.stack_push(value)
                }
                //POP
                _ if ((opcode_h >> 2) ^ 0b11 == 0) && (opcode_l ^ 0b001 == 0) => {
                    let value = self.stack_pop();
                    self.registers
                        .set_dw_reg(Cpu::get_first_argument(opcode), value)
                }
                0x2F => self.registers.a = !self.registers.a, //CMA
                0x3F => self
                    .registers
                    .set_flag(Flag::Carry, !self.registers.get_flag(Flag::Carry)), //CMC
                0x0 => println!("NOP goted."),                //NOP
                //INR
                _ if opcode_l ^ 0b100 == 0 => {
                    let to = self
                        .registers
                        .bin_as_register(Cpu::get_first_argument(opcode));
                    *to = (*to).wrapping_add(1);
                }
                //DCR
                _ if opcode_l ^ 0b101 == 0 => {
                    let to = self
                        .registers
                        .bin_as_register(Cpu::get_first_argument(opcode));
                    *to = (*to).wrapping_sub(1);
                }
                _ => unreachable!("{:x}", opcode),
            }
        }
		println!("Result: {:?}", self);
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
    fn get_first_argument(op: u8) -> u8 {
        (op & 0b111000) >> 3
    }
    fn get_second_argument(op: u8) -> u8 {
        op & 0b000111
    }
}
