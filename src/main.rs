#![feature(const_fn_fn_ptr_basics)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![no_main]
#![no_std]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::display::interface::DrawPixel;

mod bsp;
mod console;
mod cpu;
mod driver;
mod panic_wait;
mod print;
mod synchronization;
mod display;

unsafe fn kernel_init() -> ! {
    use driver::interface::DriverManager;
    for i in bsp::driver::driver_manager().all_device_drivers().iter() {
        if let Err(x) = i.init() {
            panic!("Error loading driver: {}: {}", i.compatible(), x);
        }
    }
    bsp::driver::driver_manager().post_device_driver_init();
    kernel_main()
}

fn kernel_main() -> ! {
    use bsp::console::console;
    use console::interface::All;
    use driver::interface::DriverManager;

    use display::Color::*;
    let colors = [Black, Blue, Green, Cyan, Red, Magenta,
        Brown, LightGray, DarkGray, LightBlue, LightGreen, LightCyan,
        Pink, Yellow, White];
    let mut pos = 100;
    for color in colors {
        for i in 0..100 {
            bsp::display::display().draw_pixel(pos + i, 100, color as u32);
            pos += 1;
        }
    }

    println!(
        "[0] {} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!("[1] Booting on: {}", bsp::board_name());
    println!("[2] Drivers loaded");
    for (i, driver) in bsp::driver::driver_manager()
        .all_device_drivers()
        .iter()
        .enumerate()
    {
        println!("    {}. {}", i + 1, driver.compatible());
    }
    println!(
        "[3] Chars written: {}",
        bsp::console::console().chars_written()
    );
    println!("[4] Echoing input now");
    
    console().clear_rx();
    loop {
        let c = bsp::console::console().read_char();
        bsp::console::console().write_char(c);
    }
}
