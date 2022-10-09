use dht11::Dht11;
use esp_idf_hal::prelude::Peripherals;
use std::any::Any;

pub struct DHTs{
    dhts: Vec<(String, Box<dyn Any>)>
}

impl DHTs{
    pub fn new(pins: Vec<(u8, &str)>) -> Self{
        let mut instance = Self{dhts: Vec::new()};
        for (pin, name) in pins.iter(){
            instance.add_dht_for_pin(*pin, String::from(*name));
        }
        instance
    }

    fn add_dht_for_pin(&mut self, pin: u8, name: String) -> (){
        match pin {
            33 => {
                let chosen_pin = Peripherals::take().unwrap().pins.gpio33.into_input_output().unwrap();
                let dht = Dht11::new(chosen_pin);
                self.dhts.push((name, Box::new(dht)))
            },
            32 => {
                let chosen_pin = Peripherals::take().unwrap().pins.gpio32.into_input_output().unwrap();
                let dht = Dht11::new(chosen_pin);
                self.dhts.push((name, Box::new(dht)))
            },
            27 => {
                let chosen_pin = Peripherals::take().unwrap().pins.gpio27.into_input_output().unwrap();
                let dht = Dht11::new(chosen_pin);
                self.dhts.push((name, Box::new(dht)))
            },
            26 => {
                let chosen_pin = Peripherals::take().unwrap().pins.gpio26.into_input_output().unwrap();
                let dht = Dht11::new(chosen_pin);
                self.dhts.push((name, Box::new(dht)))
            },
            25 => {
                let chosen_pin = Peripherals::take().unwrap().pins.gpio25.into_input_output().unwrap();
                let dht = Dht11::new(chosen_pin);
                self.dhts.push((name, Box::new(dht)))
            },
            _ => panic!("This pin cannot be used")
        }
    }
}