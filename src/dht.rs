use dht11::Dht11;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use esp_idf_hal::prelude::Peripherals;
use esp_idf_sys::{EspError};
use std::any::Any;

pub struct DHTs{
    dhts: Vec<(String, Box<dyn Any>)>
}

trait InputOutputPin: InputPin<Error = EspError> + OutputPin<Error = EspError>{}
impl<T: InputPin<Error = EspError> + OutputPin<Error = EspError>> InputOutputPin for T {}

impl DHTs{
    pub fn new(pins: Vec<(u8, &str)>) -> (){
        for (pin, name) in pins.iter(){
            let gpio = Self::get_gpio(pin).unwrap();
            let dht = Dht11::new(*gpio);
        }
    }

    fn get_gpio(pin: &u8) -> Result<&dyn InputOutputPin, &'static str>{
        match pin {
            33 => Ok(&Peripherals::take().unwrap().pins.gpio33.into_input_output().unwrap()),
            32 => Ok(&Peripherals::take().unwrap().pins.gpio32.into_input_output().unwrap()),
            27 => Ok(&Peripherals::take().unwrap().pins.gpio27.into_input_output().unwrap()),
            26 => Ok(&Peripherals::take().unwrap().pins.gpio26.into_input_output().unwrap()),
            25 => Ok(&Peripherals::take().unwrap().pins.gpio25.into_input_output().unwrap()),
            _ => Err("Pin not configurable for dht")
        }
    }
}

