#![allow(dead_code)]

#[rustfmt::skip]
pub mod mbox_enum {
    pub const MBOX_RESPONSE      :u32 = 0x8000_0000;
    pub const MBOX_FULL          :u32 = 0x8000_0000;
    pub const MBOX_EMPTY         :u32 = 0x4000_0000;

    pub const MBOX_REQUEST       :u32 = 0;
    pub const MBOX_CH_POWER      :u32 = 0;
    pub const MBOX_CH_FB         :u32 = 1;
    pub const MBOX_CH_VUART      :u32 = 2;
    pub const MBOX_CH_VCHIQ      :u32 = 3;
    pub const MBOX_CH_LEDS       :u32 = 4;
    pub const MBOX_CH_BTNS       :u32 = 5;
    pub const MBOX_CH_TOUCH      :u32 = 6;
    pub const MBOX_CH_COUNT      :u32 = 7;
    pub const MBOX_CH_PROP       :u32 = 8;

    pub const MBOX_TAG_SETPOWER  :u32 = 0x28001;
    pub const MBOX_TAG_SETCLKRATE:u32 = 0x38002;
    pub const MBOX_TAG_SETPHYWH  :u32 = 0x48003;
    pub const MBOX_TAG_SETVIRTWH :u32 = 0x48004;
    pub const MBOX_TAG_SETVIRTOFF:u32 = 0x48009;
    pub const MBOX_TAG_SETDEPTH  :u32 = 0x48005;
    pub const MBOX_TAG_SETPXLORDR:u32 = 0x48006;
    pub const MBOX_TAG_GETFB     :u32 = 0x40001;
    pub const MBOX_TAG_GETPITCH  :u32 = 0x40008;
    pub const MBOX_TAG_LAST      :u32 = 0      ;
}
use mbox_enum::*;

use crate::{bsp::device_driver::common::MMIODerefWrapper, println};

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_structs,
    registers::{ReadOnly, WriteOnly},
};

register_structs! {
    #[allow(non_snake_case)]
    VideoCoreRegisters {
        (0x00 => mbox_read: ReadOnly<u32>),
        (0x04 => _reserved1),
        (0x10 => mbox_poll: ReadOnly<u32>),
        (0x14 => mbox_sender: ReadOnly<u32>),
        (0x18 => mbox_status: ReadOnly<u32>),
        (0x20 => mbox_write: WriteOnly<u32>),
        (0xE8 => @END),
    }
}

type Registers = MMIODerefWrapper<VideoCoreRegisters>;

pub struct MBox {
    pub message: MBoxMessage,
    registers: Registers,
    video_core_addr: usize,
}

impl MBox {
    pub const unsafe fn new(mmio_video_core_addr: usize) -> Self {
        Self {
            message: MBoxMessage::new(),
            registers: Registers::new(mmio_video_core_addr),
            video_core_addr: mmio_video_core_addr
        }
    }
    pub fn mbox_call(&mut self, channel: u32) -> bool {
        let message_struct_addr: u32 = &mut self.message as *mut MBoxMessage as u32;
        let r: u32 = (&mut self.message as *mut MBoxMessage as u32) | (channel & 0xF);

        loop {
            let status = self.registers.mbox_status.get();
            if status & MBOX_FULL == 0 {
                break
            }
            //println!("waiting, status: {}", status);
        }
        //self.registers.mbox_write.set(r);
        unsafe {
            *(((self.video_core_addr as u32) + 0x20) as *mut u32) = r;
        }
        loop {
            loop {
                let status = self.registers.mbox_status.get();
                println!("cur status: {}", status);
                if status & MBOX_EMPTY == 0 {
                    break;
                }
            }
            println!("fuck! not empty");
            if r == self.registers.mbox_read.get() {
                return self.message.message[1] == MBOX_RESPONSE;
            }
        }
    }
}

#[repr(C, align(16))]
pub struct MBoxMessage {
    pub message: [u32; 35],
}

impl MBoxMessage {
    pub const fn new() -> Self {
        let m: [u32; 35] = [
            35 * 4,
            MBOX_REQUEST,
            MBOX_TAG_SETPHYWH, // Tag identifier
            8,                 // Value size in bytes
            0,
            1920, // Value(width)
            1080, // Value(height)
            MBOX_TAG_SETVIRTWH,
            8,
            8,
            1920,
            1080,
            MBOX_TAG_SETVIRTOFF,
            8,
            8,
            0, // Value(x)
            0, // Value(y)
            MBOX_TAG_SETDEPTH,
            4,
            4,
            32, // Bits per pixel
            MBOX_TAG_SETPXLORDR,
            4,
            4,
            1, // RGB
            MBOX_TAG_GETFB,
            8,
            8,
            4096, // FrameBufferInfo.pointer
            0,    // FrameBufferInfo.size
            MBOX_TAG_GETPITCH,
            4,
            4,
            0, // Bytes per line
            MBOX_TAG_LAST,
        ];
        Self { message: m }
    }
}
