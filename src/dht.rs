
use esp_idf_hal::{prelude::Peripherals, gpio::{InputOutput, GpioPin}};
use std::any::Any;

pub struct DHTs{
    dhts: Vec<(String, Box<dyn Any>)>
}

impl DHTs{
    pub fn new(pins: Vec<(u8, &str)>) -> Self{
        let mut instance = Self{dhts: Vec::new()};
        for (pin, name) in pins.iter(){
            let gpio = Self::get_gpio(pin).unwrap();
            instance.dhts.push((name.to_string(), Box::new(gpio)));
        }
        instance
    }

    fn get_gpio(pin: &u8) -> Result<GpioPin<InputOutput>, &'static str>{
        match pin {
            33 => Ok(Peripherals::take().unwrap().pins.gpio33.into_input_output().unwrap().degrade()),
            32 => Ok(Peripherals::take().unwrap().pins.gpio32.into_input_output().unwrap().degrade()),
            27 => Ok(Peripherals::take().unwrap().pins.gpio27.into_input_output().unwrap().degrade()),
            26 => Ok(Peripherals::take().unwrap().pins.gpio26.into_input_output().unwrap().degrade()),
            25 => Ok(Peripherals::take().unwrap().pins.gpio25.into_input_output().unwrap().degrade()),
            _ => Err("Pin not configurable for dht")
        }
    }
}