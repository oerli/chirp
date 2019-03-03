#![deny(warnings)]
#![no_std]

extern crate embedded_hal as hal;

use embedded_hal::blocking::i2c::{Write, WriteRead};

pub const DEFAULT_ADDRESS: u8 = 0x20;

enum Register {
    ChirpCapacitance = 0x00, // result: u16
    ChirpAddress = 0x01, // set new address
    //ChirpAddress = 0x02, // get address
    ChirpLightMessure = 0x03, // write: u8
    ChirpLight = 0x04, // result: u16
    ChirpTemperature = 0x05, // result: i16 / 10 (float)
    ChirpReset = 0x06, // write: u8
    ChirpVersion = 0x07,
    //ChirpSleep = 0x08,
    ChirpBusy = 0x09, // result u8 (1 = busy, 0 = idle)
}
pub struct Chirp<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C, E> Chirp<I2C> where I2C: WriteRead<Error = E> + Write<Error = E>, {
    pub fn new(i2c: I2C, address: u8) -> Self {
        Chirp { i2c, address }
    }
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    pub fn address(&mut self, address: u8) -> Result<(), E> {
        // TODO: check if address is bigger than 127 and trhow error
        // TODO: set address command twice?
        // TODO: check if update was successfull by reading address, new address might be available only after reboot?
        //if address > 127 {Err(e)}
        let result = self.i2c.write(self.address, &[Register::ChirpAddress as u8, address]);
        // TODO: only update new address when change was success fully?
        // must before address change
        self.reset()?;
        self.address = address;
        result
    }

    pub fn reset(&mut self) -> Result<(), E> {
        self.i2c.write(self.address, &[Register::ChirpReset as u8])
    }

    // start mussure for light, wait 3 seconds until reading light result
    pub fn messure(&mut self) -> Result<(), E> {
        self.i2c.write(self.address, &[Register::ChirpLightMessure as u8])
    }

    // check if busy
    pub fn busy(&mut self) -> Result<bool, E> {
        let mut buffer = [0u8; 1];
        self.i2c.write_read(self.address, &[Register::ChirpBusy as u8], &mut buffer)?;
        // better way to cast u8 to bool?
        if buffer[0] > 0 {Ok(true)} else {Ok(false)}
    }

    // get version, 0x26 means version 2.6
    pub fn version(&mut self) -> Result<u8, E> {
        let mut buffer = [0u8; 1];
        self.i2c.write_read(self.address, &[Register::ChirpVersion as u8], &mut buffer)?;
        Ok(buffer[0] as u8)
    }

    // read light, re-read after 3 seconds other wise previous result will be returned
    pub fn light(&mut self) -> Result<f32, E> {
        // create buffer of type u8 with value zero and length of two
        let mut buffer = [0u8; 2];
        self.i2c.write_read(self.address, &[Register::ChirpLight as u8], &mut buffer)?;
        Ok((((buffer[0] as u16) << 8 | buffer[1] as u16) as f32) / 10.0f32)
    }

    pub fn temperature(&mut self) -> Result<f32, E> {
        // create buffer of type u8 with value zero and length of two
        let mut buffer = [0u8; 2];
        self.i2c.write_read(self.address, &[Register::ChirpTemperature as u8], &mut buffer)?;
        Ok((((buffer[0] as u16) << 8 | buffer[1] as u16) as f32) /10.0f32)
    }

    pub fn capacitance(&mut self) -> Result<u16, E> {
        // create buffer of type u8 with value zero and length of two
        let mut buffer = [0u8; 2];
        self.i2c.write_read(self.address, &[Register::ChirpCapacitance as u8], &mut buffer)?;
        Ok((buffer[0] as u16) << 8 | buffer[1] as u16)
    }
}