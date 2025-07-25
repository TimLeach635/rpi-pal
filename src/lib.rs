//! rpi-pal provides access to the Raspberry Pi's GPIO, I2C, PWM, SPI and UART
//! peripherals through a user-friendly interface. In addition to peripheral
//! access, rpi-pal also offers support for USB to serial adapters.
//!
//! The library can be used in conjunction with a variety of platform-agnostic
//! drivers through its `embedded-hal` trait implementations. Both `embedded-hal`
//! v0.2.7 and v1 are supported.
//!
//! rpi-pal requires a recent release of Raspberry Pi OS. Similar Linux distributions
//! may work, but are unsupported.
//!
//! Both `gnu` and `musl` libc targets are supported. rpi-pal is compatible with the
//! Raspberry Pi A, A+, B, B+, 2B, 3A+, 3B, 3B+, 4B, 5, CM, CM 3, CM 3+, CM 4, CM 5, 400,
//! Zero, Zero W and Zero 2 W. Backwards compatibility for minor revisions isn't
//! guaranteed until v1.

// Used by rustdoc to link other crates to rpi-pal's docs
#![doc(html_root_url = "https://docs.rs/rpi-pal/0.22.2")]

#[macro_use]
mod macros;

pub mod gpio;
#[cfg(any(
    feature = "embedded-hal-0",
    feature = "embedded-hal",
    feature = "embedded-hal-nb"
))]
pub mod hal;
pub mod i2c;
pub mod pwm;
pub mod spi;
pub mod system;
pub mod uart;
