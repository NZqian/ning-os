#![allow(dead_code)]
use super::mailbox;
use crate::{
    bsp::device_driver::mailbox::mbox_enum,
    display::interface::{DrawPixel, DrawText, DrawShape},
    driver, println,
    synchronization::{interface::Mutex, NullLock},
};

pub struct FrameBufferInner {
    width: u32,
    height: u32,
    pitch: u32,
    isrgb: bool,
    fb: usize,
    vc_addr: usize,
}

pub struct FrameBuffer {
    inner: NullLock<FrameBufferInner>,
}

impl FrameBufferInner {
    pub const fn new(mmio_video_core_addr: usize) -> Self {
        Self {
            width: 1920,
            height: 1080,
            pitch: 1920,
            isrgb: true,
            fb: 0,
            vc_addr: mmio_video_core_addr,
        }
    }
    pub fn init(&mut self) {
        let mut mbox: mailbox::MBox;
        unsafe {
            mbox = mailbox::MBox::new(self.vc_addr);
        }
        mbox.mbox_call(mbox_enum::MBOX_CH_PROP);

        if mbox.message.message[20] == 32 && mbox.message.message[28] != 0 {
            self.fb = mbox.message.message[28] as usize & 0x3FFFFFFF;
            self.width = mbox.message.message[10];
            self.height = mbox.message.message[11];
            self.pitch = mbox.message.message[33];
            self.isrgb = mbox.message.message[24] != 0;
        } else {
            println!("fuck! mbox message error");
        }
        println!("width: {}", self.width);
        println!("height: {}", self.height);
        println!("pitch: {}", self.pitch);
        println!("is_rgb: {}", self.isrgb);
        println!("fb: {}", self.fb);
    }
}

impl FrameBuffer {
    pub const fn new(mmio_video_core_addr: usize) -> Self {
        Self {
            inner: NullLock::new(FrameBufferInner::new(mmio_video_core_addr)),
        }
    }
}

impl DrawPixel for FrameBuffer {
    fn draw_pixel(&self, x: u32, y: u32, color: u32) {
        let offset = (y * self.inner.lock(|inner| inner.pitch)) + x * 4;
        unsafe {
            let ptr = offset as usize + self.inner.lock(|innner| innner.fb);
            *(ptr as *mut u32) = color;
        }
    }
}

impl DrawText for FrameBuffer {
    fn write_char(&self, c: char) {
        
    }
    fn write_fmt(&self, args: core::fmt::Arguments) -> core::fmt::Result {
        Ok(())
    }
}

impl DrawShape for FrameBuffer {
    fn draw_rect(&self, x: usize, y: usize, width: usize, height: usize) {
        //for 
    }
}

impl driver::interface::DeviceDriver for FrameBuffer {
    fn compatible(&self) -> &'static str {
        "Frame Buffer"
    }

    unsafe fn init(&self) -> Result<(), &'static str> {
        self.inner.lock(|inner| inner.init());
        Ok(())
    }
}
