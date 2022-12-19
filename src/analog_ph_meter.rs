use embedded_hal::adc::{OneShot, Channel};
use rp_pico::{hal::Adc};

pub struct PhProbe<T> 
where
  T: Channel<Adc, ID = u8>,
{
  analog_read_pin: T,
}

pub struct PhMeasurement{
  ph: f32
}

impl<T> PhProbe<T>
where
    T: Channel<Adc, ID = u8>
{
    pub fn new(analog_pin: T) -> Self{
      PhProbe { analog_read_pin: analog_pin}
    }

    pub fn read(&mut self, mut adc: Adc) -> PhMeasurement{
      let reading: u16 = adc.read(&mut self.analog_read_pin).unwrap();
      PhMeasurement { ph: 1.0 }
    }
}