use crate::internal::core::component::CPU;
use std::fs;

pub mod internal;

pub fn main() {
    let mut core = CPU::default();

    let bytes = fs::read("roms/DMG_ROM.bin").expect("File not found!");
    core.bus.mount_bootrom(bytes);

    let poop = fs::read("roms/TETRIS.gb").expect("File not found!");
    core.bus.load_cartridge(poop);

    loop {
        core.next_frame(17556);
    }
}