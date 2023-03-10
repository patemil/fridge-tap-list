#![no_std]
#![no_main]

// Manually copied from https://github.com/unneback/LCD/blob/10709f03c5c57bdf737b54c35fc8c82c67a4ae96/GreenPAK/LCDx4.hex
const GREENPAK_DATA: [u8; 256] = [
    0xE1, 0x0D, 0x00, 0xA1, 0x0D, 0x00, 0xEF, 0xE3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x03, 0x1D, 0x10, 0x4A, 0x3E, 0xE8, 0x77, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x0F, 0x00, 0x3E, 0xE0, 0x03,
    0x3E, 0x00, 0xC4, 0xE9, 0x2F, 0x03, 0xF0, 0x0F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE9,
    0x0F, 0x00, 0xE9, 0x0F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x80, 0x04, 0x00, 0x84, 0x30, 0x00, 0x30, 0x00, 0x00, 0x30, 0x30, 0x30, 0x00, 0x03, 0x03,
    0x04, 0x03, 0x04, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x04, 0x00, 0x08, 0x00, 0x34, 0x22, 0x31, 0x0C, 0xE8, 0x00, 0xE8, 0x00, 0x1B, 0x14, 0x00,
    0x8A, 0x00, 0x00, 0x00, 0xF4, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x81, 0x00, 0x00, 0x0B, 0x00, 0x64, 0x00, 0x46, 0x40, 0x20, 0x0D, 0x25, 0x00, 0x20, 0x2D, 0x07,
    0x00, 0x00, 0x02, 0x01, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x02, 0x01, 0x00, 0x00, 0x02,
    0x00, 0x01, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xA5
];

const GREENPAK_DATA_NVM: [[u8; 16]; 16] = [
    [0xE1, 0x0D, 0x00, 0xA1, 0x0D, 0x00, 0xEF, 0xE3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x03, 0x1D, 0x10, 0x4A, 0x3E, 0xE8, 0x77, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x0F, 0x00, 0x3E, 0xE0, 0x03],
    [0x3E, 0x00, 0xC4, 0xE9, 0x2F, 0x03, 0xF0, 0x0F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE9],
    [0x0F, 0x00, 0xE9, 0x0F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x80, 0x04, 0x00, 0x84, 0x30, 0x00, 0x30, 0x00, 0x00, 0x30, 0x30, 0x30, 0x00, 0x03, 0x03],
    [0x04, 0x03, 0x04, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x04, 0x00, 0x08, 0x00, 0x34, 0x22, 0x31, 0x0C, 0xE8, 0x00, 0xE8, 0x00, 0x1B, 0x14, 0x00],
    [0x8A, 0x00, 0x00, 0x00, 0xF4, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x81, 0x00, 0x00, 0x0B, 0x00, 0x64, 0x00, 0x46, 0x40, 0x20, 0x0D, 0x25, 0x00, 0x20, 0x2D, 0x07],
    [0x00, 0x00, 0x02, 0x01, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x02, 0x01, 0x00, 0x00, 0x02],
    [0x00, 0x01, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xA5]
];

mod ltc2633;
mod greenpak;
mod sc18is606;

use crate::ltc2633::LTC2633;
extern crate alloc;


#[global_allocator]  // necessary for correct work of alloc on ESP chips
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;

    extern "C" {
        static mut _heap_start: u32;
    }

    unsafe {
        let heap_start = &_heap_start as *const _ as usize;
        ALLOCATOR.init(heap_start as *mut u8, HEAP_SIZE);
    }
}

use alloc::{str::FromStr, string::ToString};
use core::fmt::Write;
use alloc::string::String;
use esp_println::println;
use nb::block;
use fugit::RateExtU32;
use esp32c3_hal::{
    clock::ClockControl, i2c::I2C, prelude::*, timer::TimerGroup, Delay, Rtc, gpio::IO
};
use esp32c3_hal::    uart::{
        config::{Config, DataBits, Parity, StopBits},
        TxRxPins,
    };
use esp32c3_hal::Uart;

use esp_backtrace as _;

use greenpak::GreenPAK;
use shared_bus::BusManagerSimple;

