
use crate::cli_error::CliError;
use crate::dht::*;

use rppal::gpio::{Gpio, Level, Mode, IoPin};
use std::thread::sleep;
use std::time::Duration;

const MAX_COUNT: usize = 32000;
const DHT_PULSES: usize = 41;

pub struct Temperature;

impl Temperature {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait ITemperature {
    fn read(&self, pin: u8) -> Result<(), CliError>;

    fn read_pulses(&self, pin: &IoPin, level: Level) -> Result<usize, CliError>;
}

impl ITemperature for Temperature {
    fn read(&self, pin: u8) -> Result<(), CliError> {
        let mut dht_pin = Gpio::new()?.get(pin)?.into_io(Mode::Output);

        dht_pin.write(Level::High);
        sleep(Duration::from_millis(500));
        dht_pin.write(Level::Low);
        sleep(Duration::from_millis(20));

        dht_pin.set_mode(Mode::Input);
        tiny_sleep();

        // if the dht returns only high, the dht did not get proper data.
        self.read_pulses(&dht_pin, Level::High)?;

        let mut pulse_counts: [usize; DHT_PULSES * 2] = [0; DHT_PULSES * 2];
        
        // read high and low pulses from the sensor
        for c in 0..DHT_PULSES {
            let i = c * 2;

            pulse_counts[i] = self.read_pulses(&dht_pin, Level::Low)?;
            pulse_counts[i + 1] = self.read_pulses(&dht_pin, Level::High)?;
        }

        println!("{:?}", pulse_counts);

        Ok(())
    }

    fn read_pulses(&self, pin: &IoPin, level: Level) -> Result<usize, CliError> {
        let mut pulse_count = 0;
        while pin.read() == level {
            pulse_count = pulse_count + 1;

            if pulse_count > MAX_COUNT {
                return Err(CliError::Error(String::from("Timeout")));
            }
        }
        Ok(pulse_count)
    }
}
