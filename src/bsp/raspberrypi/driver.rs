use crate::driver;

struct BSPDriverManager {
    device_drivers: [&'static (dyn DeviceDriver + Sync); 3],
}

static BSP_DRIVER_MANAGER: BSPDriverManager = BSPDriverManager {
    device_drivers: [
        &super::GPIO, 
        &super::PL011_UART,
        &super::FRAME_BUFFER,
        ],
};

pub fn driver_manager() -> &'static impl driver::interface::DriverManager {
    &BSP_DRIVER_MANAGER
}

use driver::interface::DeviceDriver;

impl driver::interface::DriverManager for BSPDriverManager {
    fn all_device_drivers(&self) -> &[&'static (dyn DeviceDriver + Sync)] {
        &self.device_drivers[..]
    }
    fn post_device_driver_init(&self) {
        super::GPIO.map_pl011_uart();
    }
}
