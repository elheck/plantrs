
use esp_idf_hal::gpio::{Gpio32, Output};
use esp_idf_svc::wifi::EspWifi;
use esp_idf_hal::delay::FreeRtos;
use embedded_hal::blocking::delay::DelayMs;
use esp_idf_hal::peripherals::Peripherals;

// This is a config file that defines the following private parameter constants
mod settings;
mod wifi;
use crate::wifi::connect_wifi;

mod pin_switch;
use crate::pin_switch::{Switch, Togglable};

fn main() {
    let _wifi:EspWifi = connect_wifi().unwrap();

    esp_idf_svc::log::EspLogger::initialize_default();
    let pin2 = Peripherals::take().unwrap().pins.gpio32.into_output().unwrap();
    let mut switch: Switch<Gpio32<Output>> = Switch::new(pin2);

    let mut delay = FreeRtos;
    loop{
        switch.toggle().unwrap();
        log::info!("Pin State: {:?}", switch.current_state);
        delay.delay_ms(2000 as u32);
    }
}
