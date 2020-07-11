//TODO: get it back
//pub struct Registers {
//    pub psw: u16, //A and flags
//    pub bc: u16,  //B and C
//    pub de: u16,  //D and E
//    pub hl: u16,  //H and L
//    pub pc: u16,  //Inctruction pointer
//    pub sp: u16,  //Stack pointer
//}

#[derive(Debug)]
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
}
