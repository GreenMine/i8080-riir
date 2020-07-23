use std::fmt;

//TODO: get it back
//pub struct Registers {
//    pub psw: u16, //A and flags
//    pub bc: u16,  //B and C
//    pub de: u16,  //D and E
//    pub hl: u16,  //H and L
//    pub pc: u16,  //Inctruction pointer
//    pub sp: u16,  //Stack pointer
//}

pub struct Registers {
    pub a: u8,
    pub f: u8, // The F register is indirectly accessible by the programer.
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16, //Inctruction pointer
    pub sp: u16, //Stack pointer
}

pub enum Flag {
    Sign = 7,
    Zero = 6,
    ACarry = 4,
    Parity = 2,
    Carry = 0,
}

pub enum DwRegisters {
    PSW,
    BC,
    DE,
    HL,
}
impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
        }
    }

    pub fn get_dw_reg(&mut self, reg: DwRegisters) -> u16 {
        let dw = self.reg_as_link(reg);
        ((*dw.0 as u16) << 8) | *dw.1 as u16
    }

    pub fn set_dw_reg(&mut self, reg: DwRegisters, value: u16) -> () {
        let dw = self.reg_as_link(reg);
        *dw.0 = (value >> 8) as u8;
        *dw.1 = (value & 0xFF) as u8;
    }

    fn reg_as_link(&mut self, reg: DwRegisters) -> (&mut u8, &mut u8) {
        match reg {
            DwRegisters::PSW => (&mut self.a, &mut self.f),
            DwRegisters::BC => (&mut self.b, &mut self.c),
            DwRegisters::DE => (&mut self.d, &mut self.e),
            DwRegisters::HL => (&mut self.h, &mut self.l),
        }
    }
}

//implementation for flags(F register)
impl Registers {
    pub fn set_flag(&mut self, flag: Flag, value: bool) -> () {
        self.f |= (value as u8) << flag as u8;
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        (self.f >> (flag as u8) & 1) != 0
    }

    pub fn clr(&mut self) {
        self.f = 0;
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "A: {:X}, F:[S: {}, Z: {}, AC: {}, P: {}, C: {}], B: {:X}, C: {:X}, D: {:X}, E: {:X}, H: {:X}, L: {:X}, PC: {}, SP: {}", self.a,
			self.get_flag(Flag::Sign), self.get_flag(Flag::Zero), self.get_flag(Flag::ACarry), self.get_flag(Flag::Parity), self.get_flag(Flag::Carry), self.b, self.c, self.d, self.e, self.h, self.l, self.pc, self.sp)
    }
}
