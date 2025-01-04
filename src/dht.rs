use std::ptr::read_volatile;
use std::ptr::write_volatile;

/// A temperature and humidity reading from the DHT22.
#[derive(Debug, Clone, Copy)]
pub struct Reading {
    pub temperature: f32,
    pub humidity: f32,
}

/// Errors that may occur when reading temperature.
#[derive(Debug)]
pub enum ReadingError {
    /// Occurs if a timeout occured reading the pin.
    Timeout,

    /// Occurs if the checksum value from the DHT22 is incorrect.
    Checksum,

    /// Occurs if there is a problem accessing gpio itself on the Raspberry PI.
    Gpio(rppal::gpio::Error),
}

impl From<rppal::gpio::Error> for ReadingError {
    fn from(err: rppal::gpio::Error) -> ReadingError {
        ReadingError::Gpio(err)
    }
}

pub const DHT_PULSES: usize = 41;

pub fn tiny_sleep() {
    let mut i = 0;
    unsafe {
        while read_volatile(&mut i) < 50 {
            write_volatile(&mut i, read_volatile(&mut i) + 1);
        }
    }
}

pub fn decode(arr: [usize; DHT_PULSES * 2]) -> Result<Reading, ReadingError> {
    let mut threshold: usize = 0;

    let mut i = 2;
    while i < DHT_PULSES * 2 {
        threshold += arr[i];

        i += 2;
    }

    threshold /= DHT_PULSES - 1;

    let mut data = [0 as u8; 5];
    let mut i = 3;
    while i < DHT_PULSES * 2 {
        let index = (i - 3) / 16;
        data[index] <<= 1;
        if arr[i] >= threshold {
            data[index] |= 1;
        } else {
            // else zero bit for short pulse
        }

        i += 2;
    }

    if data[4]
        != (data[0]
            .wrapping_add(data[1])
            .wrapping_add(data[2])
            .wrapping_add(data[3])
            & 0xFF)
    {
        return Result::Err(ReadingError::Checksum);
    }

    let h_dec = data[0] as u16 * 256 + data[1] as u16;
    let h = h_dec as f32 / 10.0f32;

    let t_dec = (data[2] & 0x7f) as u16 * 256 + data[3] as u16;
    let mut t = t_dec as f32 / 10.0f32;
    if (data[2] & 0x80) != 0 {





        t *= -1.0f32;
    }

    Result::Ok(Reading {
        temperature: t,
        humidity: h,
    })
}