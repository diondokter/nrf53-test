#![no_std]
#![no_main]

use core::{ops::Range, sync::atomic::Ordering};
use cortex_m_rt::entry;
use nrf5340_app_pac as pac;

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();

    p.CACHE_S.enable.write(|w| w.enable().enabled());

    p.CLOCK_S.hfclkctrl.write(|w| w.hclk().div1());

    nrf53_lib::approtect::disable_approtect(&mut p.UICR_S, &mut p.CTRLAP_S, &mut p.NVMC_S);

    let mut spu = nrf53_lib::spu::Spu::new(p.SPU_S);
    spu.set_peripheral_permissions(nrf5340_app_pac::P0_NS::PTR, false, false, false);
    spu.set_gpio_pin_permissions_all(0, false);
    spu.set_ram_region_permissions(shared_ram_range(), true, true, true, false, false);

    p.P0_S.pin_cnf[29].write(|w| w.mcusel().network_mcu());

    // Make sure our shared ram is initialized before we boot the network core
    unsafe {
        shared_ram::initialize_net_core_blink_counter();
        shared_ram::initialize_net_core_blink_delay();
    }

    // Boot network core
    p.RESET_S.network.forceoff.write(|w| w.forceoff().release());

    p.P0_NS.pin_cnf[28].write(|w| w.dir().output());

    let mut last_counter_value = 0;

    loop {
        loop {
            let current_counter_value =
                unsafe { shared_ram::NET_CORE_BLINK_COUNTER.assume_init_ref() }
                    .load(Ordering::SeqCst);

            if current_counter_value > last_counter_value {
                last_counter_value = current_counter_value;
                break;
            }
        }

        p.P0_NS.outclr.write(|w| w.pin28().clear());
        cortex_m::asm::delay(1_000_000);
        p.P0_NS.outset.write(|w| w.pin28().set());

        unsafe {
            shared_ram::NET_CORE_BLINK_DELAY
                .assume_init_ref()
                .store(5_000_000 - (last_counter_value * 500_000) % 5_000_000, Ordering::SeqCst);
        }
    }
}

extern "C" {
    static mut _shared_ram_start: u32;
    static mut _shared_ram_end: u32;
}

pub fn shared_ram_range() -> Range<u32> {
    unsafe {
        let start = &_shared_ram_start as *const u32 as u32;
        let end = &_shared_ram_end as *const u32 as u32;
        start..end
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
