#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![no_main]
#![no_std]

use crate::console::interface::Statistics;

mod bsp;
mod console;
mod cpu;
mod panic_wait;
mod print;
mod synchronization;

unsafe fn kernel_init() -> ! {
    println!("[0] Hello from Rust!");
    println!(
        "[1] Chars written: {}",
        bsp::console::console().chars_written()
    );
    panic!("Stopping here.")
}
