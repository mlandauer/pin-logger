use crate::simple::PinLogger;
use core::cell::RefCell;
use critical_section::Mutex;
use embedded_hal::digital::OutputPin;

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
