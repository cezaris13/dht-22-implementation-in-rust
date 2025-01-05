use crate::cli_error::CliError;

use rppal::gpio::{Gpio, IoPin, Level, Mode};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

#[cfg(test)]
#[path = "./tests/temperature_tests.rs"]
mod tests;

const MAX_COUNT: usize = 32000;
const DHT_PULSES: usize = 41;

#[derive(Debug)]
pub struct TemperatureReading {
    pub temperature: f32,
    pub humidity: f32,
}

pub struct Temperature;

impl Temperature {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait ITemperature {
    fn read(&self, pin: u8) -> Result<TemperatureReading, CliError>;

    fn read_pulses(&self, pin: &IoPin, level: Level) -> Result<usize, CliError>;

    fn decode(&self, pulses: Vec<usize>) -> Result<TemperatureReading, CliError>;

    fn to_decimal(&self, bits: &Vec<u8>, signed: bool) -> i32;

    fn check_checksum(&self, chunks: &Vec<Vec<u8>>) -> Result<(), CliError>;

    fn tiny_sleep(&self);
}

impl ITemperature for Temperature {
    fn read(&self, pin: u8) -> Result<TemperatureReading, CliError> {
        let mut dht_pin = Gpio::new()?.get(pin)?.into_io(Mode::Output);

        dht_pin.write(Level::High);
        sleep(Duration::from_millis(500));
        dht_pin.write(Level::Low);
        sleep(Duration::from_millis(20));

        dht_pin.set_mode(Mode::Input);
        self.tiny_sleep();

        // if the dht returns only high, the dht did not get proper data.
        self.read_pulses(&dht_pin, Level::High)?;

        let mut pulse_counts: [usize; DHT_PULSES * 2] = [0; DHT_PULSES * 2];

        // read high and low pulses from the sensor
        for c in 0..DHT_PULSES {
            let i = c * 2;

            pulse_counts[i] = self.read_pulses(&dht_pin, Level::Low)?;
            pulse_counts[i + 1] = self.read_pulses(&dht_pin, Level::High)?;
        }

        // take low pulses values, since we know that their time value is 50ms, get the average, and modify the timings of the list of pulses
        let sum: usize = pulse_counts
            .iter()
            .step_by(2)
            .skip(1) // remove initial 80ms pulse
            .take(DHT_PULSES - 1)
            .sum();

        let average = sum as f32 / (DHT_PULSES as f32 - 1.0);

        let time_coefficient = average / 50.0;

        let modified_pulses = pulse_counts
            .iter()
            .skip(2) // remove 2 initial 80ms pulses
            .map(|pulse| (*pulse as f32 / time_coefficient).round() as usize)
            .collect::<Vec<usize>>();

        self.decode(modified_pulses)
    }

    fn read_pulses(&self, pin: &IoPin, level: Level) -> Result<usize, CliError> {
        let mut pulse_count = 0;
        while pin.read() == level {
            pulse_count = pulse_count + 1;

            if pulse_count > MAX_COUNT {
                return Err(CliError::Timeout);
            }
        }
        Ok(pulse_count)
    }

    fn decode(&self, pulses: Vec<usize>) -> Result<TemperatureReading, CliError> {
        let pulses = pulses
            .iter()
            .skip(1) // start from high pulse
            .step_by(2)
            .take(DHT_PULSES - 1)
            .map(|x| if *x > 50 { 1 as u8 } else { 0 as u8 })
            .collect::<Vec<u8>>();

        let chunks: Vec<Vec<u8>> = pulses.chunks(8).map(|s| s.into()).collect();

        self.check_checksum(&chunks)?;

        // 1st and 2nd chunks (indexed 0 and 1) are humidity bits
        let humidity = chunks[0]
            .iter()
            .chain(chunks[1].iter())
            .copied()
            .collect::<Vec<u8>>();

        // 3rd and 4th chunks (indexed 2 and 3) are temperature bits
        let temperature = chunks[2]
            .iter()
            .chain(chunks[3].iter())
            .copied()
            .collect::<Vec<u8>>();

        Ok(TemperatureReading {
            temperature: self.to_decimal(&temperature, true) as f32 / 10.0,
            humidity: self.to_decimal(&humidity, false) as f32 / 10.0,
        })
    }

    fn to_decimal(&self, bits: &Vec<u8>, signed: bool) -> i32 {
        let skip = signed as usize; // If signed, skip 1 bit; otherwise, skip 0.

        let mut result = bits
            .iter()
            .skip(skip)
            .fold(0, |acc, &bit| (acc << 1) | bit as i32);

        if signed && bits[0] == 1 {
            result = -result;
        }

        result
    }

    fn check_checksum(&self, chunks: &Vec<Vec<u8>>) -> Result<(), CliError> {
        let chunks_as_u8: Vec<u8> = chunks
            .iter()
            .map(|arr| self.to_decimal(arr, false) as u8)
            .collect();

        // checking checksum if first 4 numbers mod 255 are the same as the last one
        let sum: u8 = chunks_as_u8
            .iter()
            .take(4)
            .fold(0u8, |acc, &x| acc.wrapping_add(x));

        if let Some(last) = chunks_as_u8.last() {
            if sum != *last {
                return Err(CliError::Checksum);
            }
        }

        Ok(())
    }

    // 5 microsecond sleep
    fn tiny_sleep(&self) {
        let time = SystemTime::now();

        loop {
            match time.elapsed() {
                Ok(duration) => {
                    if duration >= Duration::from_micros(5) {
                        return;
                    }
                }
                // System clock has gone backwards, just abort
                Err(_) => return,
            }
        }
    }
}
