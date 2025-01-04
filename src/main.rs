mod dht;
mod temperature;
mod cli_error;

use crate::temperature::{ITemperature, Temperature};

use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let temperature = Temperature::new();

    let pin: u8 = 4;
    let temperature_reading = temperature.read(pin)?;

    println!("{:?}", temperature_reading);

    Ok(())
}
