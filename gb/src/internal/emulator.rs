use crate::internal::cpu::core::CPU;

const CYCLES_PER_FRAME: i32 = 69905;

pub struct Gameboy {
    cpu: CPU
}

impl Gameboy {
    pub fn next_frame(&self) {
        for _ in 0..CYCLES_PER_FRAME {
            self.cpu.step()
        }
    }
}

impl Default for Gameboy {
    fn default() -> Self {
        Self {
            cpu: CPU::default()
        }
    }
}