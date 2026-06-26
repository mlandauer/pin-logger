#![doc = include_str!("../README.md")]
#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![no_std]
#![deny(missing_docs)]
#[cfg(feature = "build")]
extern crate std;

#[cfg(feature = "build")]
/// Everything related to the build script support
pub mod build;

#[doc(hidden)]
pub mod internal;

/// Contains a "fake" implementation of an output pin struct that can be used for
/// testing and examples
pub mod test_pin;

/// Creates a static in the current context which holds the logger. It's protected by a [`critical_section::Mutex`] so can be used across threads safely.
/// By default the static is called `PIN_LOGGER` but this can be changed.
///
/// The parameter is the type of the output pins that you will pass to [`init`].
///
/// Typically you would put this in the global section of your code so it's easily accessible from anywhere. Logging is one of
/// those few times when using a global actually makes sense.
///
/// # Example
/// ```ignore
/// pin_logger::global_static!(Output);
/// ```
///
/// If you need to choose the static name:
/// ```ignore
/// pin_logger::global_static!(MY_LOGGER_NAME, Output);
/// ```
///
/// For a complete example of using the library with embassy and tasks see `examples/esp-embassy`.
///
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
/// Pass an array of hardware output pins. The pins should implement the [`OutputPin`](embedded_hal::digital::OutputPin) trait from [`embedded-hal`](embedded_hal).
///
/// The number of pins needed depends on the number of times [`pin_log`] is used. For instance, if there
/// are 7 [`pin_log`] calls, We need 3 bits to represent 8 values so at a minimum you'll need to pass 3 output pins.
/// If you want you can pass more. Any extra will just be ignored (but set low at the start).
///
/// Note that the type needs to match that passed to [`global_static`].
///
// # Example (using esp-hal)
// ```
// pin_logger::init!([
//    Output::new(p.GPIO25, Level::Low, Default::default()),
//    Output::new(p.GPIO32, Level::Low, Default::default()),
// ]);
// ```
//
// Using your own static name:
// ```
// pin_logger::init!(
//     MY_LOGGER_NAME,
//     [
//         Output::new(p.GPIO25, Level::Low, Default::default()),
//         Output::new(p.GPIO32, Level::Low, Default::default()),
//     ]
// );
// ```
//
#[macro_export]
macro_rules! init {
    ($mutex:ident, $output:expr) => {
        critical_section::with(|cs| {
            $crate::load_names!(NAMES, NAMES_LENGTH);
            let logger = $crate::internal::PinLogger::new($output);
            $mutex.borrow(cs).replace(Some(logger));
        });
    };
    ($output:expr) => {
        $crate::init!(PIN_LOGGER, $output);
    };
}

/// Log a message to the output pins
///
/// Before calling this you need to initialise the logger with [`init`].
///
/// # Example
// TODO: Would be nice to figure out how to compile this
/// ```ignore
/// pin_log!("Connecting to network");
/// ```
///
/// If you're using your own static name:
/// ```ignore
/// pin_log!(MY_LOGGER_NAME, "Connecting to network");
/// ```
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
