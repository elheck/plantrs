#![no_std]
#![no_main]

use bsp::entry;
use defmt::{info, panic};
use defmt_rtt as _;
use panic_probe as _;

use bsp::hal::{
    adc::Adc,
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use rp_pico as bsp;

mod water_pump;
use crate::water_pump::WaterPump;

mod dht;
use crate::dht::Dht11;

mod analog_ph_meter;
use crate::analog_ph_meter::PhProbe;

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

    let mut water_pump = WaterPump::new(pins.led.into(), pins.gpio4.into());
    let mut dht = Dht11::new(pins.gpio3.into());
    let mut ph_meter = PhProbe::new(pins.gpio26.into_floating_input());

    loop {
        let measurement = dht.read(&mut delay).unwrap();
        info!(
            "Humidity: {}, Temp: {}\n",
            measurement.temperature, measurement.relative_humidity
        );
        let ph = ph_meter.read(&mut adc);
        info!("Ph is currently {}\n", ph);
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
