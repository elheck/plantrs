#![no_std]
#![no_main]


use bsp::entry;
use defmt::{info, panic};
use defmt_rtt as _;
use panic_probe as _;

use rp_pico as bsp;
use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

mod pump;
use crate::pump::Pump;

mod dht;
use crate::dht::Dht11;


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

    let led_pin = pins.led.into_push_pull_output();
    let pump_pin = pins.gpio4.into_push_pull_output();
    let dht_pin = pins.gpio3.into_readable_output();

    let mut pump = Pump::new(led_pin.into(), pump_pin.into());
    let mut dht = Dht11::new(dht_pin.into());

    loop {
        dht.read(&mut delay).unwrap();
        match pump.turn_on(){
            Ok(_) => info!("Pump on"),
            Err(_) => panic!("Could not turn on pump")
        };
        delay.delay_ms(1000);
        match pump.turn_off(){
            Ok(_) => info!("Pump off"),
            Err(_) => panic!("Could not turn off pump")
        };
        delay.delay_ms(1000);
    }
}

// End of file