macro_rules! log_error {
    ($sport:ident, $value:expr, $message:expr) => {
        match $value {
            Ok(_) => {},
            Err(err) => {
                //esp_println::println!(concat!($message, ": {:?}"), err)
                writeln!($sport, concat!($message, ": {:?}"), err);
                //writeln!($sport, concat!($message, ": {:?}"), err)
            }
        }
    };
    ($port:expr, $value:expr, $message:expr$(, $args:expr)*) => {
        match $value {
            Ok(_) => {},
            Err(err) => {
                writeln!($sport, concat!($message, ": {:?}"), $($args),*, err);
                //esp_println::println!(concat!($message, ": {:?}"), $($args),*, err)
            }
        }
    };
}


use core::char;

#[riscv_rt::entry]
fn main() -> ! {
    
    init_heap();
    println!("Hello world");
    let peripherals = esp32c3_hal::peripherals::Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    //let mut serial0 = uart::new(peripherals.UART0).unwrap();
    
    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio0,
        io.pins.gpio1,
        100u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    );

    let i2c = BusManagerSimple::new(i2c);
    let mut delay = Delay::new(&clocks);

    let mut greenpak = GreenPAK::new(i2c.acquire_i2c());
    let mut dac = LTC2633::new(i2c.acquire_i2c());
    let mut spi = sc18is606::SC18IS606::new(i2c.acquire_i2c());

    let config = Config {
        baudrate: 115200,
        data_bits: DataBits::DataBits8,
        parity: Parity::ParityNone,
        stop_bits: StopBits::STOP1,
    };

    //let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let pins = TxRxPins::new_tx_rx(
        io.pins.gpio21.into_push_pull_output(),
        io.pins.gpio20.into_floating_input(),
    );

    let mut serial1 = Uart::new_with_config(peripherals.UART1, Some(config), Some(pins), &clocks);


    /*
    for i in 0..16 {
        log_error!(greenpak.erase_nvm_page(i as u8), "Failed to erase GreenPAK NVM page {}", i);
        delay.delay_ms(20u32);
    }
    log_error!(greenpak.write_program(&GREENPAK_DATA), "Failed to write program to GreenPAK");
    */

    // Enable slave select generation
    log_error!(serial1, greenpak.virtual_input(0b1000_0000, 0b0111_1111), "Failed to set virtual input");

    // Set internal VREF
    log_error!(serial1, dac.select_internal_vref(), "Failed to select internal VREF");
    log_error!(serial1, dac.write_to_and_update_a(0), "Failed to write DAC");
    log_error!(serial1, dac.write_to_and_update_b(0), "Failed to write DAC");

    // SPI configuration
    log_error!(serial1, spi.init(), "Failed to initialize SPI port to PGA");
    let mut data = [0u8; 16];
    data = spi.read_version().unwrap();
    writeln!(serial1,String::from_utf8(&data) );

    enum Command {
        Offseta(u16),
        Offsetb(u16),
        Fcount(u16),
        Chsel(u16),
        Run(u16),
        Burst(u16),
        Burstlength(u16),
        Gaina(i16),
        Gainb(i16),
        Reada(u16),
    }

    impl FromStr for Command {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.split_whitespace();

            let command = parts.next().ok_or("No command")?;

            match command {
                "offseta" => {
                    let value = parts.next().ok_or("No value")?;
                    let value = value.parse::<u16>().map_err(|_| "Invalid value")?;
                    Ok(Command::Offseta(value))
                }
                "offsetb" => {
                    let value = parts.next().ok_or("No value")?;
                    let value = value.parse::<u16>().map_err(|_| "Invalid value")?;
                    Ok(Command::Offsetb(value))
                }
                "fcount" => {
                    let value = parts.next().ok_or("No value")?;
                    let value = value.parse::<u16>().map_err(|_| "Invalid value")?;
                    Ok(Command::Fcount(value))
                }
                "ch_sel" => {
                    let value = parts.next().ok_or("No value")?;
                    let value = value.parse::<u16>().map_err(|_| "Invalid value")?;
                    Ok(Command::Chsel(value))
                }
                "run" => {
                    let value = parts.next().ok_or("No value")?;
                    let value = value.parse::<u16>().map_err(|_| "Invalid value")?;
                    Ok(Command::Run(value))
                }
                "burst" => {
                    let value = parts.next().ok_or("No value")?;
                    let value = value.parse::<u16>().map_err(|_| "Invalid value")?;
                    Ok(Command::Burst(value))
                }
                "burstlength" => {
                    let value = parts.next().ok_or("No value")?;
                    let value = value.parse::<u16>().map_err(|_| "Invalid value")?;
                    Ok(Command::Burstlength(value))
                }
                "gaina" => {
                    let value = parts.next().ok_or("No value")?;
                    let value:i16 = value.parse::<i16>().map_err(|_| "Invalid value")?;
                    Ok(Command::Gaina(value))
                }
                "gainb" => {
                    let value = parts.next().ok_or("No value")?;
                    let value:i16 = value.parse::<i16>().map_err(|_| "Invalid value")?;
                    Ok(Command::Gainb(value))
                }
                "reada" => {
                    let value = parts.next().ok_or("No register")?;
                    let value:u16 = value.parse::<u16>().map_err(|_| "Invalid value")?;
                    Ok(Command::Reada(value))
                }
                _ => Err("Invalid command".to_string()),
            }
        }
    }

    const BACKSPACE: char = 8u8 as char;

    writeln!(serial1,"");
    writeln!(serial1,"");
    writeln!(serial1,"FFI 2023-02-23\n\r");
    
    loop {

        let mut line = String::with_capacity(40);

        loop{
            loop {

                let c : char = char::from_u32(block!(serial1.read()).unwrap().into()).unwrap();

                match c {
                    '\r' => break,
                    '\n' => break,
                    '\u{8}' => {
                        line.pop();
                        write!(serial1,"\u{8} \u{8}");
                    },
                    _ => {
                        line.push(c);
                        write!(serial1,"{}",c);

                    },
                }

                
            }
            //writeln!(serial1,"line read :{} :{}",line.len(), line);

            writeln!(serial1,"");
            if let Ok(cmd) = line.parse::<Command>() {

                match cmd {
                    Command::Offseta(offset) => {
                        writeln!(serial1,"\rSetting offset to {}\r", offset);
                        log_error!(serial1, dac.write_to_and_update_a(offset), "Failed to write to DAC");
                    }
                    Command::Offsetb(offset) => {
                        writeln!(serial1,"\rSetting offset to {}\r", offset);
                        log_error!(serial1, dac.write_to_and_update_b(offset), "Failed to write to DAC");
                    }
                    Command::Fcount(value) => {
                        writeln!(serial1, "\rSetting sampling rate to {}\r", value);
                        log_error!(serial1, greenpak.write_cnt2(value as u8), "Failed to write CNT2");
                    }
                    Command::Chsel(value) => {
                        writeln!(serial1, "\rActive channel {}\r", value);
                    }
                    Command::Run(value) => {
                        writeln!(serial1, "\rEnable sampling {}\r", value);
                        log_error!(serial1, greenpak.virtual_input(0b1000_0000, 0b0111_1111), "Failed to set virtual input");
                    }
                    Command::Burst(value) => {
                        writeln!(serial1, "\rEnable burst sampling {}\r", value);
                    }
                    Command::Burstlength(value) => {
                        writeln!(serial1, "\rBurst length {}\r", value);
                        log_error!(serial1, greenpak.write_cnt0(value), "Failed to write CNT0");
                    }
                    Command::Gaina(value) => {
                        writeln!(serial1, "\rGain channel a {}\r", value);
                        log_error!(serial1, spi.setgaina(value as u8), "Failed to write gain A");
                    }
                    Command::Gainb(value) => {
                        writeln!(serial1, "\rGain channel b {}\r", value);
                        log_error!(serial1, spi.setgainb(value as u8), "Failed to write gain B");
                    }
                    Command::Reada(value) => {
                        writeln!(serial1, "\rRead from A register {}\r", value);
                        log_error!(serial1, spi.read_u16(value as u8), "Failed to write gain B");
                    }
                }
            } else { 
                writeln!(serial1, "\rCommand not found or missing parameter\n\r");
            }

            //writeln!(serial1,"line read :{} :{}",line.len(), line);

            line.clear();
            writeln!(serial1,"");
        }    

    }
}
