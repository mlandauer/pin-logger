use crate::{PinLogger, SetPin};
use alloc::boxed::Box;
use core::cell::RefCell;
use critical_section::Mutex;

static PIN_LOGGER: Mutex<RefCell<Option<PinLogger>>> = Mutex::new(RefCell::new(None));

// TODO: Would be nice to have a simpler interface that didn't depend on Box
pub fn init<const N: usize, const M: usize>(names: &[&str; M], outputs: [Box<dyn SetPin>; N]) {
    critical_section::with(|cs| {
        *PIN_LOGGER.borrow(cs).borrow_mut() = Some(PinLogger::new(names, outputs));
    });
}

/// # Panics
/// init needs to be called first before doing anything else. Otherwise this will panic
pub fn pin_log(pin_state: usize, name: &str) {
    critical_section::with(|cs| {
        let mut borrow_mut = PIN_LOGGER.borrow(cs).borrow_mut();
        let pin_logger = borrow_mut
            .as_mut()
            .expect("call init before calling pin_log!");
        pin_logger.pin_log(pin_state, name);
    });
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
