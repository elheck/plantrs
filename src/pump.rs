
use rp_pico::{hal::{gpio::{Output, PushPull, PinId, Pin, pin::{Disabled}, PushPullOutput, DynPinId, PullDown}}};

pub struct Pump{
    indicator: Pin<dyn PinId<Reset = Disabled<PullDown>>, PushPullOutput>,
    pump_switch: Pin<dyn PinId<Reset = Disabled<PullDown>>, PushPullOutput>
}

impl Pump{
    pub fn new<IndicatorGpioId: PinId, PumpGpioId: PinId> (indicator: IndicatorGpioId, pump_switch: PumpGpioId) -> Self{
        Pump { indicator, pump_switch }
    }
}