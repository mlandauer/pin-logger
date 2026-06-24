#![doc = include_str!("../README.md")]
#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![no_std]
#[cfg(feature = "build")]
extern crate std;

use core::str::from_utf8;
use embedded_hal::digital::OutputPin;
use log::info;

#[cfg(feature = "build")]
pub mod build;

const fn no_pins(names_len: usize) -> usize {
    (names_len.ilog2() + 1) as usize
}

pub trait SetPin {
    fn set_low(&mut self);
    fn set_high(&mut self);
}

impl<P: OutputPin> SetPin for P {
    fn set_low(&mut self) {
        let _ = OutputPin::set_low(self);
    }

    fn set_high(&mut self) {
        let _ = OutputPin::set_high(self);
    }
}

pub struct PinLogger<'a> {
    pin_state: usize,
    outputs: &'a mut [&'a mut dyn SetPin],
}

impl PinLogger<'static> {
    // TODO: It would be nice if we could pass more pins and if there are too many the end ones are discarded
    pub fn new<const M: usize>(
        // We don't need the array, just the size
        _names: &[&str; M],
        outputs: &'static mut [&'static mut dyn SetPin],
    ) -> Self {
        assert!(
            no_pins(M) == outputs.len(),
            "Incorrect number of pins passed in init"
        );
        for output in outputs.iter_mut() {
            output.set_low();
        }
        Self {
            pin_state: 0,
            outputs,
        }
    }

    pub fn pin_log<const N: usize>(&mut self, _names: &[&str; N], pin_state: usize, name: &str) {
        let before = self.binary_string::<N>(self.pin_state);
        let before = from_utf8(&before).unwrap();
        self.pin_state = pin_state;
        let after = self.binary_string::<N>(self.pin_state);
        let after = from_utf8(&after).unwrap();
        self.set_outputs(self.pin_state);
        info!("{before}->{after}: {name}");
    }

    fn set_outputs(&mut self, pin_state: usize) {
        let mut c = pin_state;
        for output in self.outputs.iter_mut() {
            // TODO: Do we want to panic here or return an error or ignore it?
            if c & 1 == 0 {
                output.set_low();
            } else {
                output.set_high();
            }
            c >>= 1;
        }
    }

    // TODO: Do this the same way as in build
    fn binary_string<const N: usize>(&self, pin_state: usize) -> [u8; N] {
        let mut c = pin_state;
        let mut s = [0u8; N];
        for byte in &mut s {
            *byte = if c & 1 == 0 { b'0' } else { b'1' };
            c >>= 1;
        }
        s
    }
}

#[doc(hidden)]
pub mod internal;

#[doc(hidden)]
#[macro_export]
macro_rules! load_names {
    ($name:ident, $length:ident) => {
        // TODO: Give a nice error message if included file doesn't exist (to add build script)
        // TODO: Give nice error message when OUT_DIR env variables doesn't exist
        const $length: usize = include!(concat!(env!("OUT_DIR"), "/names_length.rs"));
        const $name: [&str; $length] = include!(concat!(env!("OUT_DIR"), "/names.rs"));
    };
}

/// Log a message to the output pins
///
/// Before calling this you need to initialise the logger with [init].
///
/// # Example
// TODO: Would be nice to figure out how to compile this
/// ```ignore
/// pin_log!(logger, "Connecting to network");
/// ```
///
#[macro_export]
macro_rules! pin_log {
    ($logger:ident, $name:literal) => {{
        pin_logger::load_names!(NAMES, NAMES_LENGTH);
        const PIN_STATE: usize = pin_logger::internal::pin_state_for_name(NAMES, $name).unwrap();
        $logger.pin_log(&NAMES, PIN_STATE, $name);
    }};
}

/// Initialise the logger
///
/// Pass a list of hardware output pins. The pins should implement the [`OutputPin`] trait from embedded-hal.
/// The number of pins needed depends on the number of times [`pin_log`] is used.
///
// # Example (using esp-hal)
// ```
// let mut logger = pin_logger::init!(
//    Output::new(p.GPIO25, Level::Low, Default::default()),
//    Output::new(p.GPIO32, Level::Low, Default::default()),
// );
// ```
#[macro_export]
macro_rules! init {
    ($($output:expr),* $(,)?) => {{
        pin_logger::load_names!(NAMES, NAMES_LENGTH);
        // Boxing here so that we don't actually need all the pins to have the same type
        pin_logger::PinLogger::new(&NAMES, [$($output),*])
    }};
}

#[macro_export]
macro_rules! init2 {
    ($output:expr) => {{
        pin_logger::load_names!(NAMES, NAMES_LENGTH);
        // Boxing here so that we don't actually need all the pins to have the same type
        pin_logger::PinLogger::new(&NAMES, $output)
    }};
}
