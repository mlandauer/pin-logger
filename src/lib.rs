#![doc = include_str!("../README.md")]
#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![no_std]
#![deny(clippy::pedantic)]
extern crate alloc;
#[cfg(feature = "build")]
extern crate std;

use alloc::{boxed::Box, string::String, vec::Vec};
use log::info;

use crate::internal::SetPin;

#[cfg(feature = "build")]
pub mod build;

const fn no_pins(names_len: usize) -> usize {
    (names_len.ilog2() + 1) as usize
}

pub struct PinLogger {
    pin_state: usize,
    outputs: Vec<Box<dyn SetPin>>,
}

impl PinLogger {
    // TODO: It would be nice if we could pass more pins and if there are too many the end ones are discarded
    pub fn new<const N: usize, const M: usize>(
        // We don't need the array, just the size
        _names: &[&str; M],
        outputs: [Box<dyn SetPin>; N],
    ) -> Self {
        const { assert!(no_pins(M) == N, "Incorrect number of pins passed in init") };
        let mut outputs: Vec<_> = outputs.into_iter().collect();
        for output in &mut outputs {
            output.set_low();
        }
        Self {
            pin_state: 0,
            outputs,
        }
    }

    pub fn pin_log(&mut self, pin_state: usize, name: &str) {
        let before = self.binary_string(self.pin_state);
        self.pin_state = pin_state;
        let after = self.binary_string(self.pin_state);
        self.set_outputs(self.pin_state);
        info!("{before}->{after}: {name}");
    }

    fn set_outputs(&mut self, pin_state: usize) {
        let mut c = pin_state;
        for output in &mut self.outputs {
            if c & 1 == 0 {
                output.set_low();
            } else {
                output.set_high();
            }
            c >>= 1;
        }
    }

    // TODO: Do this the same way as in build
    fn binary_string(&self, pin_state: usize) -> String {
        let mut c = pin_state;
        let mut s = String::new();
        for _ in &self.outputs {
            s += if c & 1 == 0 { "0" } else { "1" };
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
/// Before calling this you need to initialise the logger with `init`.
///
/// # Example
/// ```
/// pin_log!("Connecting to network");
/// ```
///
#[macro_export]
macro_rules! pin_log {
    ($logger:ident, $name:literal) => {{
        pin_logger::load_names!(NAMES, NAMES_LENGTH);
        const PIN_STATE: usize = pin_logger::internal::pin_state_for_name(NAMES, $name).unwrap();
        $logger.pin_log(PIN_STATE, $name);
    }};
}

/// Initialise the logger
///
/// Pass a list of hardware output pins. The pins should implement the `OutputPin` trait from embedded-hal.
/// The number of pins needed depends on the number of times `pin_log` is used.
///
/// # Example (using esp-hal)
/// ```
/// pin_logger::init!(
///    Output::new(p.GPIO25, Level::Low, Default::default()),
///    Output::new(p.GPIO32, Level::Low, Default::default()),
/// );
/// ```
#[macro_export]
macro_rules! init {
    ($($output:expr),* $(,)?) => {{
        pin_logger::load_names!(NAMES, NAMES_LENGTH);
        // Boxing here so that we don't actually need all the pins to have the same type
        pin_logger::PinLogger::new(&NAMES, [$(alloc::boxed::Box::new($output)),*])
    }};
}
