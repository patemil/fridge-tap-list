use embedded_hal::blocking::i2c::{Write, WriteRead};

const ADDR: u8 =        0b0011000;

pub struct GreenPAK<I> {
    device: I,
}

impl<I: Write + WriteRead> GreenPAK<I> {
    /// Create device driver instance.
    pub fn new(i2c: I) -> Self {
        GreenPAK { device: i2c }
    }

    pub fn virtual_input(&mut self, byte: u8, mask: u8) -> Result<(), <I as Write>::Error> {
        // Any bit in the mask that is set to “1” in the I2C Byte Write Mask Register will mask
        // the effect of changing that particular bit in the target register, during the next Byte
        // Write Command.
        self.device.write(ADDR, &[0xC9, mask])?;
        self.device.write(ADDR, &[0x7A, byte])?;
        Ok(())
    }

    pub fn write_cnt0(&mut self, value: u16) -> Result<(), <I as Write>::Error> {
        self.device.write(ADDR, &[0xA6, (value >> 8) as u8]);
        self.device.write(ADDR, &[0xA5, (value as u8)])
    }

    pub fn write_cnt2(&mut self, byte: u8) -> Result<(), <I as Write>::Error> {
        self.device.write(ADDR, &[0xAF, byte])
    }
}
