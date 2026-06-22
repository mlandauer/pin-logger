// Simple example that doesn't depend on any particular embedded hardware that shows
// how you can use this library

use embedded_hal::digital::{Error, ErrorKind, ErrorType, OutputPin};
use log::debug;
use pin_logger::pin_log;

struct FakePin {
    number: u8,
}

impl FakePin {
    fn new(number: u8) -> Self {
        Self { number }
    }
}

#[derive(Debug)]
struct FakePinError;

impl Error for FakePinError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

impl ErrorType for FakePin {
    type Error = FakePinError;
}

type FakePinResult = Result<(), FakePinError>;

impl OutputPin for FakePin {
    fn set_low(&mut self) -> FakePinResult {
        debug!("Setting pin {} low", self.number);
        Ok(())
    }

    fn set_high(&mut self) -> FakePinResult {
        debug!("Setting pin {} high", self.number);
        Ok(())
    }
}

fn main() {
    colog::init();

    pin_logger::init!([Box::new(FakePin::new(0)), Box::new(FakePin::new(1))]);
    pin_log!("Start");
    // Do something here
    pin_log!("Middle");
    // Do something more here
    pin_log!("End")
}
