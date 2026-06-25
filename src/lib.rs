#![doc = include_str!("../README.md")]
#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![no_std]
// #![deny(missing_docs)]
#[cfg(feature = "build")]
extern crate std;

#[cfg(feature = "build")]
pub mod build;

#[doc(hidden)]
pub mod internal;

#[macro_export]
macro_rules! global_static {
    ($mutex:ident, $pin_type:ty) => {
        static $mutex: $crate::internal::Static<
            $pin_type,
            {
                $crate::load_names!(NAMES, NAMES_LENGTH);
                $crate::internal::no_pins(NAMES_LENGTH)
            },
        > = $crate::internal::init_static();
    };
    ($pin_type:ty) => {
        $crate::global_static!(PIN_LOGGER, $pin_type);
    };
}

/// Initialise the logger
///
/// Pass an array of hardware output pins. The pins should implement the [`OutputPin`] trait from embedded-hal.
/// The number of pins needed depends on the number of times [`pin_log`] is used.
///
// # Example (using esp-hal)
// ```
// pin_logger::init!([
//    Output::new(p.GPIO25, Level::Low, Default::default()),
//    Output::new(p.GPIO32, Level::Low, Default::default()),
// ]);
// ```
#[macro_export]
macro_rules! init {
    ($mutex:ident, $output:expr) => {
        critical_section::with(|cs| {
            $crate::load_names!(NAMES, NAMES_LENGTH);
            let logger = $crate::internal::PinLogger::new(&NAMES, $output);
            $mutex.borrow(cs).replace(Some(logger));
        });
    };
    ($output:expr) => {
        $crate::init!(PIN_LOGGER, $output);
    };
}

/// Log a message to the output pins
///
/// Before calling this you need to initialise the logger with [init].
///
/// # Example
// TODO: Would be nice to figure out how to compile this
/// ```ignore
/// pin_log!("Connecting to network");
/// ```
///
#[macro_export]
macro_rules! pin_log {
    ($mutex:ident, $name:literal) => {{
        $crate::load_names!(NAMES, NAMES_LENGTH);
        critical_section::with(|cs| {
            let mut borrow = $mutex.borrow(cs).borrow_mut();
            let logger = borrow.as_mut().unwrap();

            const PIN_STATE: usize = $crate::internal::pin_state_for_name(NAMES, $name)
                .expect("name not found in registry");
            logger.pin_log(PIN_STATE, $name);
        });
    }};
    ($name:literal) => {
        $crate::pin_log!(PIN_LOGGER, $name);
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
