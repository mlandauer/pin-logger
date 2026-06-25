use core::str::from_utf8;
use embedded_hal::digital::OutputPin;
use log::info;

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

pub const fn no_pins(names_len: usize) -> usize {
    (names_len.ilog2() + 1) as usize
}

#[macro_export]
macro_rules! no_pins {
    () => {{
        $crate::load_names!(NAMES, NAMES_LENGTH);
        $crate::simple::no_pins(NAMES_LENGTH)
    }};
}

pub struct PinLogger<P, const N: usize>
where
    P: OutputPin,
{
    pub(crate) pin_state: usize,
    pub(crate) outputs: [P; N],
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
