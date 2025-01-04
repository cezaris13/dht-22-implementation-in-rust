
use crate::cli_error::CliError;
use crate::dht::*;

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
    fn read(&self, pin: u8) -> Result<(), CliError>;
}

impl ITemperature for Temperature {
    fn read(&self, pin: u8) -> Result<(), CliError> {
        let mut dht_pin: rppal::gpio::IoPin = Gpio::new()?.get(pin)?.into_io(Mode::Output);

        dht_pin.write(Level::High);
        sleep(Duration::from_millis(500));
        dht_pin.write(Level::Low);
        sleep(Duration::from_millis(20));

        dht_pin.set_mode(Mode::Input);
        tiny_sleep();

        let mut count: usize = 0;

        // if the dht returns only high, the dht did not get proper data.
        while dht_pin.read() == Level::High {
            count = count + 1;

            if count > MAX_COUNT {
                return Err(CliError::Error(String::from("Timeout")));
            }
        }

        // read

        Ok(())
    }
}
