#![no_main]
#![no_std]

use panic_halt;

use cortex_m;
use microbit::hal::i2c;
use microbit::hal::i2c::I2c;
use microbit::hal::gpio::gpio::PIN;
use microbit::hal::gpio::gpio::{PIN4, PIN5, PIN6, PIN7, PIN8, PIN9, PIN10, PIN11, PIN12, PIN13, PIN14, PIN15};
use microbit::hal::gpio::{Output, PushPull};
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

type LED = PIN<Output<PushPull>>;

const DEFAULT_DELAY_MS: u32 = 2;
const LED_LAYOUT: [[(usize, usize); 5]; 5] = [
    [(0, 0), (1, 3), (0, 1), (1, 4), (0, 2)],
    [(2, 3), (2, 4), (2, 5), (2, 6), (2, 7)],
    [(1, 1), (0, 8), (1, 2), (2, 8), (1, 0)],
    [(0, 7), (0, 6), (0, 5), (0, 4), (0, 3)],
    [(2, 2), (1, 6), (2, 0), (1, 5), (2, 1)],
];

const NUMBER_1: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 1, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 1, 1, 0],
];
const NUMBER_2: [[u8; 5]; 5] = [
    [1, 1, 1, 0, 0],
    [0, 0, 0, 1, 0],
    [0, 1, 1, 0, 0],
    [1, 0, 0, 0, 0],
    [1, 1, 1, 1, 0],
];
const NUMBER_3: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 0],
    [0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [1, 0, 0, 1, 0],
    [0, 1, 1, 0, 0],
];
const NUMBER_4: [[u8; 5]; 5] = [
    [0, 0, 0, 1, 0],
    [0, 0, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [1, 1, 1, 1, 1],
    [0, 0, 0, 1, 0],
];
const NUMBER_5: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 0],
    [1, 1, 1, 0, 0],
    [0, 0, 0, 1, 0],
    [1, 1, 1, 0, 0],
];
const NUMBER_6: [[u8; 5]; 5] = [
    [0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];
const NUMBER_7: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 0],
    [0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0],
    [1, 0, 0, 0, 0],
];
const NUMBER_8: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];
const NUMBER_9: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [0, 0, 1, 0, 0],
    [0, 1, 1, 1, 0],
    [0, 1, 0, 0, 0],
];
const NUMBER_0: [[u8; 5]; 5] = [
    [0, 1, 1, 0, 0],
    [1, 0, 0, 1, 0],
    [1, 0, 1, 1, 0],
    [1, 1, 0, 1, 0],
    [0, 1, 1, 0, 0],
];

const LETTER_C: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 1, 1, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 0, 1, 1, 0],
];

const LETTER_L: [[u8; 5]; 5] = [
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 0, 1, 1, 0],
];

const LETTER_M: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [1, 1, 0, 1, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1],
];

const LETTER_E: [[u8; 5]; 5] = [
    [0, 1, 1, 0, 0],
    [1, 0, 0, 1, 0],
    [1, 1, 1, 0, 0],
    [1, 0, 0, 0, 0],
    [0, 1, 1, 0, 0],
];

const LETTER_R: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 1, 1, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
];

const SYMBOL_DOT: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0],
];
const SYMBOL_MINUS: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 1, 1, 1, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];
const SYMBOL_GRAD: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];
const SYMBOL_PERCENT: [[u8; 5]; 5] = [
    [1, 1, 0, 0, 1],
    [1, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 0, 1, 1],
    [1, 0, 0, 1, 1],
];
const SYMBOL_EMPTY: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

/// Array of all the LEDs in the 5x5 display on the board
pub struct Display {
    delay_ms: u32,
    rows: [LED; 3],
    cols: [LED; 9],
    text: [[u8; 50]; 5],
}

impl Display {
    /// Initializes all the user LEDs
    pub fn new(
        col1: PIN4<Output<PushPull>>,
        col2: PIN5<Output<PushPull>>,
        col3: PIN6<Output<PushPull>>,
        col4: PIN7<Output<PushPull>>,
        col5: PIN8<Output<PushPull>>,
        col6: PIN9<Output<PushPull>>,
        col7: PIN10<Output<PushPull>>,
        col8: PIN11<Output<PushPull>>,
        col9: PIN12<Output<PushPull>>,
        row1: PIN13<Output<PushPull>>,
        row2: PIN14<Output<PushPull>>,
        row3: PIN15<Output<PushPull>>,
    ) -> Self {
        let mut retval = Display {
            delay_ms: DEFAULT_DELAY_MS,
            rows: [row1.downgrade(), row2.downgrade(), row3.downgrade()],
            cols: [
                col1.downgrade(), col2.downgrade(), col3.downgrade(),
                col4.downgrade(), col5.downgrade(), col6.downgrade(),
                col7.downgrade(), col8.downgrade(), col9.downgrade()
            ],
            text: [
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ],
        };
        // This is needed to reduce flickering on reset
        retval.clear();
        retval
    }

