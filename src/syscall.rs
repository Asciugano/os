use core::u64;

use crate::{print, println};

#[repr(u64)]
pub enum Sysno {
    Write = 1,
    Exit = 60,
    Sleep = 162,
}

pub fn do_syscall(num: u64, a1: u64, a2: u64, a3: u64) -> u64 {
    match num {
        x if x == Sysno::Write as u64 => {
            let _fd = a1;
            let ptr = a2 as *const u8;
            let len = a3 as usize;
            unsafe {
                if let Ok(s) = core::str::from_utf8(core::slice::from_raw_parts(ptr, len)) {
                    print!("{}", s);
                } else {
                    println!("[sys_write] non-utf8, len={}", len);
                }
            }
            0
        }
        x if x == Sysno::Exit as u64 => {
            let code = a1 as i32;
            crate::scheduler::exit_current(code);
            0
        }
        x if x == Sysno::Sleep as u64 => {
            let ticks = a1 as u64;
            crate::scheduler::sleep_current(ticks);
            0
        }
        _ => {
            println!("[sys] unknwn: {} ({:#x}, {:#x}, {:#x})", num, a1, a2, a3);
            u64::MAX
        }
    }
}
