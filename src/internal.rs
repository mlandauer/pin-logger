use core::{cell::RefCell, str::from_utf8};
use critical_section::Mutex;
use embedded_hal::digital::OutputPin;
use log::info;

#[must_use]
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

// By using this we can avoid forcing the user to import RefCell and Mutex
pub type Static<T, const N: usize> = Mutex<RefCell<Option<PinLogger<T, N>>>>;

// By using this we can avoid forcing the user to import RefCell and Mutex
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
    pub fn new(outputs: [P; N]) -> Self {
        let mut logger = Self {
            pin_state: 0,
            outputs,
        };
        // Zeros the output pins
        logger.update_state(0);
        logger
    }

    pub fn pin_log(&mut self, pin_state: usize, name: &str) {
        let before = self.binary_string(self.pin_state);
        let before = from_utf8(&before).unwrap();
        self.update_state(pin_state);
        let after = self.binary_string(self.pin_state);
        let after = from_utf8(&after).unwrap();
        info!("{before}->{after}: {name}");
    }

    fn update_state(&mut self, pin_state: usize) {
        self.pin_state = pin_state;
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
