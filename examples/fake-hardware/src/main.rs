// Simple example that doesn't depend on any particular embedded hardware that shows
// how you can use this library

use crate::fake_pin::FakePin;
use pin_logger::pin_log;

mod fake_pin;

fn main() {
    colog::init();

    let mut l = pin_logger::init!(FakePin::new(0), FakePin::new(1));
    pin_log!(l, "Start");
    // Do something here
    pin_log!(l, "Middle");
    // Do something more here
    pin_log!(l, "End")
}
