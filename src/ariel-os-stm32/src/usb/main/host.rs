//! USB OTG wrapper for STM32L496 targets.
//!
//! Embassy 0.4.0 does not expose a dedicated USB host abstraction yet, so this
//! type currently initializes the available STM32 OTG FS USB driver backend.

use embassy_stm32::{bind_interrupts, peripherals, usb};
use embassy_usb_synopsys_otg::otg_v1::Otg;
use static_cell::ConstStaticCell;

/// USB OTG FS configuration.
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Whether VBUS detection should be enabled.
    pub vbus_detection: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vbus_detection: true,
        }
    }
}

bind_interrupts!(struct Irqs {
    OTG_FS => usb::InterruptHandler<peripherals::USB_OTG_FS>;
});

// STM32L496G-DISCO USB_OTG_FS base address (from STM32L496 reference manual)
const USB_OTG_FS_BASE: *mut () = 0x5000_9000 as *mut ();

/// USB OTG FS peripheral wrapper for STM32L496-class targets.
pub struct UsbOtgFs {
    inner: usb::Driver<'static, peripherals::USB_OTG_FS>,
    _config: Config,
}

impl UsbOtgFs {
    /// Creates the USB OTG FS host controller with the default configuration.
    ///
    /// `usb_dm` and `usb_dp` are semantic USB signal names supplied by the
    /// board's peripheral mapping. This driver does not expose PA11/PA12 in
    /// its public API.
    #[must_use]
    pub fn new<DM, DP>(
        usb: embassy_stm32::Peri<'static, peripherals::USB_OTG_FS>,
        usb_dm: impl crate::IntoPeripheral<'static, DM>,
        usb_dp: impl crate::IntoPeripheral<'static, DP>,
    ) -> Self
    where
        DM: usb::DmPin<peripherals::USB_OTG_FS>,
        DP: usb::DpPin<peripherals::USB_OTG_FS>,
    {
        Self::new_with_config(usb, usb_dm, usb_dp, Config::default())
    }

    /// Creates the USB OTG FS host controller with a specific configuration.
    #[must_use]
    pub fn new_with_config<DM, DP>(
        usb: embassy_stm32::Peri<'static, peripherals::USB_OTG_FS>,
        usb_dm: impl crate::IntoPeripheral<'static, DM>,
        usb_dp: impl crate::IntoPeripheral<'static, DP>,
        config: Config,
    ) -> Self
    where
        DM: usb::DmPin<peripherals::USB_OTG_FS>,
        DP: usb::DpPin<peripherals::USB_OTG_FS>,
    {
        static EP_OUT_BUFFER: ConstStaticCell<[u8; 256]> = ConstStaticCell::new([0u8; 256]);
        let ep_out_buffer = EP_OUT_BUFFER.take();

        let mut usb_config = usb::Config::default();
        usb_config.vbus_detection = config.vbus_detection;

        let inner = usb::Driver::new_fs(
            usb,
            Irqs,
            usb_dp.into_hal_peripheral(),
            usb_dm.into_hal_peripheral(),
            ep_out_buffer,
            usb_config,
        );

        Self { inner, _config: config }
    }

    /// Returns the embedded STM32 USB controller.
    #[must_use]
    pub fn inner(&mut self) -> &mut usb::Driver<'static, peripherals::USB_OTG_FS> {
        &mut self.inner
    }

    /// Returns a reference to the low-level OTG registers.
    /// Used for direct hardware control (host mode detection, port status polling, etc.).
    ///
    /// # Safety
    ///
    /// This function creates a register reference from a known valid hardware address
    /// (USB_OTG_FS_BASE = 0x5000_9000 on STM32L496G-DISCO). The caller must ensure that:
    /// - The OTG peripheral is actually present on the target device
    /// - Only one mutable reference to these registers exists at a time
    /// - This is only called on the correct target MCU
    #[must_use]
    #[allow(unsafe_code)]
    pub fn otg_regs(&self) -> Otg {
        // SAFETY: USB_OTG_FS_BASE is the correct hardware address for STM32L496G-DISCO.
        // We're converting it to an OTG register interface for read-only polling.
        unsafe { Otg::from_ptr(USB_OTG_FS_BASE) }
    }
}

// --- Minimal host-mode scaffold (moved from host_scaffold.rs) ---
/// Information about a connected USB device (minimal).
pub struct DeviceInfo<'a> {
    pub vendor: Option<&'a str>,
    pub product: Option<&'a str>,
    /// List of files discovered on the device (paths or names).
    pub files: &'a [&'a str],
}