    /// Clear display
    pub fn clear(&mut self) {
        for row in &mut self.rows {
            row.set_low();
        }
        for col in &mut self.cols {
            col.set_high();
        }
    }

    /// Convert 5x5 display image to 3x9 matrix image
    pub fn display2matrix(led_display: [[u8; 5]; 5]) -> [[u8; 9]; 3] {
        let mut led_matrix: [[u8; 9]; 3] = [[0; 9]; 3];
        for (led_display_row, layout_row) in led_display.iter().zip(LED_LAYOUT.iter()) {
            for (led_display_val, layout_loc) in led_display_row.iter().zip(layout_row) {
                led_matrix[layout_loc.0][layout_loc.1] = *led_display_val;
            }
        }
        led_matrix
    }

    /// Display 5x5 display image for a given duration
    pub fn display(&mut self, delay: &mut Delay, led_display: [[u8; 5]; 5], duration_ms: u32) {
        let led_matrix = Display::display2matrix(led_display);
        // Calculates how long to block for
        // e.g. If the duration_ms is 500ms (half a second)
        //      and self.delay_ms is 2ms (about 2ms per scan row),
        //      each refresh takes 3rows×2ms, so we need 500ms / (3×2ms) loops.
        let loops = duration_ms / (self.rows.len() as u32 * self.delay_ms);
        for _ in 0..loops {
            for (row_line, led_matrix_row) in self.rows.iter_mut().zip(led_matrix.iter()) {
                row_line.set_high();
                for (col_line, led_matrix_val) in self.cols.iter_mut().zip(led_matrix_row.iter()) {
                    // We are keeping it simple, and not adding brightness
                    if *led_matrix_val > 0 {
                        col_line.set_low();
                    }
                }
                delay.delay_ms(self.delay_ms);
                for col_line in &mut self.cols {
                    col_line.set_high();
                }
                row_line.set_low();
            }
        }
    }

    pub fn display_temperature(&mut self, delay: &mut Delay,  temperature: f32, duration_ms: u32) {

        let mut character = SYMBOL_EMPTY;
        self.add_character(character, 5, 0);

        if temperature < 0.0 {
            self.add_character(SYMBOL_MINUS, 5, 0);
        }

        character = Display::convert((temperature / 10.0) as u8);
        self.add_character(character, 5, 5);
        // for row in 0..5 {
        //     for column in 0..5 {
        //         text[row][column+5] = character[row][column];
        //     }
        // }
        // self.display(delay, display_text, duration_ms);
        // self.display(delay, SYMBOL_EMPTY, 100);
        character = Display::convert((temperature % 10.0) as u8);
        self.add_character(character, 5, 10);
        // for row in 0..5 {
        //     for column in 0..5 {
        //         text[row][column+10] = character[row][column];
        //     }
        // }
        // self.display(delay, display_text, duration_ms);
        // self.display(delay, SYMBOL_EMPTY, 100);
        // self.display(delay, SYMBOL_DOT, duration_ms);
        self.add_character(SYMBOL_DOT, 3, 15);
        // for row in 0..5 {
        //     for column in 0..3 {
        //         text[row][column+15] = SYMBOL_DOT[row][column];
        //     }
        // }
        // self.display(delay, SYMBOL_EMPTY, 100);
        character = Display::convert((temperature * 10.0 % 10.0) as u8);
        self.add_character(character, 5, 18);
        // for row in 0..5 {
        //     for column in 0..5 {
        //         text[row][column+18] = character[row][column];
        //     }
        // }
        self.add_character(SYMBOL_GRAD, 4, 23);
        // for row in 0..5 {
        //     for column in 0..5 {
        //         text[row][column+23] = SYMBOL_GRAD[row][column];
        //     }
        // }
        self.add_character(LETTER_C, 5, 27);
        // for row in 0..5 {
        //     for column in 0..5 {
        //         text[row][column+28] = LETTER_C[row][column];
        //     }
        // }
        self.add_character(SYMBOL_EMPTY, 5, 32);
        
        self.display_text(delay, self.text, 37);
        // self.display(delay, display_text, duration_ms);
        // self.display(delay, SYMBOL_EMPTY, 100);
        // self.display(delay, LETTER_C, duration_ms);
        // let display_text: Vec<[[u8;5];5]> = Vec::new();

        // let text = format!("{:.1}", number);
        // for character in format!("{:.1}", number).chars() {
            
        // }

        // display_text
    }

