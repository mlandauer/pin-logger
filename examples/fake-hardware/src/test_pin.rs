use embedded_hal::digital::{Error, ErrorKind, ErrorType, OutputPin};
use log::info;

pub(crate) struct TestPin {
    pub(crate) number: u8,
}

impl TestPin {
    pub(crate) fn new(number: u8) -> Self {
        Self { number }
    }
}

#[derive(Debug)]
pub(crate) struct TestPinError;

impl Error for TestPinError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

impl ErrorType for TestPin {
    type Error = TestPinError;
}

pub(crate) type TestPinResult = Result<(), TestPinError>;

impl OutputPin for TestPin {
    fn set_low(&mut self) -> TestPinResult {
        info!("==> Setting test pin {} low", self.number);
        Ok(())
    }

    fn set_high(&mut self) -> TestPinResult {
        info!("==> Setting test pin {} high", self.number);
        Ok(())
    }
}
