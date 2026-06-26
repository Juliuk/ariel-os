use ariel_os::hal::{i2c, peripherals};

ariel_os::hal::group_peripherals!(Peripherals {
    pins: LcdPins,
    i2c2: I2C2Pins,
    usb: UsbPeripherals,
});

pub type SensorI2c2 = i2c::controller::I2C2;
ariel_os::hal::define_peripherals!(I2C2Pins {
    i2c_scl: PH4,
    i2c_sda: PB14,
});

ariel_os::hal::define_peripherals!(LcdPins {
    lcd_te: PH7,
    lcd_d15: PD10,
    lcd_d14: PD9,
    lcd_d13: PD8,
    lcd_d12: PE15,
    lcd_d11: PE14,
    lcd_d10: PE13,
    lcd_d9: PE12,
    lcd_d8: PE11,
    lcd_d7: PE10,
    lcd_d6: PE9,
    lcd_d5: PE8,
    lcd_d4: PE7,
    lcd_d3: PD1,
    lcd_d2: PD0,
    lcd_d1: PD15,
    lcd_d0: PD14,
    lcd_rd: PD4,
    lcd_wr: PD5,
    lcd_rs: PD13,
    lcd_cs: PD7,
});

pub type UsbOtgFs = ariel_os_stm32::usb::UsbOtgFs;
ariel_os::hal::define_peripherals!(UsbPeripherals {
    usb: USB_OTG_FS,
    usb_vbus: PA9,
    usb_id: PA10,
    usb_dm: PA11,
    usb_dp: PA12,
});
