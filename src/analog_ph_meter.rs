use rp_pico::hal::{gpio::DynPin, Adc};

pub struct PhProbe{
  analog_read_pin: DynPin,
  adc: Adc
}

pub struct PhMeasurement{
  ph: f32
}

impl PhProbe {
    pub fn new(analog_pin: DynPin, adc: Adc) -> Self{
      PhProbe { analog_read_pin: analog_pin, adc }
    }

    pub fn read() -> PhMeasurement{
      PhMeasurement { ph: 1.0 }
    }
}