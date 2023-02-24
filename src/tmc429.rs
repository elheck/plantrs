
use embedded_hal::digital::v2::OutputPin;
use rp_pico::hal::{Spi, spi::{Enabled, SpiDevice}};
use embedded_hal::spi::FullDuplex;

const CLOCK_FREQUENCY_MHZ_MAX: u8 = 32;

pub enum Error{
    ChipSelectError,
    ClockFrequencyError,
}

#[derive(PartialEq, Eq)]
pub enum MotorNumber {
    Motor1,
    Motor2,
    Motor3
}

pub enum MotorState{
    Active,
    Inactive
}

pub struct MotorConfig{
    active: MotorState,
    which_motor: MotorNumber
}

impl Default for MotorConfig{
    fn default() -> Self {
        MotorConfig { 
            active: MotorState::Inactive,
            which_motor: MotorNumber::Motor1,
        }
    }
}

pub struct TMC429<CS, T> 
where
    CS: OutputPin,
    CS::Error: core::fmt::Debug,
    T: FullDuplex<u8>
{
    spi: T,
    chip_select: CS,
    motors: [MotorConfig; 3],
    clock_frequency: u8
    
}

impl <CS, T> TMC429<CS, T>
where
    CS: OutputPin,
    CS::Error: core::fmt::Debug,
    T: FullDuplex<u8>
{
    pub fn new(chip_select_pin: CS, spi: T) ->Self{
        let motors = [
            MotorConfig{
                ..Default::default()
            }, 
            MotorConfig{
                which_motor: MotorNumber::Motor2,
                ..Default::default()
            },
            MotorConfig{
                which_motor: MotorNumber::Motor3,
                ..Default::default()
            },
        ];

        TMC429{
            spi, 
            chip_select: chip_select_pin, 
            motors,
            clock_frequency: CLOCK_FREQUENCY_MHZ_MAX
        }
    }
    
    pub fn set_motor_config(&mut self, config: MotorConfig, motor: MotorNumber) -> Result<(), & 'static str>{
        for i in 0..self.motors.len(){
            if self.motors[i].which_motor == motor{
                self.motors[i] = config;
                return Ok(());
            }
        }
        Err("Couldn't find motor")
    }

    pub fn init(&mut self, clock_fequency_in_mhz: u8) -> Result<(), Error>{
        self.set_clock_frequency(clock_fequency_in_mhz)?;
        Ok(())
    }   

    fn set_clock_frequency(&mut self, clock_frequency_in_mhz: u8) -> Result<(), Error>{
        if clock_frequency_in_mhz > CLOCK_FREQUENCY_MHZ_MAX{
            self.clock_frequency = CLOCK_FREQUENCY_MHZ_MAX;
            return Err(Error::ClockFrequencyError);
        }
        self.clock_frequency = clock_frequency_in_mhz;
        Ok(()) 
    }

    fn chip_select_high(&mut self) -> Result<(), Error>{
        self.chip_select.set_high().map_err(|_| Error::ChipSelectError)
    }

    fn chip_select_low(&mut self) -> Result<(), Error>{
        self.chip_select.set_low().map_err(|_| Error::ChipSelectError)
    }
}