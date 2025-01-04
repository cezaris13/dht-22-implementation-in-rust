mod dht;
mod temperature;
mod cli_error;

use crate::temperature::{ITemperature, Temperature};

use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let pin: u8 = 4;

    let temperature = Temperature::new();
    temperature.read(pin)?;

    // test another lib
    let result = dht::read(4).unwrap();
    println!("{:?}", result);
    Ok(())
}
