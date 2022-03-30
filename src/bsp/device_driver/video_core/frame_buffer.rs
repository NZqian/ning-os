#![allow(dead_code)]
use super::mailbox;
use crate::{
    bsp::device_driver::mailbox::mbox_enum,
    display::interface::{DrawPixel, DrawShape, DrawText},
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
            width: mbox_enum::SCREEN_WIDTH,
            height: mbox_enum::SCREEN_HEIGHT,
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
            panic!("fuck! mbox message error");
        }
        //println!("width: {}", self.width);
        //println!("height: {}", self.height);
        //println!("pitch: {}", self.pitch);
        //println!("is_rgb: {}", self.isrgb);
        //println!("fb: {}", self.fb);
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
    fn write_char(&self, x: u32, y: u32, c: char) {

    }
    fn write_fmt(&self, args: core::fmt::Arguments) -> core::fmt::Result {
        Ok(())
    }
}

impl DrawShape for FrameBuffer {
    fn draw_line(&self, x1: u32, y1: u32, x2: u32, y2: u32, color: u32) {
        let (x1_, x2_, y1_, y2_): (u32, u32, u32, u32);
        if x1 > x2 {
            (x1_, x2_, y1_, y2_) = (x2, x1, y2, y1);
        } else {
            (x1_, x2_, y1_, y2_) = (x1, x2, y1, y2);
        }
        let slide: f32 = (y2_ - y1_) as f32 / (x2_ - x1_) as f32;
        if slide.is_infinite() {
            for i in y1_.min(y2_)..=y1_.max(y2) {
                self.draw_pixel(x1_, i, color);
            }
        } else {
            for i in x1_..=x2_ {
                let y = y1_ + ((i - x1_) as f32 * slide) as u32;
                self.draw_pixel(i, y, color);
            }
        }
    }
    /// top left point
    fn draw_rect(&self, x: u32, y: u32, width: u32, height: u32, color: u32) {
        self.draw_line(x, y, x + width - 1, y, color);
        self.draw_line(x, y, x, y + height - 1, color);
        self.draw_line(x + width - 1, y, x + width - 1, y + height - 1, color);
        self.draw_line(x, y + height - 1, x + width - 1, y + height - 1, color);
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
