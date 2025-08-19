use core::sync::atomic::{AtomicU64, Ordering};

static TICK_COUNT: AtomicU64 = AtomicU64::new(0);

pub fn tick() {
    TICK_COUNT.fetch_add(1, Ordering::SeqCst);
}

pub fn sleep_current(ticks: u64) {
    let _ = ticks;
    yield_now();
}

pub fn exit_current(_code: i32) -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn yield_now() {
    unimplemented!();
}
