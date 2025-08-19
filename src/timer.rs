use x86_64::instructions::port::Port;

const PIT_FREQUENCY: u32 = 1_193_182;
const DEFAULT_HZ: u32 = 100;

pub fn init(hz: u32) {
    let divisor = (PIT_FREQUENCY / hz.max(1)) as u16;
    unsafe {
        let mut cmd = Port::<u8>::new(0x43);
        let mut data = Port::<u8>::new(0x40);

        cmd.write(0x3u8);
        data.write((divisor & 0xFF) as u8);
        data.write((divisor >> 8) as u8);
    }
}

pub fn init_default() {
    init(DEFAULT_HZ);
}
