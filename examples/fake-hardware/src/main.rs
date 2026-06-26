// Simple example that doesn't depend on any particular embedded hardware that shows
// how you can use this library

use pin_logger::pin_log;
use pin_logger::test_pin::TestPin;

pin_logger::global_static!(TestPin);

fn main() {
    colog::init();

    // We only need two output pins but we can pass more if we want
    pin_logger::init!([TestPin::new(0), TestPin::new(1), TestPin::new(2)]);
    pin_log!("Start");
    // Do something here
    pin_log!("Middle");
    // Do something more here
    pin_log!("End")
}
