use embedded_hal::digital::OutputPin;
use log::info;

/// Can be used for naive testing and examples. Implements the
/// [`embedded_hal::digital::OutputPin`] trait and just logs any state
/// changes
pub struct TestPin {
    number: u8,
}

impl TestPin {
    /// Create a new test pin with a given number which is just used in logging
    pub fn new(number: u8) -> Self {
        Self { number }
    }
}

/// For errors in TestPin
#[derive(Debug)]
pub struct Error;

impl embedded_hal::digital::Error for Error {
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        embedded_hal::digital::ErrorKind::Other
    }
}

impl embedded_hal::digital::ErrorType for TestPin {
    type Error = Error;
}

impl OutputPin for TestPin {
    fn set_low(&mut self) -> Result<(), Error> {
        info!("==> Setting test pin {} low", self.number);
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Error> {
        info!("==> Setting test pin {} high", self.number);
        Ok(())
    }
}
