
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

    fn read_pulses(&self, pin: &IoPin, level: Level) -> Result<f32, CliError>;

    fn decode(&self, pulses: Vec<f32>) -> Result<(f32, f32), CliError>;

    fn to_decimal(&self, bits: &Vec<u8>) -> i32;
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

        let mut pulse_counts: [f32; DHT_PULSES * 2] = [0.0; DHT_PULSES * 2];

        // read high and low pulses from the sensor
        for c in 0..DHT_PULSES {
            let i = c * 2;

            pulse_counts[i] = self.read_pulses(&dht_pin, Level::Low)?;
            pulse_counts[i + 1] = self.read_pulses(&dht_pin, Level::High)?;
        }

        // take low pulses values, since we know that their time value is 50ms, get the average, and modify the timings of the list of pulses
        let sum: f32 = pulse_counts
            .iter()
            .step_by(2)
            .skip(1) // remove initial 80ms pulse
            .take(DHT_PULSES - 1)
            .sum();

        let average = sum / (DHT_PULSES as f32- 1.0);

        let time_coefficient = average / 50.0;

        let modified_pulses = pulse_counts
            .iter()
            .skip(2) // remove 2 initial 80ms pulses
            .map(|pulse| pulse / time_coefficient)
            .collect::<Vec<f32>>();

        self.decode(modified_pulses)?;
        Ok(())
    }

    fn read_pulses(&self, pin: &IoPin, level: Level) -> Result<f32, CliError> {
        let mut pulse_count = 0.0;
        while pin.read() == level {
            pulse_count = pulse_count + 1.0;

            if pulse_count > MAX_COUNT as f32 {
                return Err(CliError::Error(String::from("Timeout")));
            }
        }
        Ok(pulse_count)
    }

    fn decode(&self, pulses: Vec<f32>) -> Result<(f32, f32), CliError> {
        let pulses = pulses
            .iter()
            .skip(1)
            .step_by(2)
            .take(DHT_PULSES - 1)
            .map(|x| {
                if *x > 50.0 {
                    1 as u8
                } else {
                    0 as u8
                }
            })
            .collect::<Vec<u8>>();

        let chunks :Vec<Vec<u8>> = pulses
            .chunks(8)
            .map(|s| s.into())
            .collect();

        let result: Vec<u8> = chunks
            .iter()
            .map(|arr|  self.to_decimal(arr) as u8)
            .collect();

        // checking checksum if first 4 numbers mod 255 are the same as the last one
        let sum: u8 = result.iter().take(4).sum();

        if let Some(last) = result.last() {
            if sum != *last {
               return Err(CliError::Error(String::from("Checksum failed.")));
            }
        }

        let temperature = chunks[2]
            .iter()
            .chain(chunks[3].iter())
            .copied()
            .collect::<Vec<u8>>();

        let humidity = chunks[0]
            .iter()
            .chain(chunks[1].iter())
            .copied()
            .collect::<Vec<u8>>();

        println!("temperature: {:?}", self.to_decimal(&temperature) as f32 / 10.0);
        println!("humidity: {:?}", self.to_decimal(&humidity) as f32 / 10.0);

        Ok((0.0,0.0))
    }

    fn to_decimal(&self, bits: &Vec<u8>) -> i32 {
        let mut result: i32 = 0;
        for &bit in bits.iter() {
            result = (result << 1) | bit as i32;
        }

        result
    }
}
