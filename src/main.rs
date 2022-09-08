use embedded_svc::wifi::Wifi;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_svc::netif::EspNetifStack;
use esp_idf_svc::nvs::EspDefaultNvs;
use esp_idf_svc::sysloop::EspSysLoopStack;
use esp_idf_svc::wifi::EspWifi;

use embedded_svc::wifi::Configuration;
use embedded_svc::wifi::ClientConfiguration;

mod settings;

// This is a config file that defines the following private parameter constants
use crate::settings::SSID;
use crate::settings::WIFI_PW;

use std::sync::Arc;

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();
    let netif_stack = Arc::new(EspNetifStack::new().unwrap());
    let loop_stack = Arc::new(EspSysLoopStack::new().unwrap());
    let nvs = Arc::new(EspDefaultNvs::new().unwrap());
    let mut wifi = match EspWifi::new(netif_stack, loop_stack, nvs) {
        Ok(wifi) => wifi,
        Err(error) => panic!("Wifi not created because of this error: {0}", error),
    };

    let config = &Configuration::Client(ClientConfiguration {
        ssid: SSID.into(),
        password: WIFI_PW.into(),
        ..Default::default()
    });
    wifi.set_configuration(config).unwrap();   
    
    println!("Hello, world!");
}
