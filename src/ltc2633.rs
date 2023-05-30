use embedded_hal::blocking::i2c::Write;

const LTC2633_I2CADDR: u8 = 0x10;

//const CMD_WRITE_TO_INPUT_REGISTER: u8 = 0x00;
//const CMD_UPDATE_DACREGISTER: u8 = 0x10;
const CMD_WRITE_TO_AND_UPDATE_DACA: u8 = 0x30;
const CMD_WRITE_TO_AND_UPDATE_DACB: u8 = 0x31;
const CMD_SELECT_INTERNAL_VREF: u8 = 0x6F;

//const ADR_DACA: u8 = 0x00;
//const ADR_DACB: u8 = 0x01;
//const ADR_ALL:  u8 = 0x0F;

pub struct LTC2633<I> {
    device: I,
}

impl<I: Write> LTC2633<I> {
    /// Create device driver instance.
    pub fn new(i2c: I) -> Self {
        LTC2633 { device: i2c }
    }

    pub fn select_internal_vref(&mut self) -> Result<(), I::Error> {
        self.device
            .write(LTC2633_I2CADDR, &[CMD_SELECT_INTERNAL_VREF, 0x0, 0x0])?;
        Ok(())
    }

    pub fn write_to_and_update_a(&mut self, val: u16) -> Result<u16, I::Error> {
        self.device.write(
            LTC2633_I2CADDR,
            &[
                CMD_WRITE_TO_AND_UPDATE_DACA,
                (val >> 4) as u8,
                (val << 4) as u8,
            ],
        )?;
        Ok(val)
    }

    pub fn write_to_and_update_b(&mut self, val: u16) -> Result<u16, I::Error> {
        self.device.write(
            LTC2633_I2CADDR,
            &[
                CMD_WRITE_TO_AND_UPDATE_DACB,
                (val >> 4) as u8,
                (val << 4) as u8,
            ],
        )?;
        Ok(val)
    }
}
