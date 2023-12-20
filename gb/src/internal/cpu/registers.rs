pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: u8,
}

impl Registers {
    fn read_af(&self) -> u16 {
        return (self.a as u16) << 8 | self.f as u16
    }

    fn read_bc(&self) -> u16 {
        return (self.b as u16) << 8 | self.c as u16
    }

    fn read_de(&self) -> u16 {
        return (self.d as u16) << 8 | self.e as u16
    }

    fn read_hl(&self) -> u16 {
        return (self.h as u16) << 8 | self.l as u16
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