    pub fn display_capacitance(&mut self, delay: &mut Delay,  capacitance: u16, duration_ms: u32) {
        let mut character = SYMBOL_EMPTY;
        self.add_character(character, 5, 0);

        if capacitance > 1000 {
            let thousender = capacitance / 1000;
            character = Display::convert(thousender as u8);
            self.add_character(character, 5, 5);

            let hundreder = (capacitance - (thousender * 1000)) / 100;
            character = Display::convert((hundreder) as u8);
            self.add_character(character, 5, 10);

            let tener = (capacitance - (thousender * 1000) - (hundreder * 100)) / 10;
            character = Display::convert((tener) as u8);
            self.add_character(character, 5, 15);

            character = Display::convert((capacitance % 10) as u8);
            self.add_character(character, 5, 20);

            self.add_character(SYMBOL_PERCENT, 5, 25);
            self.add_character(SYMBOL_EMPTY, 5, 30);

            self.display_text(delay, self.text, 35);
        } else if capacitance > 100 {
 
            let hundreder = capacitance / 100;
            character = Display::convert((hundreder) as u8);
            self.add_character(character, 5, 5);

            let tener = (capacitance - (hundreder * 100)) / 10;
            character = Display::convert((tener) as u8);
            self.add_character(character, 5, 10);

            character = Display::convert((capacitance % 10) as u8);
            self.add_character(character, 5, 15);

            self.add_character(SYMBOL_PERCENT, 5, 20);
            self.add_character(SYMBOL_EMPTY, 5, 25);

            self.display_text(delay, self.text, 30);

        } else if capacitance > 10 {
            let tener = capacitance / 10;
            character = Display::convert((tener) as u8);
            self.add_character(character, 5, 5);

            character = Display::convert((capacitance % 10) as u8);
            self.add_character(character, 5, 10);

            self.add_character(SYMBOL_PERCENT, 5, 15);
            self.add_character(SYMBOL_EMPTY, 5, 20);

            self.display_text(delay, self.text, 25);

        } else {
            self.add_character(Display::convert(capacitance as u8), 5, 5);

            self.add_character(SYMBOL_PERCENT, 5, 10);
            self.add_character(SYMBOL_EMPTY, 5, 15);

            self.display_text(delay, self.text, 20);
        }
    }

    pub fn display_light(&mut self, delay: &mut Delay,  light: u16, duration_ms: u32) {
        let mut character = SYMBOL_EMPTY;
        self.add_character(character, 5, 0);

        if light > 1000 {
            let thousender = light / 1000;
            character = Display::convert(thousender as u8);
            self.add_character(character, 5, 5);

            let hundreder = (light - (thousender * 1000)) / 100;
            character = Display::convert((hundreder) as u8);
            self.add_character(character, 5, 10);

            let tener = (light - (thousender * 1000) - (hundreder * 100)) / 10;
            character = Display::convert((tener) as u8);
            self.add_character(character, 5, 15);

            character = Display::convert((light % 10) as u8);
            self.add_character(character, 5, 20);

            self.add_character(LETTER_L, 5, 25);
            self.add_character(LETTER_M, 5, 30);
            self.add_character(SYMBOL_EMPTY, 5, 35);

            self.display_text(delay, self.text, 40);
        } else if light > 100 {
 
            let hundreder = light / 100;
            character = Display::convert((hundreder) as u8);
            self.add_character(character, 5, 5);

            let tener = (light - (hundreder * 100)) / 10;
            character = Display::convert((tener) as u8);
            self.add_character(character, 5, 10);

            character = Display::convert((light % 10) as u8);
            self.add_character(character, 5, 15);

            self.add_character(LETTER_L, 5, 20);
            self.add_character(LETTER_M, 5, 25);
            self.add_character(SYMBOL_EMPTY, 5, 30);

            self.display_text(delay, self.text, 35);

        } else if light > 10 {
            let tener = light / 10;
            character = Display::convert((tener) as u8);
            self.add_character(character, 5, 5);

            character = Display::convert((light % 10) as u8);
            self.add_character(character, 5, 10);

            self.add_character(LETTER_L, 5, 15);
            self.add_character(LETTER_M, 5, 20);
            self.add_character(SYMBOL_EMPTY, 5, 25);

            self.display_text(delay, self.text, 30);

        } else {
            self.add_character(Display::convert(light as u8), 5, 5);

            self.add_character(LETTER_L, 5, 10);
            self.add_character(LETTER_M, 5, 15);
            self.add_character(SYMBOL_EMPTY, 5, 20);

            self.display_text(delay, self.text, 25);
        }
    }

