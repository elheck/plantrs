
use embedded_hal::digital::v2::OutputPin;
use rp_pico::hal::{Spi, spi::{Enabled, SpiDevice}};

pub enum Error{
    ChipSelectError,
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


pub struct TMC429<D, CS, const DS: u8> 
where
    D: SpiDevice,
    CS: OutputPin,
    CS::Error: core::fmt::Debug

{
    spi: Spi<Enabled, D, DS>,
    chip_select: CS,
    motors: [MotorConfig; 3]
    
}

impl <D, CS, const DS: u8> TMC429<D, CS, DS> 
where
    D: SpiDevice,
    CS: OutputPin,
    CS::Error: core::fmt::Debug
{
    pub fn new(chip_select_pin: CS, spi: Spi<Enabled, D, DS>) ->Self{
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
            motors
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

    fn chip_select_high(&mut self) -> Result<(), Error>{
        self.chip_select.set_high().map_err(|_| Error::ChipSelectError)
    }

    fn chip_select_low(&mut self) -> Result<(), Error>{
        self.chip_select.set_low().map_err(|_| Error::ChipSelectError)
    }

    pub fn init(&mut self){
    }
}