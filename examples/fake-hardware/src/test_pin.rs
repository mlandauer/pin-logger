use embedded_hal::digital::OutputPin;
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
pub(crate) struct Error;

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
