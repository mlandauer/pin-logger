#![doc = include_str!("../README.md")]
#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![no_std]
#[cfg(feature = "build")]
extern crate std;

use core::{cell::RefCell, str::from_utf8};
use critical_section::Mutex;
use embedded_hal::digital::OutputPin;
use log::info;

#[cfg(feature = "build")]
pub mod build;

pub type Static<T, const N: usize> = Mutex<RefCell<Option<PinLogger<T, N>>>>;

pub const fn init_static<P: OutputPin, const N: usize>() -> Mutex<RefCell<Option<PinLogger<P, N>>>>
{
    Mutex::new(RefCell::new(None))
}

pub const fn no_pins(names_len: usize) -> usize {
    (names_len.ilog2() + 1) as usize
}

pub struct PinLogger<P, const N: usize>
where
    P: OutputPin,
{
    pin_state: usize,
    outputs: [P; N],
}

impl<P, const N: usize> PinLogger<P, N>
where
    P: OutputPin,
{
    // TODO: It would be nice if we could pass more pins and if there are too many the end ones are discarded
    pub fn new<const M: usize>(
        // We don't need the array, just the size
        _names: &[&str; M],
        mut outputs: [P; N],
    ) -> Self {
        assert!(
            no_pins(M) == outputs.len(),
            "Incorrect number of pins passed in init"
        );
        for output in outputs.iter_mut() {
            output.set_low().unwrap();
        }
        Self {
            pin_state: 0,
            outputs,
        }
    }

    pub fn pin_log(&mut self, pin_state: usize, name: &str) {
        let before = self.binary_string(self.pin_state);
        let before = from_utf8(&before).unwrap();
        self.pin_state = pin_state;
        let after = self.binary_string(self.pin_state);
        let after = from_utf8(&after).unwrap();
        self.set_outputs(self.pin_state);
        info!("{before}->{after}: {name}");
    }

    fn set_outputs(&mut self, pin_state: usize) {
        let mut c = pin_state;
        for output in self.outputs.iter_mut() {
            // TODO: Do we want to panic here or return an error or ignore it?
            if c & 1 == 0 {
                output.set_low().unwrap();
            } else {
                output.set_high().unwrap();
            }
            c >>= 1;
        }
    }

    // TODO: Do this the same way as in build
    fn binary_string(&self, pin_state: usize) -> [u8; N] {
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
        assert!($length > 0);
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
        $crate::load_names!(NAMES, NAMES_LENGTH);
        const PIN_STATE: usize =
            $crate::internal::pin_state_for_name(NAMES, $name).expect("name not found in registry");
        $logger.pin_log(PIN_STATE, $name);
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
        $crate::load_names!(NAMES, NAMES_LENGTH);
        // Boxing here so that we don't actually need all the pins to have the same type
        $crate::PinLogger::new(&NAMES, [$($output),*])
    }};
}

#[macro_export]
macro_rules! init2 {
    ($output:expr) => {{
        $crate::load_names!(NAMES, NAMES_LENGTH);
        // Boxing here so that we don't actually need all the pins to have the same type
        $crate::PinLogger::new(&NAMES, $output)
    }};
}

#[macro_export]
macro_rules! no_pins {
    () => {{
        $crate::load_names!(NAMES, NAMES_LENGTH);
        $crate::no_pins(NAMES_LENGTH)
    }};
}

#[macro_export]
macro_rules! init_static {
    ($mutex:ident, $pin_type:ty) => {
        static $mutex: $crate::Static<$pin_type, { $crate::no_pins!() }> = $crate::init_static();
    };
    ($pin_type:ty) => {
        $crate::init_static!(PIN_LOGGER, $pin_type);
    };
}

#[macro_export]
macro_rules! init_mutex {
    ($mutex:ident, $output:expr) => {
        critical_section::with(|cs| $mutex.borrow(cs).replace(Some($crate::init2!($output))));
    };
    ($output:expr) => {
        $crate::init_mutex!(PIN_LOGGER, $output);
    };
}

#[macro_export]
macro_rules! pin_log_mutex {
    ($mutex:ident, $name:literal) => {{
        critical_section::with(|cs| {
            let mut borrow = $mutex.borrow(cs).borrow_mut();
            let l = borrow.as_mut().unwrap();
            $crate::pin_log!(l, $name);
        });
    }};
    ($name:literal) => {
        $crate::pin_log_mutex!(PIN_LOGGER, $name);
    };
}
