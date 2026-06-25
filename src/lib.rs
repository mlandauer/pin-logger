#![doc = include_str!("../README.md")]
#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![no_std]
#[cfg(feature = "build")]
extern crate std;

use core::cell::RefCell;
use critical_section::Mutex;
use embedded_hal::digital::OutputPin;

#[cfg(feature = "build")]
pub mod build;

// TODO: Move this inside macro?
pub type Static<T, const N: usize> = Mutex<RefCell<Option<simple::PinLogger<T, N>>>>;

pub const fn init_static<P: OutputPin, const N: usize>()
-> Mutex<RefCell<Option<simple::PinLogger<P, N>>>> {
    Mutex::new(RefCell::new(None))
}

pub mod simple;

#[doc(hidden)]
pub mod internal;

/// Initialise the logger
///
/// Pass an array of hardware output pins. The pins should implement the [`OutputPin`] trait from embedded-hal.
/// The number of pins needed depends on the number of times [`pin_log`] is used.
///
// # Example (using esp-hal)
// ```
// let mut logger = pin_logger::init!([
//    Output::new(p.GPIO25, Level::Low, Default::default()),
//    Output::new(p.GPIO32, Level::Low, Default::default()),
// ]);
// ```
#[macro_export]
macro_rules! init {
    ($output:expr) => {{
        $crate::load_names!(NAMES, NAMES_LENGTH);
        // Boxing here so that we don't actually need all the pins to have the same type
        $crate::simple::PinLogger::new(&NAMES, $output)
    }};
}

#[macro_export]
macro_rules! global_static {
    ($mutex:ident, $pin_type:ty) => {
        static $mutex: $crate::Static<$pin_type, { $crate::no_pins!() }> = $crate::init_static();
    };
    ($pin_type:ty) => {
        $crate::global_static!(PIN_LOGGER, $pin_type);
    };
}

#[macro_export]
macro_rules! init_mutex {
    ($mutex:ident, $output:expr) => {
        critical_section::with(|cs| $mutex.borrow(cs).replace(Some($crate::init!($output))));
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

#[macro_export]
macro_rules! no_pins {
    () => {{
        $crate::load_names!(NAMES, NAMES_LENGTH);
        $crate::simple::no_pins(NAMES_LENGTH)
    }};
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
