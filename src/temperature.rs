
use crate::cli_error::CliError;

use rppal::gpio::{Gpio, Level, Mode};
use std::thread::sleep;
use std::time::Duration;

pub struct Temperature;

impl Temperature {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait ITemperature {
    fn get_bits(&self, pin: u8) -> Result<(), CliError>;
}

impl ITemperature for Temperature {
    fn get_bits(&self, pin: u8) -> Result<(), CliError> {
        let mut dht_pin: rppal::gpio::IoPin = Gpio::new()?.get(pin)?.into_io(Mode::Output);

        dht_pin.write(Level::High);
        sleep(Duration::from_millis(500));
        dht_pin.write(Level::Low);
        sleep(Duration::from_millis(20));

        dht_pin.set_mode(Mode::Input);
        Ok(())
    }
}
