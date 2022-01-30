pub mod interface {
    pub trait DeviceDriver {
        fn compatible(&self) -> &'static str;
        unsafe fn init(&self) -> Result<(), &'static str> {
            Ok(())
        }
    }

    pub trait DriverManager {
        fn all_device_drivers(&self) -> &[&'static (dyn DeviceDriver + Sync)];
        fn post_device_driver_init(&self);
    }
}
