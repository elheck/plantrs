use std::time;
use std::thread::sleep;
use core::result::Result::Ok;

use crate::pin_switch::Togglable;


pub struct Pump<LedSwitch: Togglable, PumpSwitch: Togglable>{
    indicator_led: LedSwitch,
    pump_pin: PumpSwitch
}

impl<LedSwitch: Togglable, PumpSwitch: Togglable> Pump<LedSwitch, PumpSwitch> {
    pub fn new(indicator_led: LedSwitch, pump_pin: PumpSwitch) -> Self{
        Pump{indicator_led, pump_pin}
    }

    pub fn turn_on_blocking(& mut self, duration: time::Duration){
        self.turn_on();
        sleep(duration);
        self.turn_off();
    }

    pub fn turn_on(& mut self) {
        match self.pump_pin.switch_on() {
            Ok(_) => {
                match self.indicator_led.switch_on() {
                    Ok(_) => log::info!("Pump on"),
                    Err(_) => log::error!("Could not switch on led"),
                }
            },
            Err(_) => log::error!("Could not turn on pump"),
        }
    }

    pub fn turn_off(& mut self){
        match self.pump_pin.switch_off() {
            Ok(_) => {
                match self.indicator_led.switch_off() {
                    Ok(_) => log::info!("Pump off"),
                    Err(_) => log::error!("Could not switch off led"),
                }
            },
            Err(_) => log::error!("Could not turn off pump"),
        }
    }

}