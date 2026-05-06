#![no_std]
#![no_main]
//! Async blink example for STM32 boards supported by Embassy.

use defmt::info;
use defmt_rtt as _;
use embassy_stm32::Config;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};
use panic_probe as _;

/// Embassy hello world for the Nucleo-F401RE default target.
#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let _ = spawner;
    let peripherals = embassy_stm32::init(Config::default());
    let mut led = Output::new(peripherals.PA5, Level::Low, Speed::Low);

    info!("hello from Embassy on STM32F401RE");

    loop {
        led.set_high();
        info!("LED on");
        Timer::after(Duration::from_millis(500)).await;

        led.set_low();
        info!("LED off");
        Timer::after(Duration::from_millis(500)).await;
    }
}
