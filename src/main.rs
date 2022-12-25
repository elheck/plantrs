#![no_std]
#![no_main]

use bsp::{entry};
use defmt::{info, panic};
use defmt_rtt as _;
use panic_probe as _;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    adc::Adc,
    watchdog::Watchdog,
};
use rp_pico as bsp;

mod water_pump;
use crate::water_pump::WaterPump;

mod dht;
use crate::dht::Dht11;

mod analog_ph_meter;
use crate::analog_ph_meter::PhProbe;

mod soil_moisture;
use crate::soil_moisture::SoilMoistureSensor;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut peripherals = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(peripherals.WATCHDOG);
    let sio = Sio::new(peripherals.SIO);

    let clocks = init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        peripherals.XOSC,
        peripherals.CLOCKS,
        peripherals.PLL_SYS,
        peripherals.PLL_USB,
        &mut peripherals.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    let mut adc = Adc::new(peripherals.ADC, &mut peripherals.RESETS);

    let led_pin = pins.led.into_push_pull_output();
    let water_pump_pin = pins.gpio4.into_push_pull_output();
    let dht_pin = pins.gpio3.into_readable_output();
    let ph_pin = pins.gpio26.into_floating_input();
    let soil_moisture_power_pin = pins.gpio6.into_push_pull_output();
    let soil_moisture_read_pin = pins.gpio27.into_floating_input();

    let mut water_pump = WaterPump::new(led_pin.into(), water_pump_pin.into());
    let mut dht = Dht11::new(dht_pin.into());
    let mut ph_meter = PhProbe::new(ph_pin);
    let mut soil_moisture_sensor = SoilMoistureSensor::new(soil_moisture_read_pin, soil_moisture_power_pin.into());

    loop {
        let measurement = dht.read(&mut delay).unwrap();
        info!("Humidity: {}, Temp: {}\n", measurement.temperature, measurement.relative_humidity);
        let ph = ph_meter.read(& mut adc);
        info!("Ph is currently {}\n", ph);
        let moisture = soil_moisture_sensor.read_blocking(&mut adc, &mut delay);
        info!("Soil moisture analog value is currently {}\n", moisture);
        match water_pump.turn_on() {
            Ok(_) => info!("Pump on"),
            Err(_) => panic!("Could not turn on pump"),
        };
        delay.delay_ms(1000);
        match water_pump.turn_off() {
            Ok(_) => info!("Pump off"),
            Err(_) => panic!("Could not turn off pump"),
        };
        delay.delay_ms(1000);
    }
}

// End of file
