

use rp_pico::hal::gpio::DynPin;
use dht_sensor::{dht11::Reading, DhtError, Delay};

pub struct Dht11{
  read_pin: DynPin
}

impl Dht11{
  pub fn new(gpio: DynPin)-> Self{
    Dht11{read_pin: gpio} 
  }

  pub fn read<E>(&mut self, delay: &dyn Delay) -> Result<Reading, DhtError<E>>{

  }
}