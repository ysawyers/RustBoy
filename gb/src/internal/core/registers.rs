use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Register {
    A, B, C, D, E, H, L, F
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Flag { Z, N, H, C }

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: u8
}

impl Index<Register> for Registers {
    type Output = u8;

    fn index(&self, register: Register) -> &Self::Output {
        match register {
            Register::A => &self.a,
            Register::B => &self.b,
            Register::C => &self.c,
            Register::D => &self.d,
            Register::E => &self.e,
            Register::F => &self.f,
            Register::L => &self.l,
            Register::H => &self.h 
        }
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, register: Register) -> &mut Self::Output {
        match register {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
            Register::C => &mut self.c,
            Register::D => &mut self.d,
            Register::E => &mut self.e,
            Register::F => &mut self.f,
            Register::L => &mut self.l,
            Register::H => &mut self.h 
        }
    }
}

impl Registers {
    pub fn get_flag(&self, flag: Flag) -> u8 {
        match flag {
            Flag::Z => (self.f >> 7) & 0x1,
            Flag::N => (self.f >> 6) & 0x1,
            Flag::H => (self.f >> 5) & 0x1,
            Flag::C => (self.f >> 4) & 0x1
        }
    }

    pub fn set_flag(&mut self, flag: Flag, is_set: bool) {
        match flag {
            Flag::Z => {
                if is_set {
                    self.f |= 0b10000000
                } else {
                    self.f &= 0b01111111
                }
            },
            Flag::N => {
                if is_set {
                    self.f |= 0b01000000
                } else {
                    self.f &= 0b10111111
                }
            },
            Flag::H => {
                if is_set {
                    self.f |= 0b00100000
                } else {
                    self.f &= 0b11011111
                }
            },
            Flag::C => {
                if is_set {
                    self.f |= 0b00010000
                } else {
                    self.f &= 0b11101111
                }
            }
        }
    }

    pub fn get_de(&self) -> u16 { ((self.d as u16) << 8) | (self.e as u16) }

    pub fn get_hl(&self) -> u16 { ((self.h as u16) << 8) | (self.l as u16) }

    pub fn get_bc(&self) -> u16 { ((self.b as u16) << 8) | (self.c as u16) }

    pub fn set_hl(&mut self, val: u16) {
        self.h = ((val & 0xFF00) >> 8) as u8;
        self.l = (val & 0x00FF) as u8;
    }

    pub fn set_bc(&mut self, val: u16) {
        self.b = ((val & 0xFF00) >> 8) as u8;
        self.c = (val & 0x00FF) as u8;
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            a: 0x0,
            b: 0x0,
            c: 0x0,
            d: 0x0,
            e: 0x0,
            h: 0x0,
            l: 0x0,
            f: 0x0
        }
    }
}