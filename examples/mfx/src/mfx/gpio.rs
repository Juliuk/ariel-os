use embedded_hal_async::i2c::I2c;

use super::Mfx;
use super::registers::*;



impl<I2C> Mfx<I2C>
where
    I2C: I2c,
{
    pub(crate) async fn enable_gpio(&mut self,) -> Result<(), I2C::Error>
    {
        self.modify_reg(SYS_CTRL, |v| v | GPIO_EN).await
    }

    pub(crate) async fn set_output(&mut self,pin: u8,) -> Result<(), I2C::Error>
    {
        self.modify_reg(GPIO_DIR1, |v| v | pin).await
    }

    pub(crate) async fn set_high(&mut self,pin: u8,) -> Result<(), I2C::Error>
    {
        self.write_reg(GPO_SET1, pin).await
    }

    pub(crate) async fn set_low(&mut self,pin: u8,) -> Result<(), I2C::Error>
    {
        self.write_reg(GPO_CLR1, pin).await
    }
}
