extern crate winapi;
extern crate libloading;
extern crate owning_ref;

// mod memory;
mod value;
mod ast;
mod exec_engine;

use std::mem::size_of;
use crate::value::{Ref, LyshValue, RowValueImage};
use crate::exec_engine::context::*;


fn boot_check() {
    debug_assert_eq!(size_of::<value::LyshValue>(), 16)
}

fn main() {
    boot_check();
    println!("Hello, world!");
    println!("Value length: {}", size_of::<value::LyshValue>());
    println!("LNI length: {}", size_of::<value::LNI>());
    println!("{:?}", RowValueImage::from(LyshValue::Nil));
    println!("{:?}", RowValueImage::from(LyshValue::Uint(2)));
}
