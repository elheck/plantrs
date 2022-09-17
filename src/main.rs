use std::thread;
use std::time;
use esp_idf_svc::wifi::EspWifi;
use esp_idf_hal::peripherals::Peripherals;


// This is a config file that defines the following private parameter constants
mod settings;
mod dht;

mod pump;
use crate::pump::Pump;

mod wifi;
use crate::wifi::connect_wifi;

mod pin_switch;
use crate::pin_switch::Switch;

fn main() {
    let _wifi:EspWifi = connect_wifi().unwrap();

    esp_idf_svc::log::EspLogger::initialize_default();
    let pin2 = Peripherals::take().unwrap().pins.gpio2.into_output().unwrap();
    let pin17 = Peripherals::take().unwrap().pins.gpio17.into_output().unwrap();
    let led_switch = Switch::new(pin2);
    let pump_switch = Switch::new(pin17);  
    let mut pump = Pump::new(led_switch, pump_switch);

    loop{
        pump.turn_on_blocking(time::Duration::from_millis(1000));
        thread::sleep(time::Duration::from_millis(1000));
    }
}
