use embedded_hal::blocking::i2c::{Write, WriteRead};

const ADDR: u8 = 0b0001000;

pub struct GreenPAK<I> {
    device: I,
}

impl<I: Write + WriteRead> GreenPAK<I> {
    /// Create device driver instance.
    pub fn new(i2c: I) -> Self {
        GreenPAK { device: i2c }
    }

    pub fn write_byte(&mut self, offset: u8, byte: u8) -> Result<(), <I as Write>::Error> {
        self.device.write(ADDR, &[offset, byte])
    }

    pub fn write_program(&mut self, data: &[u8; 256]) -> Result<(), <I as Write>::Error> {
        for (idx, byte) in data.iter().enumerate() {
            self.device.write(ADDR, &[idx as u8, *byte])?;
        }

        Ok(())
    }

    pub fn write_program_nvm(&mut self, data: &[u8;256]) -> Result<(), <I as Write>::Error> {
        for (idx, chunk) in data.chunks_exact(16).enumerate() {
            self.device.write(ADDR, &[(idx * 16) as u8, chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7], chunk[8], chunk[9], chunk[10], chunk[11], chunk[12], chunk[13], chunk[14], chunk[15]])?;
        }

        Ok(())
    }

    pub fn read_byte(&mut self, offset: u8) -> Result<u8, <I as WriteRead>::Error> {
        let mut buf = [0u8; 1];
        self.device.write_read(ADDR, &[offset], &mut buf)?;
        Ok(buf[0])
    }

    pub fn read_program(&mut self) -> Result<[u8; 256], <I as WriteRead>::Error> {
        let mut data = [0u8; 256];
        let mut buf = [0u8; 1];

        for idx in 0..256usize {
            self.device.write_read(ADDR, &[idx as u8], &mut buf)?;
            data[idx] = buf[0];
        }

        Ok(data)
    }

    pub fn virtual_input(&mut self, byte: u8, mask: u8) -> Result<(), <I as Write>::Error> {
        // Any bit in the mask that is set to “1” in the I2C Byte Write Mask Register will mask 
        // the effect of changing that particular bit in the target register, during the next Byte 
        // Write Command.
        self.device.write(ADDR, &[0xC9, mask])?;
        self.device.write(ADDR, &[0x7A, byte])?;

        Ok(())
    }
    
    pub fn free(self) -> I {
        self.device
    }
}
