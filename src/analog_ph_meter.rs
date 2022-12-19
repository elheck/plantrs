use embedded_hal::adc::{OneShot, Channel};
use rp_pico::{hal::Adc};

pub struct PhProbe<T> 
where
  T: Channel<Adc, ID = u8>,
{
  analog_read_pin: T,
}

impl<T> PhProbe<T>
where
    T: Channel<Adc, ID = u8>
{
    pub fn new(analog_pin: T) -> Self{
      PhProbe { analog_read_pin: analog_pin}
    }

    pub fn read(&mut self,  adc: &mut Adc) -> f32{
      let reading: u16 = adc.read(&mut self.analog_read_pin).unwrap();
      let quant_per_volt = 4096.0 / 3.3;
      let voltage = reading as f32 / quant_per_volt;
      // atlas scientific formula https://files.atlas-scientific.com/Gravity-pH-datasheet.pdf
      let ph = (-5.648 * voltage) + 15.509;
      ph
    }
}