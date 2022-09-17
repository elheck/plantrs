use core::result::Result;
use esp_idf_hal::rmt::PinState;
use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin};



pub trait Togglable{    
    type Error;
    fn switch_on(& mut self) -> Result<(), Self::Error>;    
    fn switch_off(& mut self) -> Result<(), Self::Error>;
    fn toggle(& mut self) -> Result<(), Self::Error>;
}
pub struct Switch<Gpio: OutputPin + StatefulOutputPin>{
    pub current_state: PinState,
    pub pin: Gpio,
}

impl<Gpio: OutputPin + StatefulOutputPin> Switch<Gpio> {
    pub fn new(pin: Gpio)-> Self{
        Switch{current_state: PinState::Low, pin}
    }    
}

impl<Gpio: OutputPin + StatefulOutputPin> Togglable for Switch<Gpio>{
    type Error = <Gpio as OutputPin>::Error;
    fn switch_off(&mut self) -> Result<(), Self::Error> {
        let setting_result = self.pin.set_low();
        self.current_state = match self.pin.is_set_high(){
            Ok(state) => {
                match state {
                    true => PinState::High,
                    false => PinState::Low,
                }
            },
            Err(_)=> panic!("Couldn't read state")
        };
        setting_result
    }

    fn switch_on(& mut self) -> Result<(), Self::Error> {
        let setting_result = self.pin.set_high();
        self.current_state = match self.pin.is_set_high(){
            Ok(state) => {
                match state {
                    true => PinState::High,
                    false => PinState::Low,
                }
            },
            Err(_)=> panic!("Couldn't read state")
        };
        setting_result
    }

    fn toggle(& mut self) -> Result<(), Self::Error> {
        match self.current_state{
            PinState::High => self.switch_off(),
            PinState::Low => self.switch_on()
        }
    }
}