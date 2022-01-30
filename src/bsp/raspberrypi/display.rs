use crate::display;
pub fn display() -> &'static impl display::interface::All {
    &super::FRAME_BUFFER
}
