
// mod memory;
mod value;
mod ast;

use std::mem::size_of;

fn boot_check() {
    debug_assert_eq!(size_of::<value::LyshValue>(), 16)
}

fn main() {
    boot_check();
    println!("Hello, world!");
    println!("Value length: {}", size_of::<value::LyshValue>());
}
