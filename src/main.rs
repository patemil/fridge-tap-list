/*
//! This shows how to configure UART
//! You can short the TX and RX pin and see it reads what was written.
//! Additionally you can connect a logic analzyer to TX and see how the changes
//! of the configuration change the output signal.

#![no_std]
#![no_main]

extern crate alloc;

#[global_allocator]
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

use esp32c3_hal::{
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
    uart::{
        config::{Config, DataBits, Parity, StopBits},
        TxRxPins,
    },
    Rtc,
    Uart,
    IO,
};
use esp_backtrace as _;
use esp_println::println;
use nb::block;

use alloc::string::String;
use core::char;

#[riscv_rt::entry]
fn main() -> ! {
    init_heap(); 
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut timer0 = timer_group0.timer0;
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    // Disable watchdog timers
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let config = Config {
        baudrate: 115200,
        data_bits: DataBits::DataBits8,
        parity: Parity::ParityNone,
        stop_bits: StopBits::STOP1,
    };

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let pins = TxRxPins::new_tx_rx(
        io.pins.gpio21.into_push_pull_output(),
        io.pins.gpio20.into_floating_input(),
    );

    let mut serial1 = Uart::new_with_config(peripherals.UART1, Some(config), Some(pins), &clocks);

    timer0.start(250u64.millis());

    println!("Start");
    loop {
        
        let mut line = String::with_capacity(40);

        loop {
            let c : char = char::from_u32(block!(serial1.read()).unwrap().into()).unwrap();

            match c {
                '\r' => continue,
                '\n' => break,
                _ => line.push(c.to_ascii_lowercase()),
            }
            serial1.write(char::from(c).into());
        }
        println!("line read");
        println!("text:{}",line);

        block!(timer0.wait()).unwrap();
    }
}
*/

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
    0x00, 0x01, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xA5
];

mod greenpak;
mod lm75;
mod st7032;


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
use lm75::LM75;
use shared_bus::BusManagerSimple;
use st7032::ST7032;

macro_rules! log_error {
    ($value:expr, $message:expr) => {
        match $value {
            Ok(_) => {},
            Err(err) => {
                esp_println::println!(concat!($message, ": {:?}"), err)
            }
        }
    };
    ($value:expr, $message:expr$(, $args:expr)*) => {
        match $value {
            Ok(_) => {},
            Err(err) => {
                esp_println::println!(concat!($message, ": {:?}"), $($args),*, err)
            }
        }
    };
}

fn select_lcd<I: embedded_hal::blocking::i2c::Write + embedded_hal::blocking::i2c::WriteRead> (greenpak: &mut GreenPAK<I>, lcd: u8) -> Result<(), <I as embedded_hal::blocking::i2c::Write>::Error> {
    assert!(lcd < 4);

    greenpak.write_byte(0x7A, 0b01000000 | (lcd << 4))
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
        io.pins.gpio1.into_open_drain_output(),
        io.pins.gpio2.into_open_drain_output(),
        400u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    );

    let i2c = BusManagerSimple::new(i2c);

    let mut greenpak = GreenPAK::new(i2c.acquire_i2c());
    log_error!(greenpak.write_program(&GREENPAK_DATA), "Failed to write program to GreenPAK");

    let mut delay = Delay::new(&clocks);
    let mut sensor = LM75::new(i2c.acquire_i2c());
    let mut lcd = ST7032::new(i2c.acquire_i2c());

    for i in 0..0 {
        log_error!(select_lcd(&mut greenpak, i), "Failed to select LCD {}", i);
        log_error!(lcd.init(), "Failed to initialize LCD {}", i);
    }

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


    enum Command {
        SetOffset(f32),
        SetSamplingRate(u32),
    }

    impl FromStr for Command {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.split_whitespace();

            let command = parts.next().ok_or("No command")?;

            match command {
                "Set_offset" => {
                    let value = parts.next().ok_or("No value")?;
                    let value = value.parse::<f32>().map_err(|_| "Invalid value")?;
                    Ok(Command::SetOffset(value))
                }
                "Set_samplingrate" => {
                    let value = parts.next().ok_or("No value")?;
                    let value = value.parse::<u32>().map_err(|_| "Invalid value")?;
                    Ok(Command::SetSamplingRate(value))
                }
                _ => Err("Invalid command".to_string()),
            }
        }
    }

// ...

/*
for cmd in input.lines() {
    let cmd = input.parse::<Command>()?;
    
    match command {
        Command::SetOffset(offset) => {
            println!("Setting offset to {}", offset);
        }
        Command::SetSamplingRate(rate) => {
            println!("Setting sampling rate to {}", rate);
        }
    }
}
*/

    log_error!(select_lcd(&mut greenpak, 0), "Failed to select LCD 0");
    log_error!(lcd.set_line(0, "White House Ale"), "Failed to write to LCD 0");

    log_error!(select_lcd(&mut greenpak, 1), "Failed to select LCD 1");
    log_error!(lcd.set_line(0, "Milky Way"), "Failed to write to LCD 1");

    log_error!(select_lcd(&mut greenpak, 2), "Failed to select LCD 2");
    log_error!(lcd.set_line(0, "Jul 2022"), "Failed to write to LCD 2");

    log_error!(select_lcd(&mut greenpak, 3), "Failed to select LCD 3");
    log_error!(lcd.set_line(0, "Reservoir Hops"), "Failed to write to LCD 3");

    loop {

        let mut line = String::with_capacity(40);

        loop {
            let c : char = char::from_u32(block!(serial1.read()).unwrap().into()).unwrap();

            match c {
                '\r' => continue,
                '\n' => break,
                _ => line.push(c.to_ascii_lowercase()),
            }
        }
        println!("line read");
        println!("text:{}",line);

        /*let temp = sensor.measure().unwrap();

        log_error!(select_lcd(&mut greenpak, 0), "Failed to select LCD 0");
        log_error!(lcd.set_cursor(0, 1), "Failed to set cursor on LCD 0");
        log_error!(write!(lcd, "Temp: {: >5.1}°C", temp), "Failed to write to LCD 0");
*/
        delay.delay_ms(1000u32);
    }
}
