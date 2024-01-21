// use std::fs;

// use crate::internal::core::component::CPU;

// pub mod internal;

// pub fn u32_to_little_endian(val: u32) -> [u8; 4] {
//     [(val & 0xFF) as u8, ((val & 0xFF00) >> 8) as u8, ((val & 0xFF0000) >> 16) as u8, ((val & 0xFF000000) >> 24) as u8]
// }

// fn main() {
//     let mut core = CPU::default();

//     let bytes = fs::read("roms/DMG_ROM.bin").expect("File not found!");
//     core.bus.mount_bootrom(bytes);

//     let cart = fs::read("roms/DONKEY.gb").expect("File nto found!");
//     core.bus.load_cartridge(cart);

//     loop {
//         core.next_frame(-1);
//     }
// }


fn main() {}