use embedded_hal_async::i2c::I2c;

use super::Mfx;
use super::pins::*;

impl<I2C> Mfx<I2C>
where
    I2C: I2c,
{
    pub(crate) async fn init_usb_host_power(&mut self,) -> Result<(), I2C::Error>
    {
        self.set_output(MFX_GP3).await?;

        self.set_high(MFX_GP3).await?;

        Ok(())
    }

    pub async fn enable_usb_host_power(&mut self,) -> Result<(), I2C::Error>
    {
        self.set_low(MFX_GP3).await
    }

    pub async fn disable_usb_host_power(&mut self,) -> Result<(), I2C::Error>
    {
        self.set_high(MFX_GP3).await
    }
}