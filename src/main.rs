mod dht;
mod temperature;
mod cli_error;

use crate::temperature::{ITemperature, Temperature};

use dht::{decode, tiny_sleep, DHT_PULSES, MAX_COUNT};
use rppal::gpio::{Level, Mode};
use std::{
    error::Error,
    thread::sleep,
    time::Duration,
};

pub fn main() -> Result<(), Box<dyn Error>> {
    let pin: u8 = 4;

    let temperature = Temperature::new();

    temperature.get_bits(pin)?;
    tiny_sleep();
    // let mut count: usize = 0;

    // while dht_pin.read() == Level::High {
    //     count = count + 1;

    //     if count > MAX_COUNT {
    //         panic!("maxcount")
    //         // return Error();
    //     }
    // }

    // let mut previous_state = Level::Low;
    // let mut state_count: i32 = 0;
    // let mut final_str: String = String::from("");
    // for _ in 1..DHT_PULSES * 2 {
    //     while dht_pin.read() == previous_state {
    //         state_count += 1;
    //     }
    //     // println!("{:?}, {:?}", previous_state, state_count);
    //     if previous_state == Level::High {
    //         if state_count > 50 {
    //             final_str.push('1');
    //         } else {
    //             final_str.push('0');
    //         }
    //         previous_state = Level::Low;
    //     } else {
    //         previous_state = Level::High;
    //     }
    //     state_count = 1;
    //     // if value == Level::Low {
    //     //     print!("{}",0);
    //     // } else {
    //     //     print!("{}",1);
    //     // }
    //     // sleep(Duration::from_millis(20));
    // }
    // // dht_pin.into_io(mode)
    // // let two_secs = time::Duration::from_secs(2);
    // // for _ in 1..1000 {
    // // dht_pin.
    // // match read(4) {
    // //     Ok(res) => println!(
    // //         "Temperature: {0}, humidity: {1}",
    // //         res.temperature, res.humidity
    // //     ),
    // //     Err(e) => println!("Failed to read data: {:?}", e),
    // // }
    // // thread::sleep(two_secs);

    // // }
    // println!("");
    // println!("{:?}", final_str);
    // let mut sections = vec![];

    // for i in 0..5 {
    //     let start = i * 8;
    //     let end = start + 8;
    //     sections.push(&final_str[start..end]);
    // }

    // for (i, section) in sections.iter().enumerate() {
    //     let integer_value = u32::from_str_radix(section, 2).unwrap();

    //     println!("Section {}: {} ({})", i + 1, section, integer_value);
    // }

    // // sleep(Duration::from_millis(2000));
    // dht_pin.set_mode(Mode::Output);
    // dht_pin.write(Level::High);
    // sleep(Duration::from_millis(500));

    // dht_pin.write(Level::Low);
    // sleep(Duration::from_millis(20));

    // dht_pin.set_mode(Mode::Input); // Sometimes the pin is briefly low.
    // tiny_sleep();

    // while dht_pin.read() == Level::High {
    //     count = count + 1;

    //     if count > MAX_COUNT {
    //         panic!("aaa");
    //         // return Result::Err(ReadingError::Timeout);
    //     }
    // }
    // let mut pulse_counts: [usize; DHT_PULSES * 2] = [0; DHT_PULSES * 2];
    // for c in 0..DHT_PULSES {
    //     let i = c * 2;

    //     while dht_pin.read() == Level::Low {
    //         pulse_counts[i] = pulse_counts[i] + 1;

    //         if pulse_counts[i] > MAX_COUNT {
    //             // return Result::Err(ReadingError::Timeout);
    //         }
    //     }

    //     while dht_pin.read() == Level::High {
    //         pulse_counts[i + 1] = pulse_counts[i + 1] + 1;

    //         if pulse_counts[i + 1] > MAX_COUNT {
    //             // return Result::Err(ReadingError::Timeout);
    //         }
    //     }
    // }

    // println!("{:?}", decode(pulse_counts));
    Ok(())
}
