
// mod memory;
mod value;
mod ast;

use std::mem::size_of;

fn main() {
    println!("Hello, world!");
    println!("Value length: {}", size_of::<value::LyshValue>());
}
