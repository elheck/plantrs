use embedded_hal::digital::v2::OutputPin;

pub struct WaterPump<T, P>
where
    T: OutputPin,
    P: OutputPin,
{
    indicator: T,
    pump_switch: P,
}

impl<T, P> WaterPump<T, P>
where
    T: OutputPin,
    P: OutputPin,
{
    pub fn new(indicator: T, pump_switch: P) -> Self {
        WaterPump {
            indicator,
            pump_switch,
        }
    }

    pub fn turn_on(&mut self) -> Result<(), T::Error>
    where
        T::Error: From<<P as OutputPin>::Error>,
    {
        self.indicator.set_high()?;
        self.pump_switch.set_high()?;
        Ok(())
    }

    pub fn turn_off(&mut self) -> Result<(), T::Error>
    where
        T::Error: From<<P as OutputPin>::Error>,
    {
        self.indicator.set_low()?;
        self.pump_switch.set_low()?;
        Ok(())
    }
}
