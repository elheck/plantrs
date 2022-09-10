use esp_idf_svc::wifi::EspWifi;
// This is a config file that defines the following private parameter constants
mod settings;
mod wifi;
use crate::wifi::connect_wifi;


fn main() {
    let _wifi:EspWifi = connect_wifi().unwrap();
    println!("Hello, world!");
}
