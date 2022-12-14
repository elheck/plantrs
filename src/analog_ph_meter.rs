use embedded_hal::adc::OneShot;
use rp_pico::hal::{gpio::{DynPin, Input, Floating, Pin, PinId, bank0::Gpio26, AnyPin, FloatingInput}, Adc};

pub struct PhProbe{
  analog_read_pin: DynPin,
}

pub struct PhMeasurement{
  ph: f32
}

impl PhProbe {
    pub fn new(analog_pin: DynPin) -> Self{
      PhProbe { analog_read_pin: analog_pin}
    }

    pub fn read(&mut self, adc: Adc) -> PhMeasurement{
      let reading: u16 = adc.read(&mut <DynPin as Into<FloatingInput>>::into(self.analog_read_pin)).unwrap();
      PhMeasurement { ph: 1.0 }
    }
}