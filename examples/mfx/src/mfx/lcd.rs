use embedded_hal_async::i2c::I2c;

use super::Mfx;
use super::pins::*;

impl<I2C> Mfx<I2C>
where
    I2C: I2c,
{
    pub(crate) async fn init_lcd_reset(&mut self,) -> Result<(), I2C::Error>
    {
        self.set_output(MFX_GP2).await?;
        self.set_high(MFX_GP2).await?;

        Ok(())
    }

    pub async fn lcd_reset(&mut self,) -> Result<(), I2C::Error>
    {
        self.set_low(MFX_GP2).await?;

        ariel_os::time::Timer::after_millis(100).await;

        self.set_high(MFX_GP2).await?;

        ariel_os::time::Timer::after_millis(100).await;

        Ok(())
    }
}
