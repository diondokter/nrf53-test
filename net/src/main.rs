#![no_std]
#![no_main]

use core::sync::atomic::Ordering;
use cortex_m_rt::entry;
use nrf5340_net_hal::{gpio::Level, prelude::OutputPin};
// global logger
use nrf5340_net_pac as pac;

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();

    nrf53_lib::approtect::disable_approtect(&mut p.UICR_NS, &mut p.CTRLAP_NS, &mut p.NVMC_NS);

    let p0 = nrf5340_net_hal::gpio::p0::Parts::new(p.P0_NS);

    let mut led_pin = p0.p0_29.into_push_pull_output(Level::Low);
    let mut delay;

    loop {
        delay = unsafe {
            shared_ram::NET_CORE_BLINK_DELAY
                .assume_init_ref()
                .load(Ordering::SeqCst)
        };

        led_pin.set_high().unwrap();
        cortex_m::asm::delay(delay);
        led_pin.set_low().unwrap();
        cortex_m::asm::delay(delay);

        unsafe { shared_ram::NET_CORE_BLINK_COUNTER.assume_init_ref() }
            .fetch_add(1, Ordering::SeqCst);
    }
}

#[cortex_m_rt::exception]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

/// Called when our code panics.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    cortex_m::asm::udf();
}
