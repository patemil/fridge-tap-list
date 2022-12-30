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

    pub fn write_program(&mut self, data: &[u8; 256]) -> Result<(), <I as Write>::Error> {
        for (idx, byte) in data.iter().enumerate() {
            self.device.write(ADDR, &[idx as u8, *byte])?;
        }

        Ok(())
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

    pub fn free(self) -> I {
        self.device
    }
}
