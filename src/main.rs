#![no_std]
#![no_main]

mod lm75;

use esp_println::println;
use fugit::RateExtU32;

use esp32_hal::{
    clock::ClockControl, i2c::I2C, pac::Peripherals, prelude::*, timer::TimerGroup, Delay, Rtc, IO,
};
use esp_backtrace as _;

use lm75::LM75;

#[xtensa_lx_rt::entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio21.into_open_drain_output(),
        io.pins.gpio22.into_open_drain_output(),
        400u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    );

    let mut delay = Delay::new(&clocks);
    let mut sensor = LM75::new(i2c);

    loop {
        let temp = sensor.measure().unwrap();

        println!("Temperature: {}°C", temp);

        delay.delay_ms(1000u32);
    }
}
