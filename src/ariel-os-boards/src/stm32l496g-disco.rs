// @generated

pub mod pins {
    use ariel_os_hal::hal::peripherals;
    ariel_os_hal::define_peripherals!(LedPeripherals { led0 : PB13, led1 : PA5, });
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button0 : PC13, button1 : PI8, button2 : PI10, button3 : PI9,
        button4 : PF11, }
    );
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
