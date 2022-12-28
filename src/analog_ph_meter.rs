use embedded_hal::adc::{Channel, OneShot};
use rp_pico::hal::Adc;

const ADC_12_BIT_RESOLUTION_QUANTITY: f32 = 4096.0;
const VOLTAGE_LIMIT_ADC: f32 = 3.3;

pub struct PhProbe<T>
where
    T: Channel<Adc, ID = u8>,
{
    analog_read_pin: T,
}

impl<T> PhProbe<T>
where
    T: Channel<Adc, ID = u8>,
{
    pub fn new(analog_pin: T) -> Self {
        PhProbe {
            analog_read_pin: analog_pin,
        }
    }

    pub fn read(&mut self, adc: &mut Adc) -> f32 {
        let reading: u16 = adc.read(&mut self.analog_read_pin).unwrap();
        let quant_per_volt = ADC_12_BIT_RESOLUTION_QUANTITY / VOLTAGE_LIMIT_ADC;
        let voltage = reading as f32 / quant_per_volt;
        // atlas scientific formula https://files.atlas-scientific.com/Gravity-pH-datasheet.pdf
        (-5.648 * voltage) + 15.509
    }
}
