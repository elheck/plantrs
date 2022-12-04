
use embedded_hal::digital::v2::OutputPin;
use rp_pico::hal::gpio::{DynPin, Error};



pub struct Pump{
    indicator: DynPin,
    pump_switch: DynPin
}

impl Pump{
    pub fn new(indicator: DynPin, pump_switch: DynPin) -> Self{
        Pump { indicator, pump_switch }
    }

    pub fn turn_on(&mut self) -> Result<(), Error>{
        self.indicator.set_high()?;
        self.pump_switch.set_high()?;
        Ok(())
    }

    pub fn turn_off(&mut self) -> Result<(), Error>{
        self.indicator.set_low()?;
        self.pump_switch.set_low()?;
        Ok(())
    }
}