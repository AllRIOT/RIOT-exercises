#![no_main]
#![no_std]

use hal::{gpio, timer::Timer};
use nrf52840_hal as hal;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::pac::Peripherals::take().unwrap();

    let (mut led, mut timer) = init_device(p);

    // [TASK 5: Copy in your portable LED-and-timer application]
    todo!()
}

fn init_device(
    p: hal::pac::Peripherals,
) -> (
    nrf52840_hal::gpio::Pin<gpio::Output<gpio::PushPull>>,
    Timer<hal::pac::TIMER0>,
) {
    let p0 = gpio::p0::Parts::new(p.P0);
    let led0 = p0.p0_20.into_push_pull_output(gpio::Level::Low).degrade();
    let timer = Timer::new(p.TIMER0);

    (led0, timer)
}
