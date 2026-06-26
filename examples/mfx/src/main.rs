#![no_std]
#![no_main]

mod pins;
mod mfx;

use ariel_os::{
    hal,
    i2c::controller::{Kilohertz, highest_freq_in},
};

use crate::mfx::Mfx;

#[cfg(feature = "usb-host")]
use ariel_os_stm32::usb::{HostController, HostEvent};

#[ariel_os::task(autostart, peripherals)]
async fn mfx(peripherals: pins::Peripherals)
{
    let mut i2c_config = hal::i2c::controller::Config::default();

    i2c_config.frequency = const { highest_freq_in(Kilohertz::kHz(100)..=Kilohertz::kHz(400)) };

    let i2c = pins::SensorI2c2::new(peripherals.i2c2.i2c_sda, peripherals.i2c2.i2c_scl, i2c_config);

    let mut mfx = Mfx::new(i2c);

    mfx.init().await.unwrap();

    mfx.lcd_reset().await.unwrap();
    mfx.enable_usb_host_power().await.unwrap();

    #[cfg(feature = "usb-host")]
    {
        let mut host = HostController::new(
            peripherals.usb.usb,
            peripherals.usb.usb_dm,
            peripherals.usb.usb_dp,
        );

        // Poll the USB host port for real device connection (register-based detection)
        // In a real application, this would be called periodically (e.g., in a timer task)
        if let Some(event) = host.poll_port_status() {
            match event {
                HostEvent::DeviceConnected(dev) => {
                    info!("USB device connected: {} {}",
                        dev.vendor.unwrap_or(""), dev.product.unwrap_or("") );

                    let media = ariel_os_stm32::usb::filter_media_files(dev.files);
                    for f in media.iter() {
                        info!("media: {}", f);
                    }
                }
                HostEvent::DeviceDisconnected => {
                    info!("USB device disconnected");
                }
            }
        } else {
            info!("No USB device detected (poll_port_status returned None)");
        }
    }
}

/*
#![no_std]
#![no_main]

mod controller;

use ariel_os_boards::pins;
use ariel_os::gpio::{Input, Output, Level, Pull};
use ariel_os::log::*;
use ariel_os::time::Timer;

ariel_os::hal::group_peripherals!(Peripherals {
    leds: pins::LedPeripherals,
    buttons: pins::ButtonPeripherals,
});

#[ariel_os::task(autostart, peripherals)]
async fn mfx(peripherals: Peripherals) {
    let center = Input::new(peripherals.buttons.button0, Pull::Down);
    let up     = Input::new(peripherals.buttons.button1, Pull::Down);
    let down   = Input::new(peripherals.buttons.button2, Pull::Down);
    let left   = Input::new(peripherals.buttons.button3, Pull::Down);
    let right  = Input::new(peripherals.buttons.button4, Pull::Down);
    let mut led0 = Output::new(peripherals.leds.led0, Level::Low);
    let mut led1 = Output::new(peripherals.leds.led1, Level::Low);

    info!("Button test started");

    loop {
        if up.is_high() {
            info!("UP");
        }

        if down.is_high() {
            info!("DOWN");
        }

        if left.is_high() {
            info!("LEFT");
        }

        if right.is_high() {
            info!("RIGHT");
        }

        if center.is_high() {
            info!("CENTER");
        }

        led0.toggle();
        led1.toggle();
        Timer::after_millis(20).await;
    }
}
*/