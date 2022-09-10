use core::result::Result;
use esp_idf_hal::rmt::PinState;
use embedded_hal::{digital::v2::OutputPin};



pub trait Togglable{    
    type Error;
    fn switch_on(& mut self) -> Result<(), Self::Error>;    
    fn switch_off(& mut self) -> Result<(), Self::Error>;
    fn toggle(& mut self) -> Result<(), Self::Error>;
}
pub struct Switch<Gpio: OutputPin>{
    pub current_state: PinState,
    pub pin: Gpio,
}

impl<Gpio: OutputPin> Switch<Gpio> {
    pub fn new(pin: Gpio)-> Switch<Gpio>{
        Switch{current_state: PinState::Low, pin}
    }    
}

impl<Gpio: OutputPin> Togglable for Switch<Gpio>{
    type Error = <Gpio as OutputPin>::Error;
    fn switch_off(&mut self) -> Result<(), Self::Error> {
        self.current_state = PinState::Low;
        self.pin.set_high()
    }

    fn switch_on(& mut self) -> Result<(), Self::Error> {
        self.current_state = PinState::High;
        self.pin.set_low()
    }

    fn toggle(& mut self) -> Result<(), Self::Error> {
        match self.current_state{
            PinState::High => self.switch_off(),
            PinState::Low => self.switch_on()
        }
    }
}