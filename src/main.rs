#![no_std]
#![no_main]

use defmt::{info, panic};
use defmt_rtt as _;
use panic_probe as _;
use rp_pico::entry;

use embedded_hal::digital::v2::OutputPin;
use rp_pico::hal::{
    adc::Adc,
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

mod water_pump;
use crate::water_pump::WaterPump;

mod dht;
use crate::dht::Dht11;

mod analog_ph_meter;
use crate::analog_ph_meter::PhProbe;

//mod tmc429;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut peripherals = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(peripherals.WATCHDOG);
    let sio = Sio::new(peripherals.SIO);

    let clocks = init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
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

    let pins = rp_pico::Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    let mut adc = Adc::new(peripherals.ADC, &mut peripherals.RESETS);

    let mut water_pump = WaterPump::new(
        pins.gpio7.into_push_pull_output(),
        pins.gpio4.into_push_pull_output(),
    );
    let mut inside_dht = Dht11::new(pins.gpio15.into());
    let mut outside_dht = Dht11::new(pins.gpio18.into());
    let mut ph_meter = PhProbe::new(pins.gpio28.into_floating_input());
    let mut onboard_led = pins.led.into_push_pull_output();

    onboard_led.set_high().unwrap();
    loop {
        let measurement_inside = inside_dht.read(&mut delay).unwrap();
        info!(
            "Inside Humidity: {}, Temp: {}\n",
            measurement_inside.temperature, measurement_inside.relative_humidity
        );
        let measurement_outside = outside_dht.read(&mut delay).unwrap();
        info!(
            "Outside Humidity: {}, Temp: {}\n",
            measurement_outside.temperature, measurement_outside.relative_humidity
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
