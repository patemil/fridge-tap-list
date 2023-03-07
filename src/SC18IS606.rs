use embedded_hal::blocking::i2c::{Write, WriteRead};

const SC18IS606_I2CADDR: u8 = 0b0101000;
const FUNCID_SS0: u8 = 0x01;
const FUNCID_SS1: u8 = 0x02;

pub struct SC18IS606<I> {
    device: I,
}

impl<I: Write + WriteRead> SC18IS606<I> {

    /// Create device driver instance.
    pub fn new(i2c: I) -> Self {
        SC18IS606 { device: i2c }
    }

    pub fn init(&mut self) -> Result<(), <I as Write>::Error> {
        self.device.write(SC18IS606_I2CADDR, &[0xF0, 0x02]) ?;
        Ok(())
    }

    pub fn setgaina(&mut self, gain: u8) -> Result<(), <I as Write>::Error> {
        self.device.write(SC18IS606_I2CADDR, &[FUNCID_SS0, 0x2, gain]) ?;
        Ok(())
    }

    pub fn setgainb(&mut self, gain: u8) -> Result<(), <I as Write>::Error> {
        self.device.write(SC18IS606_I2CADDR, &[FUNCID_SS1, 0x2, gain]) ?;
        Ok(())
    }

    fn read_u16(&mut self, reg: u8) -> Result<u16, <I as WriteRead>::Error> {
        let mut buf = [0u8; 2];
        self.device.write_read(SC18IS606_I2CADDR, &[reg], &mut buf[..])?;
        Ok(u16::from_be_bytes(buf))
    }
}
