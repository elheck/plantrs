use std::sync::Arc;
use core::time::Duration;
use anyhow::Result;

use embedded_svc::wifi::Wifi;

use esp_idf_svc::netif::EspNetifStack;
use esp_idf_svc::nvs::EspDefaultNvs;
use esp_idf_svc::sysloop::EspSysLoopStack;
use esp_idf_svc::wifi::EspWifi;

use embedded_svc::wifi::Configuration;
use embedded_svc::wifi::ClientConfiguration;
use embedded_svc::wifi::ClientConnectionStatus;
use embedded_svc::wifi::ClientStatus;
use embedded_svc::wifi::ClientIpStatus;

use crate::settings::SSID;
use crate::settings::WIFI_PW;


pub fn connect_wifi() -> Result<EspWifi>{
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
    println!("SSID: {0}, PW: {1}", SSID, WIFI_PW);

    wifi.set_configuration(config).unwrap();   

    wifi.wait_status_with_timeout(Duration::from_secs(10), |s| !s.is_transitional())
        .map_err(|e| anyhow::anyhow!("Wait timeout: {:?}", e)).unwrap();

    let status = wifi.get_status();
    
    if let ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(_,))) = status.0
    {
        Ok(wifi)
    } else {
        panic!("Could not connect to wifi")
    }
}