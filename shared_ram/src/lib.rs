#![no_std]

use core::{sync::atomic::{AtomicU32, Ordering}, mem::MaybeUninit};

#[link_section = ".shared_ram"]
pub static NET_CORE_BLINK_COUNTER: MaybeUninit<AtomicU32> = MaybeUninit::uninit();

pub unsafe fn initialize_net_core_blink_counter() {
    NET_CORE_BLINK_COUNTER.assume_init_ref().store(0, Ordering::SeqCst);
}

#[link_section = ".shared_ram"]
pub static NET_CORE_BLINK_DELAY: MaybeUninit<AtomicU32> = MaybeUninit::uninit();

pub unsafe fn initialize_net_core_blink_delay() {
    NET_CORE_BLINK_DELAY.assume_init_ref().store(5_000_000, Ordering::SeqCst);
}
