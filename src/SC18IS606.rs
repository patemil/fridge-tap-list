use embedded_hal::blocking::i2c::{Write, Read, WriteRead};

const SC18IS606_I2CADDR: u8 = 0b0101000;
const FUNCID_SS0: u8 = 0x01;
const FUNCID_SS1: u8 = 0x02;
const FUNCID_VERSION: u8 = 0xFE;

pub struct SC18IS606<I> {
    device: I,
}

impl<I: Write + Read + WriteRead> SC18IS606<I> {

    /// Create device driver instance.
    pub fn new(i2c: I) -> Self {
        SC18IS606 { device: i2c }
    }

    pub fn init(&mut self) -> Result<(), <I as Write>::Error> {
        self.device.write(SC18IS606_I2CADDR, &[0xF0, 0x01]) ?;
        Ok(())
    }

    pub fn read_version(&mut self) -> Result <[u8; 16], <I as WriteRead>::Error> {
        let mut data = [0u8; 16];
        self.device.write_read(SC18IS606_I2CADDR, &[FUNCID_VERSION], &mut data)?;
        Ok(data)
    }

    // read registers in LMH6401
    pub fn read_rega1(&mut self) -> Result<(), <I as Write>::Error> {
        self.device.write(SC18IS606_I2CADDR, &mut[0x1,0x8,0x0,0x0,0x0,0x0,0x0,0x0])?; // read register
        //self.device.read(SC18IS606_I2CADDR, &mut[0x0,0x0,0x0,0x0,0x0,0x0])?; // read register
        Ok(())
    }
    pub fn read_rega2(&mut self) -> Result<(), <I as Read>::Error> {
        self.device.read(SC18IS606_I2CADDR, &mut[0x0,0x0,0x0,0x0,0x0,0x0])?; // read register
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

    pub fn read_u16(&mut self, reg: u8) -> Result<u16, <I as WriteRead>::Error> {
        let mut buf = [0u8; 2];
        self.device.write_read(SC18IS606_I2CADDR, &[reg], &mut buf[..])?;
        Ok(u16::from_be_bytes(buf))
    }
}
