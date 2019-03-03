#![no_main]
#![no_std]

use panic_halt;

use cortex_m;
use microbit::hal::i2c;
use microbit::hal::i2c::I2c;
use microbit::hal::nrf51::{UART0, GPIOTE};
use microbit::hal::prelude::*;
use microbit::hal::serial;
use microbit::hal::serial::BAUD115200;
use microbit::TWI1;
use microbit::hal::delay::Delay;

use crate::cortex_m::interrupt::Mutex;
use crate::cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;

use chirp::{Chirp, DEFAULT_ADDRESS};

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;

static GPIO: Mutex<RefCell<Option<GPIOTE>>> = Mutex::new(RefCell::new(None));
static TX: Mutex<RefCell<Option<serial::Tx<UART0>>>> = Mutex::new(RefCell::new(None));
static CHIRP: Mutex<RefCell<Option<Chirp<I2c<TWI1>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let (Some(p), Some(mut cp)) = (microbit::Peripherals::take(), Peripherals::take()) {
        cortex_m::interrupt::free(move |cs| {
            let mut delay = Delay::new(p.TIMER0);

            // Split GPIO pins
            let gpio = p.GPIO.split();

            // Configure RX and TX pins accordingly
            let tx = gpio.pin24.into_push_pull_output().downgrade();
            let rx = gpio.pin25.into_floating_input().downgrade();

            // Set up serial port using the prepared pins
            let (mut tx, _) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();
            let _ = write!(&mut tx, "n\rSetting up Chirp Sensor...!\n\r");

            // Configure SCL and SDA pins accordingly
            let scl = gpio.pin0.into_open_drain_input().downgrade();
            let sda = gpio.pin30.into_open_drain_input().downgrade();

            // Set up I2C
            let mut i2c = i2c::I2c::i2c1(p.TWI1, sda, scl);

            let mut chirp = Chirp::new(i2c, chirp::DEFAULT_ADDRESS);
            
            // Reset the Chirp Sensor to initialize correctly
            chirp.reset();
            delay.delay_ms(100_u32);

            // Change Chirp Sensor address
            // write!(&mut tx, "Change Address to 0x21");
            // chirp.address(0x21);
            // delay.delay_ms(100_u32);
            
            let version = match chirp.version() {
                    Result::Ok(version) => version,
                    Result::Err(error) => {
                        write!(&mut tx, "Error: {:?}\n\r", error);
                        loop {
                            continue;
                        };
                    }
                };
            write!(&mut tx, "Version: {}\n\r", version);
            
            loop {          
                // Start messure the sensor so it's ready for reading
                chirp.messure();
                delay.delay_ms(1000_u32);

                // Read Temperature
                let temperature = match chirp.temperature() {
                    Result::Ok(temperature) => temperature,
                    Result::Err(error) => {
                        write!(&mut tx, "Error: {:?}\n\r", error);
                        loop {
                            continue;
                        };
                    }
                };
                write!(&mut tx, "Temperature: {}\n\r", temperature);
                delay.delay_ms(1000_u32);

                // Read Capacitance
                let capacitance = match chirp.capacitance() {
                    Result::Ok(capacitance) => capacitance,
                    Result::Err(error) => {
                        write!(&mut tx, "Error: {:?}\n\r", error);
                        loop {
                            continue;
                        };
                    }
                };
                write!(&mut tx, "Capacitance: {}\n\r", capacitance);
                delay.delay_ms(1000_u32);

                // Read Light intensity
                let light = match chirp.light() {
                    Result::Ok(light) => light,
                    Result::Err(error) => {
                        write!(&mut tx, "Error: {:?}\n\r", error);
                        loop {
                            continue;
                        };
                    }
                };
                write!(&mut tx, "Light: {}\n\r", light);
            }       
            
        });
    }

    loop {
        continue;
    }
}
