use crate::internal::emulator::Gameboy;

mod internal;

fn main() {
    let gb = Gameboy::default();
    
    gb.next_frame();
}