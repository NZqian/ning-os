#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
mod bcm;
mod video_core;
mod common;

#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
pub use bcm::*;
pub use video_core::*;
