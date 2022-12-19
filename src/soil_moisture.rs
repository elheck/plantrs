
use cortex_m::prelude::_embedded_hal_adc_OneShot;
use embedded_hal::blocking::delay::{DelayMs};
use embedded_hal::adc::Channel;
use embedded_hal::digital::v2::OutputPin;
use rp_pico::hal::{gpio::DynPin, Adc};

const DELAY_TIME: u16 = 5000;

pub struct SoilMoistureSensor<AnalogIn>{
  read_pin: AnalogIn,
  power_switch : DynPin
}

impl<AnalogIn> SoilMoistureSensor<AnalogIn>
where
  AnalogIn: Channel<Adc, ID = u8>,
{
  pub fn new(read: AnalogIn, switch: DynPin) -> Self{
    SoilMoistureSensor { read_pin: read, power_switch: switch}
  }

  pub fn read_blocking<T: DelayMs<u16>>(&mut self, adc: &mut Adc, delay: &mut T) -> u16 {
    self.power_switch.set_high().unwrap();
    delay.delay_ms(DELAY_TIME);
    let value: u16 = adc.read(&mut self.read_pin).unwrap();
    self.power_switch.set_low().unwrap();
    value
  }
}