    fn add_character(&mut self, character: [[u8; 5]; 5], character_length: usize, position: usize) {
        for row in 0..5 {
            for column in 0..character_length {
                self.text[row][column+position] = character[row][column];
            }
        }
    }

    fn display_text(&mut self, delay: &mut Delay, text: [[u8; 5 * 10]; 5], length: u8) {
        let mut display = [
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ];

        for position in 0..length-5 {
            for row in 0..5 {
                for column in 0..5 {
                    display[row][column] = text[row][column + position as usize];
                }
            }
            self.display(delay, display, 200);
        }
    }

    fn convert(character: u8) -> [[u8; 5]; 5] {
        match character {
            0 => NUMBER_0,
            1 => NUMBER_1,
            2 => NUMBER_2,
            3 => NUMBER_3,
            4 => NUMBER_4,
            5 => NUMBER_5,
            6 => NUMBER_6,
            7 => NUMBER_7,
            8 => NUMBER_8,
            9 => NUMBER_9,
            _ => SYMBOL_EMPTY,
        }
    }
}


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

            let mut chirp = Chirp::new(i2c, 0x23);

            // Reset the Chirp Sensor to initialize correctly
            chirp.reset();
            delay.delay_ms(500_u32);

            // Change Chirp Sensor address
            // write!(&mut tx, "Change Address to 0x21");
            // chirp.address(0x21);
            // delay.delay_ms(100_u32);

            let version = match chirp.version() {
                    Result::Ok(version) => version,
                    Result::Err(error) => {
                        write!(&mut tx, "Error: {:?}\n\r", error);
                        // scan all i2c devices
                        for address in 0x01..0x7F {
                            chirp = Chirp::new(chirp.destroy(), address);
                            chirp.reset();
                            delay.delay_ms(500_u32);
                            let version = match chirp.version() {
                                Result::Ok(version) => write!(&mut tx, "Info: 0x{:x} found version {}\n\r", address, version),
                                Result::Err(error) => { write!(&mut tx, "Error: 0x{:x} {:?}\n\r", address, error); continue; }
                            };
                        }
                        loop {
                            continue;
                        };
                    }
                };
            write!(&mut tx, "Version: {}\n\r", version);

            // Display pins
            let row1 = gpio.pin13.into_push_pull_output();
            let row2 = gpio.pin14.into_push_pull_output();
            let row3 = gpio.pin15.into_push_pull_output();
            let col1 = gpio.pin4.into_push_pull_output();
            let col2 = gpio.pin5.into_push_pull_output();
            let col3 = gpio.pin6.into_push_pull_output();
            let col4 = gpio.pin7.into_push_pull_output();
            let col5 = gpio.pin8.into_push_pull_output();
            let col6 = gpio.pin9.into_push_pull_output();
            let col7 = gpio.pin10.into_push_pull_output();
            let col8 = gpio.pin11.into_push_pull_output();
            let col9 = gpio.pin12.into_push_pull_output();
            
            let mut leds = Display::new(
                col1, col2, col3,
                col4, col5, col6,
                col7, col8, col9,
                row1, row2, row3,
            );

            let mut current_display = [
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ];
            
            // Start messure the sensor so it's ready for reading
            chirp.messure();
            // delay.delay_ms(100_u32);
            while chirp.busy().unwrap() {
                delay.delay_ms(100_u32);
                write!(&mut tx, ".");
            }

            loop {
                // current_display = NUMBER_0;

                // leds.display(&mut delay, current_display, 100);

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
                leds.display_temperature(&mut delay, temperature, 1500);
                write!(&mut tx, "Temperature: {}\n\r", temperature);
                // delay.delay_ms(1000_u32);

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
                leds.display_capacitance(&mut delay, capacitance, 1500);
                write!(&mut tx, "Capacitance: {}\n\r", capacitance);
                // delay.delay_ms(1000_u32);

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
                leds.display_light(&mut delay, light as u16, 1500);
                write!(&mut tx, "Light: {}\n\r", light);
                
            }       
            
        });
    }

    loop {
        continue;
    }
}
