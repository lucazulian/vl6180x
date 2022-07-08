use embedded_hal::{
    blocking::i2c::{Write, WriteRead},
    digital::v2::OutputPin,
};

use crate::{error::Error2, mode::ReadyMode, VL6180X};

/// Sensor is powered off
#[derive(Debug, Copy, Clone)]
pub struct PoweredOffMode {}

impl<I2C, E> VL6180X<PoweredOffMode, I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Powers on the sensor by setting the `x_shutdown_pin` high.
    /// It then busy waits for the device to be booted and initializes the device.
    pub fn power_on_and_init<PE, P: OutputPin<Error = PE>>(
        mut self,
        x_shutdown_pin: &mut P,
    ) -> Result<VL6180X<ReadyMode, I2C>, Error2<E, PE>> {
        self.power_on_and_init_direct(x_shutdown_pin)?;
        Ok(self.into_mode(ReadyMode))
    }
}
