use rp_pico::hal::gpio::{DynPin, Error};
use dht_sensor::{dht11::{Reading}, DhtError, Delay, DhtReading};

pub struct Dht11{
  read_pin: DynPin
}

impl Dht11{
  pub fn new(gpio: DynPin)-> Self{
    Dht11{read_pin: gpio} 
  }

  pub fn read(&mut self, delay: &mut dyn Delay) -> Result<Reading, DhtError<Error>>{
    dht_sensor::dht11::Reading::read(delay, &mut self.read_pin)
  }
}