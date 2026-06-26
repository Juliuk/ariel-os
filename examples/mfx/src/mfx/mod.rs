use embedded_hal_async::i2c::I2c;

mod gpio;
mod lcd;
mod usb;
mod registers;
mod pins;

pub struct Mfx<I2C>
{
    i2c: I2C,
    addr: u8,
}

impl<I2C> Mfx<I2C>
where
    I2C: I2c,
{
    pub fn new(i2c: I2C) -> Self
    {
        Self
        {
            i2c,
            addr: 0x42,
        }
    }

    pub async fn init(&mut self) -> Result<(), I2C::Error>
    {
        self.enable_gpio().await?;

        self.init_lcd_reset().await?;
        self.init_usb_host_power().await?;

        Ok(())
    }

    pub(crate) async fn read_reg(&mut self, reg: u8,) -> Result<u8, I2C::Error>
    {
        let mut buf = [0];
        self.i2c.write_read(self.addr, &[reg], &mut buf).await?;
        Ok(buf[0])
    }

    pub(crate) async fn write_reg(&mut self, reg: u8, value: u8,) -> Result<(), I2C::Error>
    {
        self.i2c.write(self.addr, &[reg, value]).await
    }

    pub(crate) async fn modify_reg<F>(&mut self, reg: u8, f: F,) -> Result<(), I2C::Error>
    where
        F: FnOnce(u8) -> u8,
    {
        let v = self.read_reg(reg).await?;
        self.write_reg(reg, f(v)).await
    }
}
