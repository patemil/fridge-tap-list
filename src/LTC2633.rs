use embedded_hal::blocking::i2c::Write;

const LTC2633_I2CADDR: u8 = 0x20;

const CMD_WriteToInputRegister: u8 = 0x00;
const CMD_UpdateDACRegister: u8 = 0x10;
const CMD_WriteToAndUpdate: u8 = 0x30;
const CMD_SelectInternalVREF: u8 = 0x60;

const ADR_DACA: u8 = 0x00;
const ADR_DACB: u8 = 0x01;
const ADR_ALL:  u8 = 0x0F;

pub struct LTC2633<I> {
    device: I,
}

impl<I: Write> LTC2633<I> {
    /// Create device driver instance.
    pub fn new(i2c: I) -> Self {
        LTC2633 { device: i2c }
    }

    fn SelectInternalVREF (&mut self) -> Result<(), I::Error> {
        self.device.write(LTC2633_I2CADDR, &[CMD_SelectInternalVREF ^ ADR_DACA, 0x0, 0x0]) ?;
        self.device.write(LTC2633_I2CADDR, &[CMD_SelectInternalVREF ^ ADR_DACB, 0x0, 0x0]) ?;
        Ok(())
    }

    fn write_u16(&mut self, reg: u16) -> Result<u16, I::Error> {
        let mut buf = [0u8; 2];
        self.device.write(LTC2633_I2CADDR, &mut buf[..])?;
        Ok(u16::from_be_bytes(buf))
    }
}
