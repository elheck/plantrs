
use dht11::{Dht11, Measurement};
use esp_idf_hal::{prelude::Peripherals, gpio::{InputOutput, GpioPin}, delay::Ets};

pub struct DHTs{
    dhts: Vec<(String, Box<Dht11<GpioPin<InputOutput>>>)>
}

impl DHTs{
    pub fn new(pins: Vec<(u8, &str)>) -> Self{
        let mut instance = Self{dhts: Vec::new()};
        for (pin, name) in pins.iter(){
            let gpio = Self::get_gpio(pin).unwrap();
            let dht = Dht11::new(gpio);
            instance.dhts.push((name.to_string(), Box::new(dht)));
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

    pub fn get_measurements(&mut self) -> Vec<(String, Measurement)>{
        let mut delay = Ets{};
        let mut measurement_vec = Vec::new();
        for (name, dht) in self.dhts.iter_mut(){
            let measurement = dht.perform_measurement(&mut delay).unwrap();
            measurement_vec.push((name.clone(), measurement));
        }
        measurement_vec
    }

    pub fn get_measurement_for(&mut self, dht_name: &str) -> Result<Measurement, &'static str>{
        let mut delay = Ets{};
        for(name, dht) in self.dhts.iter_mut(){
            if name.clone().eq(&String::from(dht_name)){
                return Ok(dht.perform_measurement(&mut delay).unwrap());
            }
        }
        Err("Name not found")
    }
    
}