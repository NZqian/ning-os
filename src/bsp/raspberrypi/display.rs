use crate::display;
pub fn display() -> &'static impl display::interface::DrawPixel {
    &super::FRAME_BUFFER
}
