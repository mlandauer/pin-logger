#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![no_std]
#![deny(clippy::pedantic)]
extern crate alloc;
#[cfg(feature = "build")]
extern crate std;

use alloc::{boxed::Box, string::String, vec::Vec};
use embedded_hal::digital::OutputPin;
use log::info;

#[cfg(feature = "build")]
pub mod build;

const fn no_pins(names_len: usize) -> usize {
    (names_len.ilog2() + 1) as usize
}

pub trait SetPin: Send {
    fn set_low(&mut self);
    fn set_high(&mut self);
}

impl<P: OutputPin + Send> SetPin for P {
    fn set_low(&mut self) {
        let _ = OutputPin::set_low(self);
    }

    fn set_high(&mut self) {
        let _ = OutputPin::set_high(self);
    }
}

struct PinLogger {
    pin_state: usize,
    outputs: Vec<Box<dyn SetPin>>,
}

impl PinLogger {
    // TODO: It would be nice if we could pass more pins and if there are too many the end ones are discarded
    fn new<const N: usize, const M: usize>(
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

    fn pin_log(&mut self, pin_state: usize, name: &str) {
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

pub mod internal;

#[macro_export]
macro_rules! pin_log {
    ($name:literal) => {{
        // TODO: Give a nice error message if included file doesn't exist (to add build script)
        const NAMES_LENGTH: usize = include!(concat!(env!("OUT_DIR"), "/names_length.rs"));
        const NAMES: [&str; NAMES_LENGTH] = include!(concat!(env!("OUT_DIR"), "/mark_names.rs"));
        const PIN_STATE: usize = pin_logger::internal::pin_state_for_name(NAMES, $name).unwrap();
        // TODO: Move internal function to a module which has hidden documentation
        pin_logger::internal::pin_log(PIN_STATE, $name);
    }};
}

#[macro_export]
macro_rules! init {
    ($outputs:expr) => {{
        const NAMES_LENGTH: usize = include!(concat!(env!("OUT_DIR"), "/names_length.rs"));
        const NAMES: [&str; NAMES_LENGTH] = include!(concat!(env!("OUT_DIR"), "/mark_names.rs"));
        pin_logger::internal::init(&NAMES, $outputs);
    }};
}
