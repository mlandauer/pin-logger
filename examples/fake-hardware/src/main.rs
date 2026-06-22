// Simple example that doesn't depend on any particular embedded hardware that shows
// how you can use this library

use crate::fake_pin::FakePin;
use pin_logger::pin_log;

mod fake_pin;

fn main() {
    colog::init();

    pin_logger::init!(FakePin::new(0), FakePin::new(1));
    pin_log!("Start");
    // Do something here
    pin_log!("Middle");
    // Do something more here
    pin_log!("End")
}
