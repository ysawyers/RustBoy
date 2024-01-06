use crate::internal::core::component::CPU;
use std::fs;

pub mod internal;

pub fn main() {
    let mut core = CPU::default();
    let boot = fs::read("roms/DMG_ROM.bin").expect("File not found!");
    core.bus.mount_bootrom(boot);

    let file = fs::read("roms/BOMB.gb").expect("File not found!");
    core.bus.load_cartridge(file);

    loop {
        core.next_frame(17556, -1);
    }
}