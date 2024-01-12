// NRxy: each index represents a specific channel (x) for the register ID (y) within the channel.
pub struct APU {
    nr0: [u8; 5], // channel-specific feature (if present)
    nr1: [u8; 5], // controls the length timer
    nr2: [u8; 5], // controls the volume and envelope
    nr3: [u8; 5], // controls the period (maybe only partially?)
    nr4: [u8; 5]  // has the channel's trigger and length timer enable bits, as well as any leftover bits of period
}

impl APU {
    pub fn write_registers(&mut self, addr: u16, val: u8) {
        match addr {    
            0xFF10 => self.nr0[0] = val, // ???????
            0xFF11 => self.nr1[0] = val,
            0xFF12 => self.nr2[0] = val,

            0xFF24 => self.nr0[4] = val,
            0xFF25 => self.nr1[4] = val,
            0xFF26 => self.nr2[4] = val & 0x80, // NR52: lower 4 bits are read only so masked to prevent writes.
            
            _ => unreachable!()
        }
    }

    pub fn read_registers(&mut self, addr: u16) -> u8 {
        match addr {
            0xFF10 => self.nr0[0], // Channel 1 sweep ???????
            0xFF11 => self.nr1[0] >> 6, // Channel 1 length timer & duty cycle
            0xFF12 => self.nr2[0], // Channel 1 volume & envelope

            0xFF24 => self.nr0[4], // Master volume & VIN panning
            0xFF25 => self.nr1[4], // sound panning
            0xFF26 => self.nr2[4], // audio master control

            _ => unreachable!()
        }
    }
}

impl Default for APU {
    fn default() -> Self {
        Self {
            nr0: [0x0; 5],
            nr1: [0x0; 5],
            nr2: [0x0; 5],
            nr3: [0x0; 5],
            nr4: [0x0; 5]
        }
    }
}