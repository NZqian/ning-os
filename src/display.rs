pub mod interface {
    use core::fmt;

    pub trait DrawPixel {
        fn draw_pixel(&self, x: u32, y: u32, attr: u32);
    }

    pub trait DrawText {
        fn write_char(&self, x: u32, y: u32, c: char);
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
    }

    pub trait DrawShape {
        fn draw_line(&self, x1: u32, y1: u32, x2: u32, y2: u32, color: u32);
        fn draw_rect(&self, x: u32, y: u32, width: u32, height: u32, color: u32);
    }

    pub trait All = DrawPixel + DrawText + DrawShape;
}

/*
struct Color {}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> u32 {
        let mut c:u32 = (r as u32) << 16;
        c = c + ((g as u32) << 8);
        c = c + (b as u32);
        c
    }
}
*/
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Color {
    Black = 0x000000,
    Blue = 0x0000AA,
    Green = 0x00AA00,
    Cyan = 0x00AAAA,
    Red = 0xAA0000,
    Magenta = 0xAA00AA,
    Brown = 0xAA5500,
    LightGray = 0xAAAAAA,
    DarkGray = 0x555555,
    LightBlue = 0x5555FF,
    LightGreen = 0x55FF55,
    LightCyan = 0x55FFFF,
    LightRed = 0xFF5555,
    Pink = 0xFF55FF,
    Yellow = 0xFFFF55,
    White = 0xFFFFFF,
}
