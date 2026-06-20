#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![no_std]
use alloc::{boxed::Box, string::String, vec::Vec};
use core::cell::RefCell;
use critical_section::Mutex;
use embedded_hal::digital::OutputPin;
use log::info;
extern crate alloc;

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

static PIN_LOGGER: Mutex<RefCell<Option<PinLogger>>> = Mutex::new(RefCell::new(None));

pub fn init_internal<const N: usize, const M: usize>(
    names: &[&str; M],
    outputs: [Box<dyn SetPin>; N],
) {
    critical_section::with(|cs| {
        *PIN_LOGGER.borrow(cs).borrow_mut() = Some(PinLogger::new(names, outputs));
    });
}

pub fn pin_log_internal(pin_state: usize, name: &str) {
    critical_section::with(|cs| {
        let mut borrow_mut = PIN_LOGGER.borrow(cs).borrow_mut();
        let pin_logger = borrow_mut
            .as_mut()
            .expect("call init before calling pin_log!");
        pin_logger.pin_log(pin_state, name);
    });
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
        self.pin_state = pin_state.try_into().expect("number is out of bounds");
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

pub const fn pin_state_for_name<const N: usize>(names: [&str; N], name: &str) -> Option<usize> {
    let mut i: usize = 0;
    while i < N {
        if names[i] == name {
            // The first item on the list should have a number of one
            return Some(i + 1);
        }
        i += 1;
    }
    None
}

#[macro_export]
macro_rules! pin_log {
    ($name:literal) => {{
        const PIN_STATE: usize = pin_logger::pin_state_for_name(NAMES, $name).unwrap();
        pin_logger::pin_log_internal(PIN_STATE, $name);
    }};
}

#[macro_export]
macro_rules! init {
    ($outputs:expr) => {{
        pin_logger::init_internal(&NAMES, $outputs);
    }};
}