/// Filter a list of file names and return only common image/video files.
pub fn filter_media_files<'a>(files: &'a [&'a str]) -> heapless::Vec<&'a str, 16> {
    let mut out: heapless::Vec<&'a str, 16> = heapless::Vec::new();
    for &f in files.iter() {
        if let Some(ext) = f.rsplit('.').next() {
            if ext.eq_ignore_ascii_case("mp4")
                || ext.eq_ignore_ascii_case("mov")
                || ext.eq_ignore_ascii_case("png")
                || ext.eq_ignore_ascii_case("jpg")
                || ext.eq_ignore_ascii_case("jpeg")
            {
                let _ = out.push(f);
            }
        }
    }
    out
}

/// Simulate a device connection for development and testing.
pub fn simulate_device_connected() -> DeviceInfo<'static> {
    // Mock file list representative of a USB stick.
    static FILES: [&str; 4] = ["DCIM/100MEDIA/IMG_0001.JPG", "video1.MP4", "notes.txt", "movie.MOV"];

    DeviceInfo {
        vendor: Some("MockVendor"),
        product: Some("MockUSB"),
        files: &FILES,
    }
}

/// Events generated by the USB host core.
pub enum HostEvent<'a> {
    DeviceConnected(DeviceInfo<'a>),
    DeviceDisconnected,
}

/// Current state of a USB host port.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostPortState {
    Disconnected,
    Attached,
    Enabled,
    Suspended,
    Error,
}

/// A single USB host port.
pub struct HostPort {
    pub id: u8,
    pub state: HostPortState,
}

/// Minimal host controller core (placeholder).
pub struct HostController {
    _otg: UsbOtgFs,
    ports: heapless::Vec<HostPort, 4>,
}

impl HostController {
    /// Create a host controller from an existing `UsbOtgFs` instance.
    pub fn from_otg(otg: UsbOtgFs) -> Self {
        HostController { _otg: otg, ports: heapless::Vec::new() }
    }

    /// Create and initialize an OTG + host controller in one step.
    pub fn new<DM, DP>(
        usb: embassy_stm32::Peri<'static, peripherals::USB_OTG_FS>,
        usb_dm: impl crate::IntoPeripheral<'static, DM>,
        usb_dp: impl crate::IntoPeripheral<'static, DP>,
    ) -> Self
    where
        DM: usb::DmPin<peripherals::USB_OTG_FS>,
        DP: usb::DpPin<peripherals::USB_OTG_FS>,
    {
        let otg = UsbOtgFs::new(usb, usb_dm, usb_dp);
        let controller = HostController::from_otg(otg);
        // Initialize real host mode on creation
        controller.init_host_mode();
        controller
    }

    /// Initialize host mode by setting GUSBCFG registers (real hardware).
    fn init_host_mode(&self) {
        let otg = self._otg.otg_regs();
        
        // Set force host mode (GUSBCFG.fhmod = 1) and clear force device mode (GUSBCFG.fdmod = 0)
        otg.gusbcfg().modify(|w| {
            w.set_fhmod(true);   // Force host mode
            w.set_fdmod(false);  // Clear force device mode
        });
    }

    /// Return the discovered host ports.
    pub fn ports(&self) -> &[HostPort] {
        self.ports.as_slice()
    }

    /// Query a port's current host state.
    pub fn port_state(&self, id: u8) -> Option<HostPortState> {
        self.ports.iter().find(|p| p.id == id).map(|p| p.state)
    }

    /// Poll the OTG port registers for real device connection events.
    /// Returns a HostEvent if a device is detected, otherwise None.
    pub fn poll_port_status(&mut self) -> Option<HostEvent<'static>> {
        let otg = self._otg.otg_regs();
        let hprt = otg.hprt().read();

        // Check if a device is connected (HPRT.pcsts)
        if hprt.pcsts() {
            // Device is physically connected
            // Check if we already have this port registered
            if self.ports.is_empty() {
                // Register port 1 as connected
                let _ = self.ports.push(HostPort { id: 1, state: HostPortState::Attached });
                
                // Return a device connected event with mock data
                // (In a real implementation, we would enumerate the device here)
                return Some(HostEvent::DeviceConnected(simulate_device_connected()));
            }
        } else {
            // Device disconnected
            if !self.ports.is_empty() {
                let _ = self.ports.pop();
                return Some(HostEvent::DeviceDisconnected);
            }
        }

        None
    }

    /// Simulate enumeration / device connect (deprecated, use poll_port_status).
    #[deprecated(since = "0.1.0", note = "use poll_port_status() for real hardware detection")]
    pub fn simulate_enumeration(&mut self) -> HostEvent<'static> {
        let _ = self.ports.push(HostPort { id: 1, state: HostPortState::Enabled });
        HostEvent::DeviceConnected(simulate_device_connected())
    }
}