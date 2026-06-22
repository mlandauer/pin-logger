use embedded_hal::digital::OutputPin;

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
