use embedded_hal::digital::{Error, ErrorKind, ErrorType, OutputPin};
use log::info;

pub(crate) struct FakePin {
    pub(crate) number: u8,
}

impl FakePin {
    pub(crate) fn new(number: u8) -> Self {
        Self { number }
    }
}

#[derive(Debug)]
pub(crate) struct FakePinError;

impl Error for FakePinError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

impl ErrorType for FakePin {
    type Error = FakePinError;
}

pub(crate) type FakePinResult = Result<(), FakePinError>;

impl OutputPin for FakePin {
    fn set_low(&mut self) -> FakePinResult {
        info!("==> Setting fake pin {} low", self.number);
        Ok(())
    }

    fn set_high(&mut self) -> FakePinResult {
        info!("==> Setting fake pin {} high", self.number);
        Ok(())
    }
}
