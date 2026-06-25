use core::str::from_utf8;
use embedded_hal::digital::OutputPin;
use log::info;

pub const fn no_pins(names_len: usize) -> usize {
    (names_len.ilog2() + 1) as usize
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
